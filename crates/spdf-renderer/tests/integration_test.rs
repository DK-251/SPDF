//! Full round-trip integration test:
//! Create document → serialize DOM → render PDF → write container → read container → validate

use spdf_core::container::{read_container, write_container, ContainerLayers};
use spdf_core::dom::*;
use spdf_core::manifest::Manifest;
use spdf_core::types::*;
use spdf_renderer::render_to_pdf;
use spdf_validator::{validate_document, validate_manifest};

fn ts() -> Timestamps {
    Timestamps::now()
}

fn eid() -> ElementId {
    ElementId::new()
}

fn sample_invoice() -> Document {
    Document {
        version: SpdfVersion::V1_0,
        document_id: DocumentId::new(),
        title: "Invoice INV-2026-100".to_string(),
        locale: "en-IN".to_string(),
        direction: TextDirection::Ltr,
        document_state: DocumentState::Draft,
        pages: vec![Page {
            eid: eid(),
            page_number: 1,
            elements: vec![
                Element::Heading(HeadingElement {
                    eid: eid(),
                    level: 1,
                    text: "SPDF Corp — Tax Invoice".to_string(),
                    font_family: None,
                    font_size: None,
                    color: None,
                    timestamps: ts(),
                }),
                Element::InvoiceHeader(InvoiceHeaderElement {
                    eid: eid(),
                    invoice_number: "INV-2026-100".to_string(),
                    issue_date: "2026-03-25".to_string(),
                    due_date: "2026-04-25".to_string(),
                    vendor: PartyInfo {
                        name: "SPDF Corp".to_string(),
                        address: Some("HSR Layout, Bangalore 560102".to_string()),
                        gstin: Some("29AABCU9603R1ZM".to_string()),
                    },
                    client: PartyInfo {
                        name: "Acme Industries Pvt Ltd".to_string(),
                        address: Some("MG Road, Mumbai 400001".to_string()),
                        gstin: Some("27AADCA1234B1ZK".to_string()),
                    },
                    currency: Some("INR".to_string()),
                    timestamps: ts(),
                }),
                Element::LineItemTable(LineItemTableElement {
                    eid: eid(),
                    headers: vec![
                        "Description".into(),
                        "Qty".into(),
                        "Rate".into(),
                        "Amount".into(),
                    ],
                    rows: vec![
                        vec![
                            TableCell {
                                value: "API Integration Service".into(),
                                spdf_type: None,
                            },
                            TableCell {
                                value: "1".into(),
                                spdf_type: Some("integer".into()),
                            },
                            TableCell {
                                value: "75000.00".into(),
                                spdf_type: Some("currency".into()),
                            },
                            TableCell {
                                value: "75000.00".into(),
                                spdf_type: Some("currency".into()),
                            },
                        ],
                        vec![
                            TableCell {
                                value: "Custom PDF Templates (5)".into(),
                                spdf_type: None,
                            },
                            TableCell {
                                value: "5".into(),
                                spdf_type: Some("integer".into()),
                            },
                            TableCell {
                                value: "10000.00".into(),
                                spdf_type: Some("currency".into()),
                            },
                            TableCell {
                                value: "50000.00".into(),
                                spdf_type: Some("currency".into()),
                            },
                        ],
                    ],
                    timestamps: ts(),
                }),
                Element::HorizontalRule(HorizontalRuleElement {
                    eid: eid(),
                    timestamps: ts(),
                }),
                Element::PaymentTerms(PaymentTermsElement {
                    eid: eid(),
                    subtotal: "125000.00".to_string(),
                    discount: Some("12500.00".to_string()),
                    tax_label: Some("IGST 18%".to_string()),
                    tax_amount: Some("20250.00".to_string()),
                    total: "132750.00".to_string(),
                    payment_method: Some("NEFT".to_string()),
                    timestamps: ts(),
                }),
                Element::SignatureBlock(SignatureBlockElement {
                    eid: eid(),
                    signer_name: "Deepak Sahu".to_string(),
                    signer_title: Some("Founder & Director".to_string()),
                    signed_at: None,
                    certificate_fingerprint: None,
                    locked: false,
                    timestamps: ts(),
                }),
            ],
        }],
    }
}

