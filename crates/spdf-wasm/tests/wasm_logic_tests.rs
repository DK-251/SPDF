//! Tests for WASM internal helper functions (pure Rust, no wasm-bindgen needed).

use spdf_core::container::{write_container, ContainerLayers};
use spdf_core::dom::*;
use spdf_core::manifest::Manifest;
use spdf_core::signing::sign_document_simple;
use spdf_core::types::*;

use spdf_wasm::{
    diff_internal, extract_invoice_internal, get_document_info_internal, list_redactions_internal,
    validate_internal, verify_internal,
};

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

fn make_simple_doc() -> Document {
    Document {
        version: SpdfVersion::V1_0,
        document_id: DocumentId::new(),
        title: "WASM Test".to_string(),
        locale: "en-US".to_string(),
        direction: TextDirection::Ltr,
        document_state: DocumentState::Draft,
        pages: vec![Page {
            eid: eid(),
            page_number: 1,
            elements: vec![Element::Heading(HeadingElement {
                eid: eid(),
                level: 1,
                text: "Hello WASM".to_string(),
                font_family: None,
                font_size: None,
                color: None,
                timestamps: ts(),
            })],
        }],
    }
}

fn make_invoice_doc() -> Document {
    Document {
        version: SpdfVersion::V1_0,
        document_id: DocumentId::new(),
        title: "Invoice WASM Test".to_string(),
        locale: "en-IN".to_string(),
        direction: TextDirection::Ltr,
        document_state: DocumentState::Draft,
        pages: vec![Page {
            eid: eid(),
            page_number: 1,
            elements: vec![
                Element::InvoiceHeader(InvoiceHeaderElement {
                    eid: eid(),
                    invoice_number: "INV-001".to_string(),
                    issue_date: "2026-01-01".to_string(),
                    due_date: "2026-02-01".to_string(),
                    vendor: PartyInfo {
                        name: "Vendor Co".to_string(),
                        address: None,
                        gstin: None,
                    },
                    client: PartyInfo {
                        name: "Client Inc".to_string(),
                        address: None,
                        gstin: None,
                    },
                    currency: Some("INR".to_string()),
                    timestamps: ts(),
                }),
                Element::PaymentTerms(PaymentTermsElement {
                    eid: eid(),
                    subtotal: "1000.00".to_string(),
                    discount: None,
                    tax_label: None,
                    tax_amount: None,
                    total: "1000.00".to_string(),
                    payment_method: None,
                    timestamps: ts(),
                }),
            ],
        }],
    }
}

// ---------- validate ----------

#[test]
fn validate_valid_document() {
    let doc = make_simple_doc();
    let spdf = build_spdf(&doc);
    let result = validate_internal(&spdf).unwrap();
    let report: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(report["valid"], true);
}

#[test]
fn validate_invalid_bytes() {
    let result = validate_internal(b"not a zip");
    assert!(result.is_err());
}

// ---------- get_document_info ----------

#[test]
fn get_document_info_returns_metadata() {
    let doc = make_simple_doc();
    let spdf = build_spdf(&doc);
    let result = get_document_info_internal(&spdf).unwrap();
    let info: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(info["title"], "WASM Test");
    assert_eq!(info["locale"], "en-US");
    assert_eq!(info["page_count"], 1);
    assert_eq!(info["element_count"], 1);
}

// ---------- verify ----------

#[test]
fn verify_unsigned_document() {
    let doc = make_simple_doc();
    let spdf = build_spdf(&doc);
    let result = verify_internal(&spdf).unwrap();
    let report: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(report["valid"], false);
    assert_eq!(report["signature_count"], 0);
}

#[test]
fn verify_signed_document() {
    let mut doc = make_simple_doc();
    doc.document_state = DocumentState::Review;
    doc.pages[0]
        .elements
        .push(Element::SignatureBlock(SignatureBlockElement {
            eid: eid(),
            signer_name: "Test".to_string(),
            signer_title: None,
            signed_at: None,
            certificate_fingerprint: None,
            locked: false,
            timestamps: ts(),
        }));
    let spdf = build_spdf(&doc);
    let signed = sign_document_simple(&spdf, "Tester", "test@spdf.dev").unwrap();
    let result = verify_internal(&signed).unwrap();
    let report: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(report["valid"], true);
}

// ---------- diff ----------

#[test]
fn diff_identical_documents() {
    let doc = make_simple_doc();
    let spdf = build_spdf(&doc);
    let result = diff_internal(&spdf, &spdf).unwrap();
    let report: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(report["summary"]["total_changes"], 0);
}

#[test]
fn diff_different_documents() {
    let doc_a = make_simple_doc();
    let mut doc_b = make_simple_doc();
    doc_b.title = "Different Title".to_string();
    let spdf_a = build_spdf(&doc_a);
    let spdf_b = build_spdf(&doc_b);
    let result = diff_internal(&spdf_a, &spdf_b).unwrap();
    let report: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert!(report["summary"]["total_changes"].as_u64().unwrap() > 0);
}

// ---------- list_redactions ----------

#[test]
fn list_redactions_empty() {
    let doc = make_simple_doc();
    let spdf = build_spdf(&doc);
    let result = list_redactions_internal(&spdf).unwrap();
    let entries: Vec<serde_json::Value> = serde_json::from_str(&result).unwrap();
    assert!(entries.is_empty());
}

// ---------- extract_invoice ----------

#[test]
fn extract_invoice_with_data() {
    let doc = make_invoice_doc();
    let spdf = build_spdf(&doc);
    let result = extract_invoice_internal(&spdf).unwrap();
    let data: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(data["has_invoice"], true);
    assert_eq!(data["invoice_number"], "INV-001");
    assert_eq!(data["total"], "1000.00");
    assert_eq!(data["currency"], "INR");
}

#[test]
fn extract_invoice_without_data() {
    let doc = make_simple_doc();
    let spdf = build_spdf(&doc);
    let result = extract_invoice_internal(&spdf).unwrap();
    let data: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(data["has_invoice"], false);
}
