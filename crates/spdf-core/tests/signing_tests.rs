use spdf_core::container::{read_container, write_container, ContainerLayers};
use spdf_core::dom::*;
use spdf_core::manifest::Manifest;
use spdf_core::signing::{sign_document_simple, transition_document, verify_document_simple};
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

fn make_review_doc() -> Document {
    Document {
        version: SpdfVersion::V1_0,
        document_id: DocumentId::new(),
        title: "Test Document".to_string(),
        locale: "en-US".to_string(),
        direction: TextDirection::Ltr,
        document_state: DocumentState::Review,
        pages: vec![Page {
            eid: eid(),
            page_number: 1,
            elements: vec![
                Element::Heading(HeadingElement {
                    eid: eid(),
                    level: 1,
                    text: "Test Heading".to_string(),
                    font_family: None,
                    font_size: None,
                    color: None,
                    timestamps: ts(),
                }),
                Element::SignatureBlock(SignatureBlockElement {
                    eid: eid(),
                    signer_name: "John Doe".to_string(),
                    signer_title: Some("CEO".to_string()),
                    signed_at: None,
                    certificate_fingerprint: None,
                    locked: false,
                    timestamps: ts(),
                }),
            ],
        }],
    }
}

fn make_draft_doc() -> Document {
    let mut doc = make_review_doc();
    doc.document_state = DocumentState::Draft;
    doc
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

// ---------- sign_document_simple ----------

#[test]
fn sign_review_document_succeeds() {
    let doc = make_review_doc();
    let spdf = build_spdf(&doc);
    let signed = sign_document_simple(&spdf, "Alice", "alice@spdf.dev").unwrap();
    assert!(!signed.is_empty());
}

#[test]
fn signed_document_state_is_signed() {
    let doc = make_review_doc();
    let spdf = build_spdf(&doc);
    let signed = sign_document_simple(&spdf, "Alice", "alice@spdf.dev").unwrap();
    let extracted = read_container(&signed).unwrap();
    let result: Document = serde_json::from_slice(&extracted.semantic).unwrap();
    assert_eq!(result.document_state, DocumentState::Signed);
}

#[test]
fn signed_document_has_signature_entry() {
    let doc = make_review_doc();
    let spdf = build_spdf(&doc);
    let signed = sign_document_simple(&spdf, "Alice", "alice@spdf.dev").unwrap();

    let cursor = std::io::Cursor::new(&signed);
    let mut archive = zip::ZipArchive::new(cursor).unwrap();
    let mut found = false;
    for i in 0..archive.len() {
        let entry = archive.by_index(i).unwrap();
        if entry.name() == "signatures/signature_001.json" {
            found = true;
            break;
        }
    }
    assert!(found, "signatures/signature_001.json not found in archive");
}

#[test]
fn signed_document_locks_signature_blocks() {
    let doc = make_review_doc();
    let spdf = build_spdf(&doc);
    let signed = sign_document_simple(&spdf, "Alice", "alice@spdf.dev").unwrap();
    let extracted = read_container(&signed).unwrap();
    let result: Document = serde_json::from_slice(&extracted.semantic).unwrap();

    for page in &result.pages {
        for element in &page.elements {
            if let Element::SignatureBlock(sb) = element {
                assert!(sb.locked, "SignatureBlock should be locked after signing");
                assert!(
                    sb.signed_at.is_some(),
                    "SignatureBlock should have signed_at"
                );
            }
        }
    }
}

#[test]
fn signed_document_has_audit_event() {
    let doc = make_review_doc();
    let spdf = build_spdf(&doc);
    let signed = sign_document_simple(&spdf, "Alice", "alice@spdf.dev").unwrap();
    let extracted = read_container(&signed).unwrap();
    let audit: serde_json::Value = serde_json::from_slice(&extracted.audit).unwrap();
    let entries = audit["entries"].as_array().unwrap();
    assert!(!entries.is_empty());
    assert_eq!(entries.last().unwrap()["event"], "STATE_CHANGED");
    assert_eq!(entries.last().unwrap()["to"], "SIGNED");
}

#[test]
fn sign_draft_document_fails_wrong_state() {
    let doc = make_draft_doc();
    let spdf = build_spdf(&doc);
    let result = sign_document_simple(&spdf, "Alice", "alice@spdf.dev");
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("wrong document state"));
}

#[test]
fn sign_already_signed_fails() {
    let doc = make_review_doc();
    let spdf = build_spdf(&doc);
    let signed = sign_document_simple(&spdf, "Alice", "alice@spdf.dev").unwrap();
    let result = sign_document_simple(&signed, "Bob", "bob@spdf.dev");
    assert!(result.is_err());
}

#[test]
fn signature_record_has_correct_fields() {
    let doc = make_review_doc();
    let spdf = build_spdf(&doc);
    let signed = sign_document_simple(&spdf, "Alice", "alice@spdf.dev").unwrap();

    let cursor = std::io::Cursor::new(&signed);
    let mut archive = zip::ZipArchive::new(cursor).unwrap();
    let mut entry = archive.by_name("signatures/signature_001.json").unwrap();
    let mut buf = Vec::new();
    std::io::Read::read_to_end(&mut entry, &mut buf).unwrap();
    let record: serde_json::Value = serde_json::from_slice(&buf).unwrap();

    assert!(record["signature_id"].as_str().unwrap().starts_with("sig-"));
    assert_eq!(record["signer_name"], "Alice");
    assert_eq!(record["signer_email"], "alice@spdf.dev");
    assert_eq!(record["algorithm"], "SHA256_SIMPLE");
    assert!(!record["content_hash"].as_str().unwrap().is_empty());
    assert!(!record["signed_at"].as_str().unwrap().is_empty());
}

