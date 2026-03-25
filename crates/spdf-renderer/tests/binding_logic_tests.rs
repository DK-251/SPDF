//! Tests that mirror the exact code paths used by the PyO3 binding functions.
//!
//! Since spdf-python is a cdylib (no integration tests possible), these tests
//! exercise the same logic: validate_spdf, generate_spdf, render_to_pdf,
//! parse_semantic, and extract_invoice_data paths — all in pure Rust.

use serde_json::json;
use spdf_core::container::{self, ContainerLayers};
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

fn sample_invoice_doc() -> Document {
    Document {
        version: SpdfVersion::V1_0,
        document_id: DocumentId::new(),
        title: "Invoice INV-2026-200".to_string(),
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
                    text: "Tax Invoice".to_string(),
                    font_family: None,
                    font_size: None,
                    color: None,
                    timestamps: ts(),
                }),
                Element::InvoiceHeader(InvoiceHeaderElement {
                    eid: eid(),
                    invoice_number: "INV-2026-200".to_string(),
                    issue_date: "2026-03-25".to_string(),
                    due_date: "2026-04-25".to_string(),
                    vendor: PartyInfo {
                        name: "SPDF Corp".to_string(),
                        address: Some("HSR Layout, Bangalore 560102".to_string()),
                        gstin: Some("29AABCU9603R1ZM".to_string()),
                    },
                    client: PartyInfo {
                        name: "Acme Industries".to_string(),
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
                                value: "API Integration".into(),
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
                                value: "PDF Templates (5)".into(),
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
            ],
        }],
    }
}

/// Helper: build a valid .spdf container from a Document.
fn build_spdf_container(doc: &Document) -> Vec<u8> {
    let semantic_json = serde_json::to_vec_pretty(doc).unwrap();
    let pdf_bytes = render_to_pdf(doc).unwrap();

    let layers = ContainerLayers {
        semantic: semantic_json,
        layout: serde_json::to_vec(&json!({"layout": "default"})).unwrap(),
        styles: serde_json::to_vec(&json!({"styles": {}})).unwrap(),
        render: pdf_bytes,
        metadata: serde_json::to_vec(&json!({"title": &doc.title, "locale": &doc.locale})).unwrap(),
        audit: serde_json::to_vec(&json!({"entries": []})).unwrap(),
    };

    let mut manifest = Manifest::new(
        doc.document_id.clone(),
        GeneratorInfo {
            name: "spdf-test".to_string(),
            version: "0.1.0".to_string(),
        },
    );

    container::write_container(&mut manifest, &layers, &[]).unwrap()
}

// ========================================================================
// validate_spdf path: read container → validate manifest + document → report
// ========================================================================

#[test]
fn validate_spdf_valid_invoice() {
    let doc = sample_invoice_doc();
    let spdf_bytes = build_spdf_container(&doc);

    let extracted = container::read_container(&spdf_bytes).unwrap();
    let manifest_report = validate_manifest(&extracted.manifest);
    let parsed: Document = serde_json::from_slice(&extracted.semantic).unwrap();
    let doc_report = validate_document(&parsed);

    assert!(manifest_report.is_valid(), "manifest: {:?}", manifest_report.errors);
    assert!(doc_report.is_valid(), "document: {:?}", doc_report.errors);
}

#[test]
fn validate_spdf_catches_bad_document() {
    let mut doc = sample_invoice_doc();
    doc.title = "".to_string(); // triggers E_001
    let spdf_bytes = build_spdf_container(&doc);

    let extracted = container::read_container(&spdf_bytes).unwrap();
    let parsed: Document = serde_json::from_slice(&extracted.semantic).unwrap();
    let doc_report = validate_document(&parsed);

    assert!(!doc_report.is_valid());
    assert!(doc_report.errors.iter().any(|e| e.code == "E_001"));
}

#[test]
fn validate_spdf_invalid_bytes() {
    let result = container::read_container(b"not a zip file");
    assert!(result.is_err());
}

