//! Semantic diff engine: compare two SPDF documents at element level.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::container::read_container;
use crate::dom::{Document, Element};
use crate::error::SpdfResult;
use crate::types::ElementId;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SemanticImpact {
    None,
    Minor,
    Moderate,
    Major,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChangeType {
    Added,
    Removed,
    Modified,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffChange {
    pub change_type: ChangeType,
    pub eid: String,
    pub element_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_value: Option<serde_json::Value>,
    pub impact: SemanticImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffSummary {
    pub added: usize,
    pub removed: usize,
    pub modified: usize,
    pub total_changes: usize,
    pub highest_impact: SemanticImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffReport {
    pub metadata_changes: Vec<DiffChange>,
    pub element_changes: Vec<DiffChange>,
    pub summary: DiffSummary,
}

fn element_type_name(el: &Element) -> &'static str {
    match el {
        Element::Heading(_) => "Heading",
        Element::Paragraph(_) => "Paragraph",
        Element::Table(_) => "Table",
        Element::Image(_) => "Image",
        Element::VectorImage(_) => "VectorImage",
        Element::CodeBlock(_) => "CodeBlock",
        Element::HorizontalRule(_) => "HorizontalRule",
        Element::PageBreak(_) => "PageBreak",
        Element::Attachment(_) => "Attachment",
        Element::InvoiceHeader(_) => "InvoiceHeader",
        Element::LineItemTable(_) => "LineItemTable",
        Element::PaymentTerms(_) => "PaymentTerms",
        Element::SignatureBlock(_) => "SignatureBlock",
        Element::Stamp(_) => "Stamp",
        Element::Annotation(_) => "Annotation",
        Element::Redaction(_) => "Redaction",
        Element::FormField(_) => "FormField",
        Element::VariablePlaceholder(_) => "VariablePlaceholder",
    }
}

fn classify_field_impact(element_type: &str, field: &str) -> SemanticImpact {
    match field {
        "document_state" => SemanticImpact::Critical,
        "locked" | "signed_at" | "certificate_fingerprint" => SemanticImpact::Critical,
        "subtotal" | "total" | "tax_amount" | "discount" => SemanticImpact::Major,
        "invoice_number" | "issue_date" | "due_date" => SemanticImpact::Major,
        "text" | "code" | "label" | "field_name" => SemanticImpact::Moderate,
        "created_at" | "modified_at" => SemanticImpact::Minor,
        "font_family" | "font_size" | "color" | "width" | "height" => SemanticImpact::Minor,
        _ => match element_type {
            "PaymentTerms" | "InvoiceHeader" | "LineItemTable" => SemanticImpact::Major,
            "SignatureBlock" => SemanticImpact::Critical,
            _ => SemanticImpact::Moderate,
        },
    }
}

fn diff_json_objects(
    eid: &str,
    element_type: &str,
    old: &serde_json::Value,
    new: &serde_json::Value,
) -> Vec<DiffChange> {
    let mut changes = Vec::new();

    let old_obj = old.as_object();
    let new_obj = new.as_object();

    if let (Some(old_map), Some(new_map)) = (old_obj, new_obj) {
        for (key, old_val) in old_map {
            if key == "eid" || key == "element_type" {
                continue;
            }
            match new_map.get(key) {
                Some(new_val) if old_val != new_val => {
                    changes.push(DiffChange {
                        change_type: ChangeType::Modified,
                        eid: eid.to_string(),
                        element_type: element_type.to_string(),
                        field: Some(key.clone()),
                        old_value: Some(old_val.clone()),
                        new_value: Some(new_val.clone()),
                        impact: classify_field_impact(element_type, key),
                    });
                }
                None => {
                    changes.push(DiffChange {
                        change_type: ChangeType::Modified,
                        eid: eid.to_string(),
                        element_type: element_type.to_string(),
                        field: Some(key.clone()),
                        old_value: Some(old_val.clone()),
                        new_value: None,
                        impact: classify_field_impact(element_type, key),
                    });
                }
                _ => {}
            }
        }
        for (key, new_val) in new_map {
            if key == "eid" || key == "element_type" {
                continue;
            }
            if !old_map.contains_key(key) {
                changes.push(DiffChange {
                    change_type: ChangeType::Modified,
                    eid: eid.to_string(),
                    element_type: element_type.to_string(),
                    field: Some(key.clone()),
                    old_value: None,
                    new_value: Some(new_val.clone()),
                    impact: classify_field_impact(element_type, key),
                });
            }
        }
    }

    changes
}

fn collect_elements(doc: &Document) -> HashMap<String, &Element> {
    let mut map = HashMap::new();
    for page in &doc.pages {
        for element in &page.elements {
            map.insert(element.eid().0.clone(), element);
        }
    }
    map
}

/// Compare two SPDF documents and produce a diff report.
pub fn diff_documents(doc_a_bytes: &[u8], doc_b_bytes: &[u8]) -> SpdfResult<DiffReport> {
    let ext_a = read_container(doc_a_bytes)?;
    let ext_b = read_container(doc_b_bytes)?;

    let doc_a: Document = serde_json::from_slice(&ext_a.semantic)?;
    let doc_b: Document = serde_json::from_slice(&ext_b.semantic)?;

    let mut metadata_changes = Vec::new();

    // Compare metadata fields
    if doc_a.title != doc_b.title {
        metadata_changes.push(DiffChange {
            change_type: ChangeType::Modified,
            eid: "document".to_string(),
            element_type: "Document".to_string(),
            field: Some("title".to_string()),
            old_value: Some(serde_json::Value::String(doc_a.title.clone())),
            new_value: Some(serde_json::Value::String(doc_b.title.clone())),
            impact: SemanticImpact::Moderate,
        });
    }
    if doc_a.locale != doc_b.locale {
        metadata_changes.push(DiffChange {
            change_type: ChangeType::Modified,
            eid: "document".to_string(),
            element_type: "Document".to_string(),
            field: Some("locale".to_string()),
            old_value: Some(serde_json::Value::String(doc_a.locale.clone())),
            new_value: Some(serde_json::Value::String(doc_b.locale.clone())),
            impact: SemanticImpact::Minor,
        });
    }
    if doc_a.direction != doc_b.direction {
        metadata_changes.push(DiffChange {
            change_type: ChangeType::Modified,
            eid: "document".to_string(),
            element_type: "Document".to_string(),
            field: Some("direction".to_string()),
            old_value: Some(serde_json::to_value(&doc_a.direction).unwrap_or_default()),
            new_value: Some(serde_json::to_value(&doc_b.direction).unwrap_or_default()),
            impact: SemanticImpact::Minor,
        });
    }
    if doc_a.document_state != doc_b.document_state {
        metadata_changes.push(DiffChange {
            change_type: ChangeType::Modified,
            eid: "document".to_string(),
            element_type: "Document".to_string(),
            field: Some("document_state".to_string()),
            old_value: Some(serde_json::to_value(&doc_a.document_state).unwrap_or_default()),
            new_value: Some(serde_json::to_value(&doc_b.document_state).unwrap_or_default()),
            impact: SemanticImpact::Critical,
        });
    }

    // Compare elements
    let map_a = collect_elements(&doc_a);
    let map_b = collect_elements(&doc_b);

    let mut element_changes = Vec::new();

    // Removed: in A but not B
    for (eid, el) in &map_a {
        if !map_b.contains_key(eid) {
            element_changes.push(DiffChange {
                change_type: ChangeType::Removed,
                eid: eid.clone(),
                element_type: element_type_name(el).to_string(),
                field: None,
                old_value: None,
                new_value: None,
                impact: classify_field_impact(element_type_name(el), ""),
            });
        }
    }

    // Added: in B but not A
    for (eid, el) in &map_b {
        if !map_a.contains_key(eid) {
            element_changes.push(DiffChange {
                change_type: ChangeType::Added,
                eid: eid.clone(),
                element_type: element_type_name(el).to_string(),
                field: None,
                old_value: None,
                new_value: None,
                impact: classify_field_impact(element_type_name(el), ""),
            });
        }
    }

    // Modified: in both, compare via JSON
    for (eid, el_a) in &map_a {
        if let Some(el_b) = map_b.get(eid) {
            let json_a = serde_json::to_value(el_a).unwrap_or_default();
            let json_b = serde_json::to_value(el_b).unwrap_or_default();
            if json_a != json_b {
                let field_changes =
                    diff_json_objects(eid, element_type_name(el_a), &json_a, &json_b);
                element_changes.extend(field_changes);
            }
        }
    }

    let added = element_changes
        .iter()
        .filter(|c| c.change_type == ChangeType::Added)
        .count();
    let removed = element_changes
        .iter()
        .filter(|c| c.change_type == ChangeType::Removed)
        .count();
    let modified = element_changes
        .iter()
        .filter(|c| c.change_type == ChangeType::Modified)
        .count();
    let total = metadata_changes.len() + element_changes.len();

    let highest_impact = metadata_changes
        .iter()
        .chain(element_changes.iter())
        .map(|c| c.impact)
        .max()
        .unwrap_or(SemanticImpact::None);

    Ok(DiffReport {
        metadata_changes,
        element_changes,
        summary: DiffSummary {
            added,
            removed,
            modified,
            total_changes: total,
            highest_impact,
        },
    })
}