#[test]
fn full_round_trip_invoice() {
    // 1. Create document
    let doc = sample_invoice();

    // 2. Validate the document structure
    let doc_report = validate_document(&doc);
    assert!(
        doc_report.is_valid(),
        "Document validation failed: {:?}",
        doc_report.errors
    );

    // 3. Serialize DOM to JSON (semantic layer)
    let semantic_json = serde_json::to_vec_pretty(&doc).expect("DOM serialization failed");
    assert!(!semantic_json.is_empty());

    // 4. Render to PDF
    let pdf_bytes = render_to_pdf(&doc).expect("PDF rendering failed");
    assert!(
        pdf_bytes.len() > 100,
        "PDF too small: {} bytes",
        pdf_bytes.len()
    );
    // Verify it starts with %PDF
    assert_eq!(
        &pdf_bytes[..5],
        b"%PDF-",
        "rendered output is not a valid PDF"
    );

    // 5. Build container layers
    let layers = ContainerLayers {
        semantic: semantic_json,
        layout: serde_json::to_vec(&serde_json::json!({"layout": "default"})).unwrap(),
        styles: serde_json::to_vec(&serde_json::json!({"styles": {}})).unwrap(),
        render: pdf_bytes,
        metadata: serde_json::to_vec(&serde_json::json!({
            "title": doc.title,
            "locale": doc.locale,
        }))
        .unwrap(),
        audit: serde_json::to_vec(&serde_json::json!({"events": []})).unwrap(),
    };

    // 6. Write to .spdf container
    let mut manifest = Manifest::new(
        doc.document_id.clone(),
        GeneratorInfo {
            name: "spdf-integration-test".to_string(),
            version: "0.1.0".to_string(),
        },
    );
    let container_bytes =
        write_container(&mut manifest, &layers, &[]).expect("Container write failed");
    assert!(container_bytes.len() > 0);

    // 7. Validate the manifest
    let manifest_report = validate_manifest(&manifest);
    assert!(
        manifest_report.is_valid(),
        "Manifest validation failed: {:?}",
        manifest_report.errors
    );

    // 8. Read container back
    let extracted = read_container(&container_bytes).expect("Container read failed");

    // 9. Verify all layers survived the round-trip
    assert_eq!(extracted.manifest.document_id, doc.document_id);
    assert_eq!(extracted.manifest.format, "SPDF");

    // 10. Deserialize the semantic layer back to a Document
    let rt_doc: Document =
        serde_json::from_slice(&extracted.semantic).expect("DOM deserialization failed");
    assert_eq!(rt_doc.title, "Invoice INV-2026-100");
    assert_eq!(rt_doc.pages.len(), 1);
    assert_eq!(rt_doc.pages[0].elements.len(), 7);

    // 11. Validate the round-tripped document
    let rt_report = validate_document(&rt_doc);
    assert!(
        rt_report.is_valid(),
        "Round-tripped doc validation failed: {:?}",
        rt_report.errors
    );

    // 12. Verify the rendered PDF layer starts with %PDF-
    assert_eq!(&extracted.render[..5], b"%PDF-");

    // 13. Financial values survived as strings (not coerced to floats)
    let semantic_str = String::from_utf8_lossy(&extracted.semantic);
    assert!(semantic_str.contains(r#""132750.00"#));
    assert!(semantic_str.contains(r#""20250.00"#));
}

#[test]
fn multi_page_round_trip() {
    let doc = Document {
        version: SpdfVersion::V1_0,
        document_id: DocumentId::new(),
        title: "Multi-Page Report".to_string(),
        locale: "en-US".to_string(),
        direction: TextDirection::Ltr,
        document_state: DocumentState::Review,
        pages: vec![
            Page {
                eid: eid(),
                page_number: 1,
                elements: vec![
                    Element::Heading(HeadingElement {
                        eid: eid(),
                        level: 1,
                        text: "Chapter 1".to_string(),
                        font_family: None,
                        font_size: None,
                        color: None,
                        timestamps: ts(),
                    }),
                    Element::Paragraph(ParagraphElement {
                        eid: eid(),
                        text: "Introduction content.".to_string(),
                        font_family: None,
                        font_size: None,
                        color: None,
                        timestamps: ts(),
                    }),
                ],
            },
            Page {
                eid: eid(),
                page_number: 2,
                elements: vec![
                    Element::Heading(HeadingElement {
                        eid: eid(),
                        level: 1,
                        text: "Chapter 2".to_string(),
                        font_family: None,
                        font_size: None,
                        color: None,
                        timestamps: ts(),
                    }),
                    Element::CodeBlock(CodeBlockElement {
                        eid: eid(),
                        language: "rust".to_string(),
                        code: "fn main() {\n    println!(\"Hello SPDF\");\n}".to_string(),
                        timestamps: ts(),
                    }),
                ],
            },
        ],
    };

    // Validate
    let report = validate_document(&doc);
    assert!(report.is_valid());

    // Render
    let pdf_bytes = render_to_pdf(&doc).unwrap();
    assert_eq!(&pdf_bytes[..5], b"%PDF-");

    // Container round-trip
    let semantic = serde_json::to_vec_pretty(&doc).unwrap();
    let layers = ContainerLayers {
        semantic,
        layout: b"{}".to_vec(),
        styles: b"{}".to_vec(),
        render: pdf_bytes,
        metadata: b"{}".to_vec(),
        audit: b"{}".to_vec(),
    };

    let mut manifest = Manifest::new(
        doc.document_id.clone(),
        GeneratorInfo {
            name: "test".to_string(),
            version: "0.1.0".to_string(),
        },
    );
    let bytes = write_container(&mut manifest, &layers, &[]).unwrap();
    let extracted = read_container(&bytes).unwrap();
    let rt_doc: Document = serde_json::from_slice(&extracted.semantic).unwrap();

    assert_eq!(rt_doc.pages.len(), 2);
    assert_eq!(rt_doc.document_state, DocumentState::Review);
}

#[test]
fn round_trip_with_assets() {
    let doc = Document {
        version: SpdfVersion::V1_0,
        document_id: DocumentId::new(),
        title: "Doc with Assets".to_string(),
        locale: "en-US".to_string(),
        direction: TextDirection::Ltr,
        document_state: DocumentState::Draft,
        pages: vec![Page {
            eid: eid(),
            page_number: 1,
            elements: vec![Element::Image(ImageElement {
                eid: eid(),
                asset_id: "logo-001".to_string(),
                alt_text: Some("Company logo".to_string()),
                caption: None,
                width: 200.0,
                height: 80.0,
                timestamps: ts(),
            })],
        }],
    };

    let pdf_bytes = render_to_pdf(&doc).unwrap();
    let semantic = serde_json::to_vec(&doc).unwrap();
    let logo_data = b"FAKE PNG DATA FOR TESTING".to_vec();

    let layers = ContainerLayers {
        semantic,
        layout: b"{}".to_vec(),
        styles: b"{}".to_vec(),
        render: pdf_bytes,
        metadata: b"{}".to_vec(),
        audit: b"{}".to_vec(),
    };

    let assets = vec![("logo-001.png".to_string(), logo_data.clone())];
    let mut manifest = Manifest::new(
        doc.document_id.clone(),
        GeneratorInfo {
            name: "test".to_string(),
            version: "0.1.0".to_string(),
        },
    );

    let bytes = write_container(&mut manifest, &layers, &assets).unwrap();
    let extracted = read_container(&bytes).unwrap();

    assert_eq!(extracted.assets.len(), 1);
    assert_eq!(extracted.assets[0].0, "logo-001.png");
    assert_eq!(extracted.assets[0].1, logo_data);
}
