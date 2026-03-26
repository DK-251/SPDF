//! Redaction: replace elements with proof-hashed redaction markers.

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::container::read_container;
use crate::dom::{Document, Element, RedactionElement};
use crate::error::{SpdfError, SpdfResult};
use crate::manifest::sha256_hex;
use crate::signing::rebuild_container_with_signatures;
use crate::types::{ElementId, Timestamps};

/// Entry returned by `list_redactions`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedactionListEntry {
    pub eid: String,
    pub redacted_eid: String,
    pub reason: String,
    pub erasure_proof_hash: String,
}

/// Result of verifying a single redaction's proof hash.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedactionVerification {
    pub redaction_eid: String,
    pub redacted_eid: String,
    pub proof_hash: String,
    pub found: bool,
}

/// Redact an element from an SPDF document.
///
/// Replaces the target element with a `RedactionElement` containing the
/// SHA-256 erasure proof hash of the original element's JSON serialization.
pub fn redact_element(spdf_bytes: &[u8], target_eid: &str, reason: &str) -> SpdfResult<Vec<u8>> {
    let extracted = read_container(spdf_bytes)?;
    let mut doc: Document = serde_json::from_slice(&extracted.semantic)?;

    let mut found = false;
    for page in &mut doc.pages {
        for element in &mut page.elements {
            if element.eid().0 == target_eid {
                // Compute proof hash from original element
                let original_json = serde_json::to_vec(element)?;
                let erasure_proof_hash = sha256_hex(&original_json);

                let redaction_eid = ElementId::new();
                *element = Element::Redaction(RedactionElement {
                    eid: redaction_eid,
                    redacted_eid: ElementId(target_eid.to_string()),
                    reason: reason.to_string(),
                    erasure_proof_hash,
                    timestamps: Timestamps::now(),
                });
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    if !found {
        return Err(SpdfError::ElementNotFound(target_eid.to_string()));
    }

    let new_semantic = serde_json::to_vec_pretty(&doc)?;

    let now_str = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
    let mut audit: serde_json::Value = serde_json::from_slice(&extracted.audit)?;
    if let Some(entries) = audit
        .as_object_mut()
        .and_then(|o| o.get_mut("entries"))
        .and_then(|v| v.as_array_mut())
    {
        entries.push(serde_json::json!({
            "event": "REDACTED",
            "target_eid": target_eid,
            "reason": reason,
            "timestamp": now_str,
        }));
    }
    let new_audit = serde_json::to_vec_pretty(&audit)?;

    rebuild_container_with_signatures(&extracted, &new_semantic, &new_audit, &[])
}

/// List all redactions in an SPDF document.
pub fn list_redactions(spdf_bytes: &[u8]) -> SpdfResult<Vec<RedactionListEntry>> {
    let extracted = read_container(spdf_bytes)?;
    let doc: Document = serde_json::from_slice(&extracted.semantic)?;

    let mut entries = Vec::new();
    for page in &doc.pages {
        for element in &page.elements {
            if let Element::Redaction(r) = element {
                entries.push(RedactionListEntry {
                    eid: r.eid.0.clone(),
                    redacted_eid: r.redacted_eid.0.clone(),
                    reason: r.reason.clone(),
                    erasure_proof_hash: r.erasure_proof_hash.clone(),
                });
            }
        }
    }

    Ok(entries)
}

/// Verify a redaction exists and return its proof hash.
pub fn verify_redaction(
    spdf_bytes: &[u8],
    redaction_eid: &str,
) -> SpdfResult<RedactionVerification> {
    let extracted = read_container(spdf_bytes)?;
    let doc: Document = serde_json::from_slice(&extracted.semantic)?;

    for page in &doc.pages {
        for element in &page.elements {
            if let Element::Redaction(r) = element {
                if r.eid.0 == redaction_eid || r.redacted_eid.0 == redaction_eid {
                    return Ok(RedactionVerification {
                        redaction_eid: r.eid.0.clone(),
                        redacted_eid: r.redacted_eid.0.clone(),
                        proof_hash: r.erasure_proof_hash.clone(),
                        found: true,
                    });
                }
            }
        }
    }

    Ok(RedactionVerification {
        redaction_eid: redaction_eid.to_string(),
        redacted_eid: String::new(),
        proof_hash: String::new(),
        found: false,
    })
}
