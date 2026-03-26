use spdf_core::container::{read_container, write_container, ContainerLayers};
use spdf_core::dom::*;
use spdf_core::manifest::Manifest;
use spdf_core::redaction::{list_redactions, redact_element, verify_redaction};
use spdf_core::types::*;

fn ts() -> Timestamps {
    Timestamps::now()
}

fn eid() -> ElementId {
    ElementId::new()
}

fn test_generator() -> GeneratorInfo {
    GeneratorInfo {
        name: "spdf-test".to_string(),
        version: "0.0.1".to_string(),
    }
}

fn build_spdf(doc: &Document) -> Vec<u8> {
    let semantic = serde_json::to_vec_pretty(doc).unwrap();
    let mut manifest = Manifest::new(doc.document_id.clone(), test_generator());
    let layers = ContainerLayers {
        semantic,
        layout: br#"{"layout":"default"}"#.to_vec(),
        styles: br#"{"styles":{}}"#.to_vec(),
        render: b"%PDF-1.4 fake".to_vec(),
        metadata: br#"{"author":"test"}"#.to_vec(),
        audit: br#"{"entries":[]}"#.to_vec(),
    };
    write_container(&mut manifest, &layers, &[]).unwrap()
}

fn make_doc_with_known_eids() -> (Document, String, String) {
    let heading_eid = eid();
    let para_eid = eid();
    let h_str = heading_eid.0.clone();
    let p_str = para_eid.0.clone();
    let doc = Document {
        version: SpdfVersion::V1_0,
        document_id: DocumentId::new(),
        title: "Redaction Test".to_string(),
        locale: "en-US".to_string(),
        direction: TextDirection::Ltr,
        document_state: DocumentState::Draft,
        pages: vec![Page {
            eid: eid(),
            page_number: 1,
            elements: vec![
                Element::Heading(HeadingElement {
                    eid: heading_eid,
                    level: 1,
                    text: "Sensitive Heading".to_string(),
                    font_family: None,
                    font_size: None,
                    color: None,
                    timestamps: ts(),
                }),
                Element::Paragraph(ParagraphElement {
                    eid: para_eid,
                    text: "Contains PII data".to_string(),
                    font_family: None,
                    font_size: None,
                    color: None,
                    timestamps: ts(),
                }),
            ],
        }],
    };
    (doc, h_str, p_str)
}

// ---------- redact_element ----------

#[test]
fn redact_element_succeeds() {
    let (doc, _, para_eid) = make_doc_with_known_eids();
    let spdf = build_spdf(&doc);
    let result = redact_element(&spdf, &para_eid, "Contains PII");
    assert!(result.is_ok());
}

#[test]
fn redacted_element_replaced_with_redaction() {
    let (doc, _, para_eid) = make_doc_with_known_eids();
    let spdf = build_spdf(&doc);
    let redacted = redact_element(&spdf, &para_eid, "PII").unwrap();
    let extracted = read_container(&redacted).unwrap();
    let result: Document = serde_json::from_slice(&extracted.semantic).unwrap();

    let has_redaction = result.pages[0]
        .elements
        .iter()
        .any(|e| matches!(e, Element::Redaction(_)));
    assert!(has_redaction);
}

#[test]
fn redacted_element_has_proof_hash() {
    let (doc, _, para_eid) = make_doc_with_known_eids();
    let spdf = build_spdf(&doc);
    let redacted = redact_element(&spdf, &para_eid, "PII").unwrap();
    let extracted = read_container(&redacted).unwrap();
    let result: Document = serde_json::from_slice(&extracted.semantic).unwrap();

    for element in &result.pages[0].elements {
        if let Element::Redaction(r) = element {
            assert!(!r.erasure_proof_hash.is_empty());
            assert_eq!(r.redacted_eid.0, para_eid);
            return;
        }
    }
    panic!("No redaction element found");
}

#[test]
fn redact_nonexistent_element_fails() {
    let (doc, _, _) = make_doc_with_known_eids();
    let spdf = build_spdf(&doc);
    let result = redact_element(&spdf, "el-nonexistent-0000", "reason");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));
}

#[test]
fn redact_preserves_other_elements() {
    let (doc, heading_eid, para_eid) = make_doc_with_known_eids();
    let spdf = build_spdf(&doc);
    let redacted = redact_element(&spdf, &para_eid, "PII").unwrap();
    let extracted = read_container(&redacted).unwrap();
    let result: Document = serde_json::from_slice(&extracted.semantic).unwrap();

    let has_heading = result.pages[0].elements.iter().any(|e| {
        if let Element::Heading(h) = e {
            h.eid.0 == heading_eid
        } else {
            false
        }
    });
    assert!(has_heading, "Heading should be preserved");
}

#[test]
fn redact_appends_audit_event() {
    let (doc, _, para_eid) = make_doc_with_known_eids();
    let spdf = build_spdf(&doc);
    let redacted = redact_element(&spdf, &para_eid, "PII").unwrap();
    let extracted = read_container(&redacted).unwrap();
    let audit: serde_json::Value = serde_json::from_slice(&extracted.audit).unwrap();
    let entries = audit["entries"].as_array().unwrap();
    assert!(!entries.is_empty());
    assert_eq!(entries.last().unwrap()["event"], "REDACTED");
}

#[test]
fn multiple_redactions() {
    let (doc, heading_eid, para_eid) = make_doc_with_known_eids();
    let spdf = build_spdf(&doc);
    let redacted1 = redact_element(&spdf, &para_eid, "PII").unwrap();
    let redacted2 = redact_element(&redacted1, &heading_eid, "Classified").unwrap();

    let entries = list_redactions(&redacted2).unwrap();
    assert_eq!(entries.len(), 2);
}

// ---------- list_redactions ----------

#[test]
fn list_redactions_empty_on_clean_doc() {
    let (doc, _, _) = make_doc_with_known_eids();
    let spdf = build_spdf(&doc);
    let entries = list_redactions(&spdf).unwrap();
    assert!(entries.is_empty());
}

#[test]
fn list_redactions_shows_redacted_elements() {
    let (doc, _, para_eid) = make_doc_with_known_eids();
    let spdf = build_spdf(&doc);
    let redacted = redact_element(&spdf, &para_eid, "PII data").unwrap();

    let entries = list_redactions(&redacted).unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].redacted_eid, para_eid);
    assert_eq!(entries[0].reason, "PII data");
}

// ---------- verify_redaction ----------

#[test]
fn verify_redaction_found() {
    let (doc, _, para_eid) = make_doc_with_known_eids();
    let spdf = build_spdf(&doc);
    let redacted = redact_element(&spdf, &para_eid, "PII").unwrap();

    let verification = verify_redaction(&redacted, &para_eid).unwrap();
    assert!(verification.found);
    assert!(!verification.proof_hash.is_empty());
}

#[test]
fn verify_redaction_not_found() {
    let (doc, _, _) = make_doc_with_known_eids();
    let spdf = build_spdf(&doc);
    let verification = verify_redaction(&spdf, "el-nonexistent-0000").unwrap();
    assert!(!verification.found);
}