// ---------- verify_document_simple ----------

#[test]
fn verify_signed_document_is_valid() {
    let doc = make_review_doc();
    let spdf = build_spdf(&doc);
    let signed = sign_document_simple(&spdf, "Alice", "alice@spdf.dev").unwrap();
    let report = verify_document_simple(&signed).unwrap();

    assert!(report.valid);
    assert!(!report.tamper_detected);
    assert_eq!(report.signature_count, 1);
    assert!(report.signatures[0].valid);
}

#[test]
fn verify_unsigned_document_reports_no_signatures() {
    let doc = make_review_doc();
    let spdf = build_spdf(&doc);
    let report = verify_document_simple(&spdf).unwrap();

    assert!(!report.valid);
    assert!(!report.tamper_detected);
    assert_eq!(report.signature_count, 0);
}

#[test]
fn verify_tampered_document_detects_tampering() {
    let doc = make_review_doc();
    let spdf = build_spdf(&doc);
    let signed = sign_document_simple(&spdf, "Alice", "alice@spdf.dev").unwrap();

    // Tamper: re-read, modify semantic, rebuild without updating signature
    let extracted = read_container(&signed).unwrap();
    let mut tampered_doc: Document = serde_json::from_slice(&extracted.semantic).unwrap();
    tampered_doc.title = "TAMPERED TITLE".to_string();
    let tampered_semantic = serde_json::to_vec_pretty(&tampered_doc).unwrap();

    // Rebuild with tampered semantic but keep old signature
    let cursor = std::io::Cursor::new(&signed);
    let mut archive = zip::ZipArchive::new(cursor).unwrap();
    let mut sig_buf = Vec::new();
    {
        let mut entry = archive.by_name("signatures/signature_001.json").unwrap();
        std::io::Read::read_to_end(&mut entry, &mut sig_buf).unwrap();
    }

    let mut manifest = extracted.manifest.clone();
    manifest.layers.semantic = spdf_core::manifest::sha256_hex(&tampered_semantic);
    manifest.finalize();

    let mut buf = Vec::new();
    let mut zip_w = zip::ZipWriter::new(std::io::Cursor::new(&mut buf));
    let opts =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let stored =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    zip_w.start_file("manifest.json", opts).unwrap();
    std::io::Write::write_all(&mut zip_w, &serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
    zip_w.start_file("layers/semantic.json", opts).unwrap();
    std::io::Write::write_all(&mut zip_w, &tampered_semantic).unwrap();
    zip_w.start_file("layers/layout.json", opts).unwrap();
    std::io::Write::write_all(&mut zip_w, &extracted.layout).unwrap();
    zip_w.start_file("layers/styles.json", opts).unwrap();
    std::io::Write::write_all(&mut zip_w, &extracted.styles).unwrap();
    zip_w.start_file("layers/render.pdf", stored).unwrap();
    std::io::Write::write_all(&mut zip_w, &extracted.render).unwrap();
    zip_w.start_file("layers/metadata.json", opts).unwrap();
    std::io::Write::write_all(&mut zip_w, &extracted.metadata).unwrap();
    zip_w.start_file("layers/audit.json", opts).unwrap();
    std::io::Write::write_all(&mut zip_w, &extracted.audit).unwrap();
    zip_w
        .start_file("signatures/signature_001.json", opts)
        .unwrap();
    std::io::Write::write_all(&mut zip_w, &sig_buf).unwrap();
    zip_w.set_comment("SPDF/1.0");
    zip_w.finish().unwrap();

    let report = verify_document_simple(&buf).unwrap();
    assert!(!report.valid);
    assert!(report.tamper_detected);
}

#[test]
fn verify_reports_signer_details() {
    let doc = make_review_doc();
    let spdf = build_spdf(&doc);
    let signed = sign_document_simple(&spdf, "Bob Smith", "bob@example.com").unwrap();
    let report = verify_document_simple(&signed).unwrap();

    assert_eq!(report.signatures[0].signer_name, "Bob Smith");
    assert_eq!(report.signatures[0].signer_email, "bob@example.com");
}

// ---------- transition_document ----------

#[test]
fn transition_draft_to_review() {
    let doc = make_draft_doc();
    let spdf = build_spdf(&doc);
    let result = transition_document(&spdf, DocumentState::Review).unwrap();
    let extracted = read_container(&result).unwrap();
    let transitioned: Document = serde_json::from_slice(&extracted.semantic).unwrap();
    assert_eq!(transitioned.document_state, DocumentState::Review);
}

#[test]
fn transition_invalid_state_fails() {
    let doc = make_draft_doc();
    let spdf = build_spdf(&doc);
    let result = transition_document(&spdf, DocumentState::Signed);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("invalid state transition"));
}

#[test]
fn transition_appends_audit_event() {
    let doc = make_draft_doc();
    let spdf = build_spdf(&doc);
    let result = transition_document(&spdf, DocumentState::Review).unwrap();
    let extracted = read_container(&result).unwrap();
    let audit: serde_json::Value = serde_json::from_slice(&extracted.audit).unwrap();
    let entries = audit["entries"].as_array().unwrap();
    assert_eq!(entries.last().unwrap()["event"], "STATE_CHANGED");
}
