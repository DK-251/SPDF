//! Document signing: SHA-256 content hashing, state transitions, and verification.

use std::io::{Cursor, Read, Write};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use crate::container::{read_container, ExtractedLayers};
use crate::dom::{Document, Element};
use crate::error::{SpdfError, SpdfResult};
use crate::manifest::sha256_hex;
use crate::types::DocumentState;

/// A signature record stored in `signatures/signature_NNN.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureRecord {
    pub signature_id: String,
    pub signer_name: String,
    pub signer_email: String,
    pub signed_at: String,
    pub content_hash: String,
    pub algorithm: String,
}

/// Result of verifying a single signature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureVerification {
    pub signature_id: String,
    pub signer_name: String,
    pub signer_email: String,
    pub valid: bool,
    pub expected_hash: String,
    pub actual_hash: String,
}

/// Full verification report for a document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationReport {
    pub valid: bool,
    pub tamper_detected: bool,
    pub signature_count: usize,
    pub signatures: Vec<SignatureVerification>,
}

fn compute_content_hash(layers: &ExtractedLayers) -> String {
    let mut combined = Vec::new();
    combined.extend_from_slice(&layers.semantic);
    combined.extend_from_slice(&layers.layout);
    combined.extend_from_slice(&layers.styles);
    combined.extend_from_slice(&layers.metadata);
    sha256_hex(&combined)
}

pub fn rebuild_container_with_signatures(
    extracted: &ExtractedLayers,
    new_semantic: &[u8],
    new_audit: &[u8],
    signatures: &[(String, Vec<u8>)],
) -> SpdfResult<Vec<u8>> {
    let mut manifest = extracted.manifest.clone();
    manifest.layers.semantic = sha256_hex(new_semantic);
    manifest.layers.audit = sha256_hex(new_audit);
    manifest.finalize();

    let mut buf = Vec::new();
    let mut zip = ZipWriter::new(Cursor::new(&mut buf));
    let deflate = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    let stored = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

    zip.start_file("manifest.json", deflate)?;
    zip.write_all(&serde_json::to_vec_pretty(&manifest)?)?;

    zip.start_file("layers/semantic.json", deflate)?;
    zip.write_all(new_semantic)?;

    zip.start_file("layers/layout.json", deflate)?;
    zip.write_all(&extracted.layout)?;

    zip.start_file("layers/styles.json", deflate)?;
    zip.write_all(&extracted.styles)?;

    zip.start_file("layers/render.pdf", stored)?;
    zip.write_all(&extracted.render)?;

    zip.start_file("layers/metadata.json", deflate)?;
    zip.write_all(&extracted.metadata)?;

    zip.start_file("layers/audit.json", deflate)?;
    zip.write_all(new_audit)?;

    for (name, data) in &extracted.assets {
        zip.start_file(format!("assets/{name}"), deflate)?;
        zip.write_all(data)?;
    }

    for (name, data) in signatures {
        zip.start_file(name.clone(), deflate)?;
        zip.write_all(data)?;
    }

    zip.set_comment("SPDF/1.0");
    zip.finish()?;

    Ok(buf)
}

/// Sign an SPDF document with SHA-256 simple signing.
///
/// The document must be in `Review` state. After signing:
/// - State transitions to `Signed`
/// - All `SignatureBlock` elements get `locked: true` and `signed_at`
/// - A `STATE_CHANGED` audit event is appended
/// - A `signatures/signature_001.json` entry is added to the container
pub fn sign_document_simple(
    spdf_bytes: &[u8],
    signer_name: &str,
    signer_email: &str,
) -> SpdfResult<Vec<u8>> {
    let extracted = read_container(spdf_bytes)?;

    let mut doc: Document = serde_json::from_slice(&extracted.semantic)?;

    if doc.document_state != DocumentState::Review {
        return Err(SpdfError::WrongState {
            expected: DocumentState::Review,
            actual: doc.document_state,
        });
    }

    let content_hash = compute_content_hash(&extracted);
    let now = Utc::now();
    let now_str = now.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

    let sig_id = format!("sig-{}", uuid::Uuid::new_v4());
    let record = SignatureRecord {
        signature_id: sig_id,
        signer_name: signer_name.to_string(),
        signer_email: signer_email.to_string(),
        signed_at: now_str.clone(),
        content_hash,
        algorithm: "SHA256_SIMPLE".to_string(),
    };

    doc.document_state = DocumentState::Signed;

    for page in &mut doc.pages {
        for element in &mut page.elements {
            if let Element::SignatureBlock(sb) = element {
                sb.locked = true;
                sb.signed_at = Some(now_str.clone());
            }
        }
    }

    let new_semantic = serde_json::to_vec_pretty(&doc)?;

    let mut audit: serde_json::Value = serde_json::from_slice(&extracted.audit)?;
    let entries = audit
        .as_object_mut()
        .and_then(|o| o.get_mut("entries"))
        .and_then(|v| v.as_array_mut());
    if let Some(entries) = entries {
        entries.push(serde_json::json!({
            "event": "STATE_CHANGED",
            "from": "REVIEW",
            "to": "SIGNED",
            "timestamp": now_str,
            "actor": signer_name,
        }));
    }
    let new_audit = serde_json::to_vec_pretty(&audit)?;

    let sig_json = serde_json::to_vec_pretty(&record)?;
    let signatures = vec![("signatures/signature_001.json".to_string(), sig_json)];

    rebuild_container_with_signatures(&extracted, &new_semantic, &new_audit, &signatures)
}