#[test]
fn validate_spdf_combined_report_shape() {
    let doc = sample_invoice_doc();
    let spdf_bytes = build_spdf_container(&doc);

    let extracted = container::read_container(&spdf_bytes).unwrap();
    let manifest_report = validate_manifest(&extracted.manifest);
    let parsed: Document = serde_json::from_slice(&extracted.semantic).unwrap();
    let doc_report = validate_document(&parsed);

    let combined = serde_json::json!({
        "valid": manifest_report.is_valid() && doc_report.is_valid(),
        "manifest_errors": manifest_report.errors,
        "document_errors": doc_report.errors,
        "error_count": manifest_report.error_count() + doc_report.error_count(),
        "fatal_count": manifest_report.fatal_count() + doc_report.fatal_count(),
    });

    let json_str = serde_json::to_string(&combined).unwrap();
    let parsed_back: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    assert_eq!(parsed_back["valid"], true);
    assert_eq!(parsed_back["error_count"], 0);
    assert_eq!(parsed_back["fatal_count"], 0);
}

// ========================================================================
// generate_spdf path: parse semantic JSON → render PDF → build container
// ========================================================================

#[test]
fn generate_spdf_from_json_strings() {
    let doc = sample_invoice_doc();
    let semantic_json = serde_json::to_string_pretty(&doc).unwrap();
    let layout_json = r#"{"layout": "default"}"#;
    let styles_json = r#"{"styles": {}}"#;
    let metadata_json = r#"{"title": "Test"}"#;
    let audit_json = r#"{"entries": []}"#;

    // Parse semantic (same as PyO3 generate_spdf does)
    let parsed: Document = serde_json::from_str(&semantic_json).unwrap();
    let pdf_bytes = render_to_pdf(&parsed).unwrap();

    let mut manifest = Manifest::new(
        parsed.document_id.clone(),
        GeneratorInfo {
            name: "spdf-python".to_string(),
            version: "0.1.0".to_string(),
        },
    );

    let layers = ContainerLayers {
        semantic: semantic_json.as_bytes().to_vec(),
        layout: layout_json.as_bytes().to_vec(),
        styles: styles_json.as_bytes().to_vec(),
        render: pdf_bytes,
        metadata: metadata_json.as_bytes().to_vec(),
        audit: audit_json.as_bytes().to_vec(),
    };

    let container_bytes = container::write_container(&mut manifest, &layers, &[]).unwrap();
    assert!(!container_bytes.is_empty());

    // Verify round-trip: read it back
    let extracted = container::read_container(&container_bytes).unwrap();
    assert_eq!(extracted.manifest.format, "SPDF");
    let rt_doc: Document = serde_json::from_slice(&extracted.semantic).unwrap();
    assert_eq!(rt_doc.title, "Invoice INV-2026-200");
    assert_eq!(&extracted.render[..5], b"%PDF-");
}

#[test]
fn generate_spdf_bad_semantic_json_errors() {
    let bad_json = r#"{"not": "a document"}"#;
    let result = serde_json::from_str::<Document>(bad_json);
    assert!(result.is_err());
}

#[test]
fn generate_spdf_preserves_financial_values() {
    let doc = sample_invoice_doc();
    let semantic_json = serde_json::to_string(&doc).unwrap();

    let parsed: Document = serde_json::from_str(&semantic_json).unwrap();
    let pdf_bytes = render_to_pdf(&parsed).unwrap();

    let layers = ContainerLayers {
        semantic: semantic_json.as_bytes().to_vec(),
        layout: b"{}".to_vec(),
        styles: b"{}".to_vec(),
        render: pdf_bytes,
        metadata: b"{}".to_vec(),
        audit: b"{}".to_vec(),
    };

    let mut manifest = Manifest::new(parsed.document_id.clone(), GeneratorInfo {
        name: "test".to_string(),
        version: "0.1.0".to_string(),
    });

    let container_bytes = container::write_container(&mut manifest, &layers, &[]).unwrap();
    let extracted = container::read_container(&container_bytes).unwrap();
    let semantic_str = String::from_utf8_lossy(&extracted.semantic);

    assert!(semantic_str.contains(r#""132750.00"#));
    assert!(semantic_str.contains(r#""20250.00"#));
    assert!(semantic_str.contains(r#""125000.00"#));
}

// ========================================================================
// render_to_pdf path: read container → parse semantic → render to PDF
// ========================================================================

#[test]
fn render_to_pdf_from_container() {
    let doc = sample_invoice_doc();
    let spdf_bytes = build_spdf_container(&doc);

    let extracted = container::read_container(&spdf_bytes).unwrap();
    let parsed: Document = serde_json::from_slice(&extracted.semantic).unwrap();
    let pdf_bytes = render_to_pdf(&parsed).unwrap();

    assert!(pdf_bytes.len() > 100);
    assert_eq!(&pdf_bytes[..5], b"%PDF-");
}

#[test]
fn render_to_pdf_multi_page() {
    let doc = Document {
        version: SpdfVersion::V1_0,
        document_id: DocumentId::new(),
        title: "Multi-Page".to_string(),
        locale: "en-US".to_string(),
        direction: TextDirection::Ltr,
        document_state: DocumentState::Draft,
        pages: vec![
            Page {
                eid: eid(),
                page_number: 1,
                elements: vec![Element::Heading(HeadingElement {
                    eid: eid(),
                    level: 1,
                    text: "Page One".to_string(),
                    font_family: None,
                    font_size: None,
                    color: None,
                    timestamps: ts(),
                })],
            },
            Page {
                eid: eid(),
                page_number: 2,
                elements: vec![Element::Paragraph(ParagraphElement {
                    eid: eid(),
                    text: "Page Two content".to_string(),
                    font_family: None,
                    font_size: None,
                    color: None,
                    timestamps: ts(),
                })],
            },
        ],
    };

    let spdf_bytes = build_spdf_container(&doc);
    let extracted = container::read_container(&spdf_bytes).unwrap();
    let parsed: Document = serde_json::from_slice(&extracted.semantic).unwrap();
    let pdf_bytes = render_to_pdf(&parsed).unwrap();

    assert_eq!(&pdf_bytes[..5], b"%PDF-");
}

// ========================================================================
// parse_semantic path: JSON string → Document → re-serialize
// ========================================================================

#[test]
fn parse_semantic_round_trip() {
    let doc = sample_invoice_doc();
    let json_str = serde_json::to_string_pretty(&doc).unwrap();

    // Parse (same as PyO3 parse_semantic)
    let parsed: Document = serde_json::from_str(&json_str).unwrap();
    let re_serialized = serde_json::to_string_pretty(&parsed).unwrap();

    // Verify key fields survive
    let final_doc: Document = serde_json::from_str(&re_serialized).unwrap();
    assert_eq!(final_doc.title, "Invoice INV-2026-200");
    assert_eq!(final_doc.locale, "en-IN");
    assert_eq!(final_doc.pages.len(), 1);
    assert_eq!(final_doc.pages[0].elements.len(), 4);
}

#[test]
fn parse_semantic_rejects_invalid_json() {
    let result = serde_json::from_str::<Document>("totally not json {{{");
    assert!(result.is_err());
}

#[test]
fn parse_semantic_rejects_wrong_schema() {
    let result = serde_json::from_str::<Document>(r#"{"pages": "not an array"}"#);
    assert!(result.is_err());
}

// ========================================================================
// extract_invoice_data path: container → parse → find invoice elements
// ========================================================================

#[test]
fn extract_invoice_data_full() {
    let doc = sample_invoice_doc();
    let spdf_bytes = build_spdf_container(&doc);

    let extracted = container::read_container(&spdf_bytes).unwrap();
    let parsed: Document = serde_json::from_slice(&extracted.semantic).unwrap();

    let mut invoice_header = None;
    let mut line_item_table = None;
    let mut payment_terms = None;

    for page in &parsed.pages {
        for element in &page.elements {
            match element {
                Element::InvoiceHeader(ih) => invoice_header = Some(ih),
                Element::LineItemTable(lt) => line_item_table = Some(lt),
                Element::PaymentTerms(pt) => payment_terms = Some(pt),
                _ => {}
            }
        }
    }

    // InvoiceHeader fields
    let ih = invoice_header.expect("InvoiceHeader not found");
    assert_eq!(ih.invoice_number, "INV-2026-200");
    assert_eq!(ih.issue_date, "2026-03-25");
    assert_eq!(ih.due_date, "2026-04-25");
    assert_eq!(ih.vendor.name, "SPDF Corp");
    assert_eq!(ih.vendor.gstin.as_deref(), Some("29AABCU9603R1ZM"));
    assert_eq!(ih.client.name, "Acme Industries");
    assert_eq!(ih.currency.as_deref(), Some("INR"));

    // LineItemTable
    let lt = line_item_table.expect("LineItemTable not found");
    assert_eq!(lt.headers.len(), 4);
    assert_eq!(lt.rows.len(), 2);
    assert_eq!(lt.rows[0][3].value, "75000.00");
    assert_eq!(lt.rows[1][3].value, "50000.00");

    // PaymentTerms
    let pt = payment_terms.expect("PaymentTerms not found");
    assert_eq!(pt.subtotal, "125000.00");
    assert_eq!(pt.discount.as_deref(), Some("12500.00"));
    assert_eq!(pt.tax_label.as_deref(), Some("IGST 18%"));
    assert_eq!(pt.tax_amount.as_deref(), Some("20250.00"));
    assert_eq!(pt.total, "132750.00");
    assert_eq!(pt.payment_method.as_deref(), Some("NEFT"));
}

#[test]
fn extract_invoice_data_no_invoice_elements() {
    let doc = Document {
        version: SpdfVersion::V1_0,
        document_id: DocumentId::new(),
        title: "Plain Report".to_string(),
        locale: "en-US".to_string(),
        direction: TextDirection::Ltr,
        document_state: DocumentState::Draft,
        pages: vec![Page {
            eid: eid(),
            page_number: 1,
            elements: vec![Element::Paragraph(ParagraphElement {
                eid: eid(),
                text: "No invoice data here.".to_string(),
                font_family: None,
                font_size: None,
                color: None,
                timestamps: ts(),
            })],
        }],
    };

    let spdf_bytes = build_spdf_container(&doc);
    let extracted = container::read_container(&spdf_bytes).unwrap();
    let parsed: Document = serde_json::from_slice(&extracted.semantic).unwrap();

    let mut invoice_header = None;
    let mut line_item_table = None;
    let mut payment_terms = None;

    for page in &parsed.pages {
        for element in &page.elements {
            match element {
                Element::InvoiceHeader(ih) => invoice_header = Some(ih),
                Element::LineItemTable(lt) => line_item_table = Some(lt),
                Element::PaymentTerms(pt) => payment_terms = Some(pt),
                _ => {}
            }
        }
    }

    assert!(invoice_header.is_none());
    assert!(line_item_table.is_none());
    assert!(payment_terms.is_none());
}

#[test]
fn extract_invoice_data_json_output_shape() {
    let doc = sample_invoice_doc();
    let spdf_bytes = build_spdf_container(&doc);

    let extracted = container::read_container(&spdf_bytes).unwrap();
    let parsed: Document = serde_json::from_slice(&extracted.semantic).unwrap();

    let mut invoice_header = None;
    let mut line_item_table = None;
    let mut payment_terms = None;

    for page in &parsed.pages {
        for element in &page.elements {
            match element {
                Element::InvoiceHeader(ih) => invoice_header = Some(ih),
                Element::LineItemTable(lt) => line_item_table = Some(lt),
                Element::PaymentTerms(pt) => payment_terms = Some(pt),
                _ => {}
            }
        }
    }

    // Build the same JSON structure the PyO3 function builds
    let line_items: Vec<serde_json::Value> = line_item_table
        .map(|lt| {
            lt.rows
                .iter()
                .map(|row| {
                    let cells: Vec<serde_json::Value> = row
                        .iter()
                        .enumerate()
                        .map(|(i, cell)| {
                            let header = lt.headers.get(i).map(|h| h.as_str()).unwrap_or("unknown");
                            json!({"header": header, "value": cell.value, "type": cell.spdf_type})
                        })
                        .collect();
                    json!(cells)
                })
                .collect()
        })
        .unwrap_or_default();

    let result = json!({
        "invoice_number": invoice_header.map(|h| &h.invoice_number),
        "issue_date": invoice_header.map(|h| &h.issue_date),
        "due_date": invoice_header.map(|h| &h.due_date),
        "vendor": invoice_header.map(|h| json!({
            "name": h.vendor.name,
            "address": h.vendor.address,
            "gstin": h.vendor.gstin,
        })),
        "client": invoice_header.map(|h| json!({
            "name": h.client.name,
            "address": h.client.address,
            "gstin": h.client.gstin,
        })),
        "currency": invoice_header.and_then(|h| h.currency.as_ref()),
        "line_items": line_items,
        "subtotal": payment_terms.map(|p| &p.subtotal),
        "tax_label": payment_terms.and_then(|p| p.tax_label.as_ref()),
        "tax_amount": payment_terms.and_then(|p| p.tax_amount.as_ref()),
        "discount": payment_terms.and_then(|p| p.discount.as_ref()),
        "total": payment_terms.map(|p| &p.total),
        "payment_method": payment_terms.and_then(|p| p.payment_method.as_ref()),
    });

    let json_str = serde_json::to_string(&result).unwrap();
    let parsed_back: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed_back["invoice_number"], "INV-2026-200");
    assert_eq!(parsed_back["total"], "132750.00");
    assert_eq!(parsed_back["currency"], "INR");
    assert_eq!(parsed_back["vendor"]["name"], "SPDF Corp");
    assert_eq!(parsed_back["client"]["gstin"], "27AADCA1234B1ZK");
    assert_eq!(parsed_back["line_items"].as_array().unwrap().len(), 2);
    assert_eq!(parsed_back["tax_amount"], "20250.00");
    assert_eq!(parsed_back["discount"], "12500.00");
    assert_eq!(parsed_back["payment_method"], "NEFT");
}

// ========================================================================
// Regression: Phase 1 core paths still work end-to-end
// ========================================================================

#[test]
fn regression_container_checksum_integrity() {
    let doc = sample_invoice_doc();
    let spdf_bytes = build_spdf_container(&doc);

    let extracted = container::read_container(&spdf_bytes).unwrap();

    // All layer checksums must be 64-char hex (SHA-256)
    assert_eq!(extracted.manifest.layers.semantic.len(), 64);
    assert_eq!(extracted.manifest.layers.layout.len(), 64);
    assert_eq!(extracted.manifest.layers.styles.len(), 64);
    assert_eq!(extracted.manifest.layers.render.len(), 64);
    assert_eq!(extracted.manifest.layers.metadata.len(), 64);
    assert_eq!(extracted.manifest.layers.audit.len(), 64);
    assert_eq!(extracted.manifest.manifest_hash.len(), 64);
}

#[test]
fn regression_document_state_machine() {
    assert!(DocumentState::Draft.can_transition_to(&DocumentState::Review));
    assert!(DocumentState::Review.can_transition_to(&DocumentState::Signed));
    assert!(DocumentState::Signed.can_transition_to(&DocumentState::Certified));
    assert!(!DocumentState::Draft.can_transition_to(&DocumentState::Signed));
    assert!(!DocumentState::Certified.can_transition_to(&DocumentState::Draft));
}

#[test]
fn regression_all_element_types_serialize() {
    let elements: Vec<Element> = vec![
        Element::Heading(HeadingElement {
            eid: eid(),
            level: 1,
            text: "H1".into(),
            font_family: None,
            font_size: None,
            color: None,
            timestamps: ts(),
        }),
        Element::Paragraph(ParagraphElement {
            eid: eid(),
            text: "P".into(),
            font_family: None,
            font_size: None,
            color: None,
            timestamps: ts(),
        }),
        Element::Table(TableElement {
            eid: eid(),
            headers: vec!["A".into()],
            rows: vec![TableRow {
                cells: vec![TableCell {
                    value: "1".into(),
                    spdf_type: None,
                }],
            }],
            timestamps: ts(),
        }),
        Element::Image(ImageElement {
            eid: eid(),
            asset_id: "img-001".into(),
            alt_text: None,
            caption: None,
            width: 100.0,
            height: 100.0,
            timestamps: ts(),
        }),
        Element::CodeBlock(CodeBlockElement {
            eid: eid(),
            language: "rust".into(),
            code: "fn main() {}".into(),
            timestamps: ts(),
        }),
        Element::HorizontalRule(HorizontalRuleElement {
            eid: eid(),
            timestamps: ts(),
        }),
        Element::PageBreak(PageBreakElement { eid: eid() }),
        Element::InvoiceHeader(InvoiceHeaderElement {
            eid: eid(),
            invoice_number: "INV-001".into(),
            issue_date: "2026-01-01".into(),
            due_date: "2026-02-01".into(),
            vendor: PartyInfo {
                name: "V".into(),
                address: None,
                gstin: None,
            },
            client: PartyInfo {
                name: "C".into(),
                address: None,
                gstin: None,
            },
            currency: None,
            timestamps: ts(),
        }),
        Element::SignatureBlock(SignatureBlockElement {
            eid: eid(),
            signer_name: "Signer".into(),
            signer_title: None,
            signed_at: None,
            certificate_fingerprint: None,
            locked: false,
            timestamps: ts(),
        }),
        Element::FormField(FormFieldElement {
            eid: eid(),
            field_name: "name".into(),
            field_type: FormFieldType::Text,
            label: "Name".into(),
            required: true,
            default_value: None,
            options: None,
            timestamps: ts(),
        }),
    ];

    for el in &elements {
        let json = serde_json::to_string(el).unwrap();
        let _rt: Element = serde_json::from_str(&json).unwrap();
    }
    assert_eq!(elements.len(), 10);
}

#[test]
fn regression_validator_catches_all_error_codes() {
    // E_001: empty title
    let mut doc = sample_invoice_doc();
    doc.title = "".into();
    assert!(validate_document(&doc).errors.iter().any(|e| e.code == "E_001"));

    // E_002: empty locale
    let mut doc = sample_invoice_doc();
    doc.locale = "".into();
    assert!(validate_document(&doc).errors.iter().any(|e| e.code == "E_002"));

    // F_001: no pages
    let mut doc = sample_invoice_doc();
    doc.pages.clear();
    assert!(validate_document(&doc).errors.iter().any(|e| e.code == "F_001"));

    // F_002: empty page
    let mut doc = sample_invoice_doc();
    doc.pages[0].elements.clear();
    assert!(validate_document(&doc).errors.iter().any(|e| e.code == "F_002"));

    // E_006: heading level 0
    let mut doc = sample_invoice_doc();
    doc.pages[0].elements = vec![Element::Heading(HeadingElement {
        eid: eid(),
        level: 0,
        text: "Bad".into(),
        font_family: None,
        font_size: None,
        color: None,
        timestamps: ts(),
    })];
    assert!(validate_document(&doc).errors.iter().any(|e| e.code == "E_006"));
}

#[test]
fn regression_render_produces_valid_pdf() {
    let doc = sample_invoice_doc();
    let pdf = render_to_pdf(&doc).unwrap();

    assert_eq!(&pdf[..5], b"%PDF-");
    assert!(pdf.len() > 500, "PDF too small: {} bytes", pdf.len());
}