/// Verify all signatures in an SPDF document.
///
/// Recomputes the content hash from layers and compares against each
/// stored signature record.
pub fn verify_document_simple(spdf_bytes: &[u8]) -> SpdfResult<VerificationReport> {
    let extracted = read_container(spdf_bytes)?;
    let actual_hash = compute_content_hash(&extracted);

    let cursor = Cursor::new(spdf_bytes);
    let mut archive = ZipArchive::new(cursor)?;

    let mut sig_records: Vec<SignatureRecord> = Vec::new();
    let sig_names: Vec<String> = (0..archive.len())
        .filter_map(|i| {
            let entry = archive.by_index(i).ok()?;
            let name = entry.name().to_string();
            if name.starts_with("signatures/") && name.ends_with(".json") {
                Some(name)
            } else {
                None
            }
        })
        .collect();

    for name in &sig_names {
        let mut entry = archive.by_name(name)?;
        let mut buf = Vec::new();
        entry.read_to_end(&mut buf)?;
        let record: SignatureRecord = serde_json::from_slice(&buf)?;
        sig_records.push(record);
    }

    let mut tamper_detected = false;
    let verifications: Vec<SignatureVerification> = sig_records
        .into_iter()
        .map(|rec| {
            let valid = rec.content_hash == actual_hash;
            if !valid {
                tamper_detected = true;
            }
            SignatureVerification {
                signature_id: rec.signature_id,
                signer_name: rec.signer_name,
                signer_email: rec.signer_email,
                valid,
                expected_hash: rec.content_hash,
                actual_hash: actual_hash.clone(),
            }
        })
        .collect();

    let all_valid = !verifications.is_empty() && verifications.iter().all(|v| v.valid);

    Ok(VerificationReport {
        valid: all_valid,
        tamper_detected,
        signature_count: verifications.len(),
        signatures: verifications,
    })
}

/// Transition a document to a new state (generic state transition).
///
/// Validates the transition is allowed, updates the semantic layer,
/// and appends a `STATE_CHANGED` audit event.
pub fn transition_document(
    spdf_bytes: &[u8],
    target_state: DocumentState,
) -> SpdfResult<Vec<u8>> {
    let extracted = read_container(spdf_bytes)?;
    let mut doc: Document = serde_json::from_slice(&extracted.semantic)?;

    if !doc.document_state.can_transition_to(&target_state) {
        return Err(SpdfError::InvalidStateTransition {
            from: doc.document_state,
            to: target_state,
        });
    }

    let from_state = doc.document_state;
    doc.document_state = target_state;

    let new_semantic = serde_json::to_vec_pretty(&doc)?;

    let now_str = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
    let mut audit: serde_json::Value = serde_json::from_slice(&extracted.audit)?;
    if let Some(entries) = audit
        .as_object_mut()
        .and_then(|o| o.get_mut("entries"))
        .and_then(|v| v.as_array_mut())
    {
        entries.push(serde_json::json!({
            "event": "STATE_CHANGED",
            "from": format!("{from_state:?}").to_uppercase(),
            "to": format!("{target_state:?}").to_uppercase(),
            "timestamp": now_str,
        }));
    }
    let new_audit = serde_json::to_vec_pretty(&audit)?;

    rebuild_container_with_signatures(&extracted, &new_semantic, &new_audit, &[])
}
