use spdf_core::dom::*;
use spdf_core::types::*;

fn ts() -> Timestamps {
    Timestamps::now()
}

fn eid() -> ElementId {
    ElementId::new()
}

// ---------- Document round-trip ----------

#[test]
fn document_serialize_deserialize() {
    let doc = Document {
        version: SpdfVersion::V1_0,
        document_id: DocumentId::new(),
        title: "Test Invoice".to_string(),
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
                    text: "Invoice #001".to_string(),
                    font_family: None,
                    font_size: None,
                    color: None,
                    timestamps: ts(),
                }),
                Element::Paragraph(ParagraphElement {
                    eid: eid(),
                    text: "Thank you for your business.".to_string(),
                    font_family: Some("Inter".to_string()),
                    font_size: Some(12.0),
                    color: Some("#333333".to_string()),
                    timestamps: ts(),
                }),
            ],
        }],
    };

    let json = serde_json::to_string_pretty(&doc).unwrap();
    let roundtrip: Document = serde_json::from_str(&json).unwrap();

    assert_eq!(roundtrip.title, "Test Invoice");
    assert_eq!(roundtrip.pages.len(), 1);
    assert_eq!(roundtrip.pages[0].elements.len(), 2);
}

// ---------- Content elements ----------

#[test]
fn heading_element_serde() {
    let el = Element::Heading(HeadingElement {
        eid: eid(),
        level: 2,
        text: "Section Title".to_string(),
        font_family: Some("Roboto".to_string()),
        font_size: Some(18.0),
        color: Some("#000000".to_string()),
        timestamps: ts(),
    });
    let json = serde_json::to_string(&el).unwrap();
    assert!(json.contains(r#""element_type":"Heading"#));
    let _: Element = serde_json::from_str(&json).unwrap();
}

#[test]
fn paragraph_element_serde() {
    let el = Element::Paragraph(ParagraphElement {
        eid: eid(),
        text: "Body text here.".to_string(),
        font_family: None,
        font_size: None,
        color: None,
        timestamps: ts(),
    });
    let json = serde_json::to_string(&el).unwrap();
    assert!(json.contains(r#""element_type":"Paragraph"#));
    let rt: Element = serde_json::from_str(&json).unwrap();
    if let Element::Paragraph(p) = rt {
        assert_eq!(p.text, "Body text here.");
        assert!(p.font_family.is_none());
    } else {
        panic!("expected Paragraph");
    }
}

#[test]
fn table_element_serde() {
    let el = Element::Table(TableElement {
        eid: eid(),
        headers: vec!["Name".to_string(), "Amount".to_string()],
        rows: vec![TableRow {
            cells: vec![
                TableCell { value: "Item A".to_string(), spdf_type: None },
                TableCell { value: "2500.00".to_string(), spdf_type: Some("currency".to_string()) },
            ],
        }],
        timestamps: ts(),
    });
    let json = serde_json::to_string(&el).unwrap();
    assert!(json.contains(r#""spdf:type":"currency"#));
    let _: Element = serde_json::from_str(&json).unwrap();
}

#[test]
fn image_element_serde() {
    let el = Element::Image(ImageElement {
        eid: eid(),
        asset_id: "asset-001".to_string(),
        alt_text: Some("Company logo".to_string()),
        caption: Some("Logo".to_string()),
        width: 200.0,
        height: 100.0,
        timestamps: ts(),
    });
    let json = serde_json::to_string(&el).unwrap();
    let _: Element = serde_json::from_str(&json).unwrap();
}

#[test]
fn vector_image_element_serde() {
    let el = Element::VectorImage(VectorImageElement {
        eid: eid(),
        asset_id: "asset-svg-001".to_string(),
        alt_text: Some("Diagram".to_string()),
        width: 400.0,
        height: 300.0,
        timestamps: ts(),
    });
    let json = serde_json::to_string(&el).unwrap();
    assert!(json.contains(r#""element_type":"VectorImage"#));
    let _: Element = serde_json::from_str(&json).unwrap();
}

#[test]
fn code_block_element_serde() {
    let el = Element::CodeBlock(CodeBlockElement {
        eid: eid(),
        language: "rust".to_string(),
        code: "fn main() {}".to_string(),
        timestamps: ts(),
    });
    let json = serde_json::to_string(&el).unwrap();
    let _: Element = serde_json::from_str(&json).unwrap();
}

#[test]
fn horizontal_rule_element_serde() {
    let el = Element::HorizontalRule(HorizontalRuleElement {
        eid: eid(),
        timestamps: ts(),
    });
    let json = serde_json::to_string(&el).unwrap();
    assert!(json.contains(r#""element_type":"HorizontalRule"#));
    let _: Element = serde_json::from_str(&json).unwrap();
}

#[test]
fn page_break_element_serde() {
    let el = Element::PageBreak(PageBreakElement {
        eid: eid(),
    });
    let json = serde_json::to_string(&el).unwrap();
    let _: Element = serde_json::from_str(&json).unwrap();
}

#[test]
fn attachment_element_serde() {
    let el = Element::Attachment(AttachmentElement {
        eid: eid(),
        asset_id: "asset-att-001".to_string(),
        file_name: "contract.pdf".to_string(),
        mime_type: "application/pdf".to_string(),
        timestamps: ts(),
    });
    let json = serde_json::to_string(&el).unwrap();
    let _: Element = serde_json::from_str(&json).unwrap();
}

// ---------- Domain-specific elements ----------

#[test]
fn invoice_header_element_serde() {
    let el = Element::InvoiceHeader(InvoiceHeaderElement {
        eid: eid(),
        invoice_number: "INV-2026-001".to_string(),
        issue_date: "2026-03-25".to_string(),
        due_date: "2026-04-25".to_string(),
        vendor: PartyInfo {
            name: "SPDF Corp".to_string(),
            address: Some("Bangalore, India".to_string()),
            gstin: Some("29AABCU9603R1ZM".to_string()),
        },
        client: PartyInfo {
            name: "Client LLC".to_string(),
            address: None,
            gstin: None,
        },
        currency: Some("INR".to_string()),
        timestamps: ts(),
    });
    let json = serde_json::to_string(&el).unwrap();
    assert!(json.contains("INV-2026-001"));
    assert!(json.contains("29AABCU9603R1ZM"));
    let rt: Element = serde_json::from_str(&json).unwrap();
    if let Element::InvoiceHeader(h) = rt {
        assert_eq!(h.vendor.gstin.as_deref(), Some("29AABCU9603R1ZM"));
    } else {
        panic!("expected InvoiceHeader");
    }
}

#[test]
fn line_item_table_element_serde() {
    let el = Element::LineItemTable(LineItemTableElement {
        eid: eid(),
        headers: vec!["Description".into(), "Qty".into(), "Rate".into(), "Amount".into()],
        rows: vec![vec![
            TableCell { value: "Consulting".to_string(), spdf_type: None },
            TableCell { value: "10".to_string(), spdf_type: Some("integer".to_string()) },
            TableCell { value: "5000.00".to_string(), spdf_type: Some("currency".to_string()) },
            TableCell { value: "50000.00".to_string(), spdf_type: Some("currency".to_string()) },
        ]],
        timestamps: ts(),
    });
    let json = serde_json::to_string(&el).unwrap();
    // Financial values as strings, not floats
    assert!(json.contains(r#""50000.00"#));
    let _: Element = serde_json::from_str(&json).unwrap();
}

#[test]
fn payment_terms_element_serde() {
    let el = Element::PaymentTerms(PaymentTermsElement {
        eid: eid(),
        subtotal: "50000.00".to_string(),
        discount: Some("5000.00".to_string()),
        tax_label: Some("GST 18%".to_string()),
        tax_amount: Some("8100.00".to_string()),
        total: "53100.00".to_string(),
        payment_method: Some("NEFT".to_string()),
        timestamps: ts(),
    });
    let json = serde_json::to_string(&el).unwrap();
    // Amounts must remain as strings
    assert!(json.contains(r#""53100.00"#));
    assert!(!json.contains("53100.0,")); // no float coercion
    let _: Element = serde_json::from_str(&json).unwrap();
}

// ---------- Trust elements ----------

#[test]
fn signature_block_element_serde() {
    let el = Element::SignatureBlock(SignatureBlockElement {
        eid: eid(),
        signer_name: "Deepak Sahu".to_string(),
        signer_title: Some("Founder".to_string()),
        signed_at: Some("2026-03-25T10:00:00Z".to_string()),
        certificate_fingerprint: Some("SHA256:abc123".to_string()),
        locked: true,
        timestamps: ts(),
    });
    let json = serde_json::to_string(&el).unwrap();
    assert!(json.contains(r#""locked":true"#));
    let _: Element = serde_json::from_str(&json).unwrap();
}

#[test]
fn stamp_element_serde() {
    let el = Element::Stamp(StampElement {
        eid: eid(),
        stamp_type: "company_seal".to_string(),
        asset_id: "asset-stamp-001".to_string(),
        timestamps: ts(),
    });
    let json = serde_json::to_string(&el).unwrap();
    let _: Element = serde_json::from_str(&json).unwrap();
}

#[test]
fn annotation_element_serde() {
    let target = eid();
    let el = Element::Annotation(AnnotationElement {
        eid: eid(),
        author: "reviewer@example.com".to_string(),
        content: "Please verify this amount.".to_string(),
        target_eid: Some(target.clone()),
        timestamps: ts(),
    });
    let json = serde_json::to_string(&el).unwrap();
    assert!(json.contains(&target.0));
    let _: Element = serde_json::from_str(&json).unwrap();
}

#[test]
fn annotation_without_target_serde() {
    let el = Element::Annotation(AnnotationElement {
        eid: eid(),
        author: "admin".to_string(),
        content: "General note".to_string(),
        target_eid: None,
        timestamps: ts(),
    });
    let json = serde_json::to_string(&el).unwrap();
    assert!(!json.contains("target_eid"));
    let _: Element = serde_json::from_str(&json).unwrap();
}

#[test]
fn redaction_element_serde() {
    let redacted = eid();
    let el = Element::Redaction(RedactionElement {
        eid: eid(),
        redacted_eid: redacted,
        reason: "PII removal".to_string(),
        erasure_proof_hash: "sha256:deadbeef".to_string(),
        timestamps: ts(),
    });
    let json = serde_json::to_string(&el).unwrap();
    assert!(json.contains("PII removal"));
    let _: Element = serde_json::from_str(&json).unwrap();
}

// ---------- Interactive elements ----------

#[test]
fn form_field_text_serde() {
    let el = Element::FormField(FormFieldElement {
        eid: eid(),
        field_name: "company_name".to_string(),
        field_type: FormFieldType::Text,
        label: "Company Name".to_string(),
        required: true,
        default_value: None,
        options: None,
        timestamps: ts(),
    });
    let json = serde_json::to_string(&el).unwrap();
    assert!(json.contains(r#""required":true"#));
    let _: Element = serde_json::from_str(&json).unwrap();
}

#[test]
fn form_field_select_with_options_serde() {
    let el = Element::FormField(FormFieldElement {
        eid: eid(),
        field_name: "payment_method".to_string(),
        field_type: FormFieldType::Select,
        label: "Payment Method".to_string(),
        required: true,
        default_value: Some("NEFT".to_string()),
        options: Some(vec!["NEFT".into(), "UPI".into(), "RTGS".into()]),
        timestamps: ts(),
    });
    let json = serde_json::to_string(&el).unwrap();
    assert!(json.contains("NEFT"));
    let rt: Element = serde_json::from_str(&json).unwrap();
    if let Element::FormField(f) = rt {
        assert_eq!(f.options.as_ref().unwrap().len(), 3);
    } else {
        panic!("expected FormField");
    }
}

#[test]
fn form_field_all_types_serde() {
    let types = vec![
        FormFieldType::Text,
        FormFieldType::Textarea,
        FormFieldType::Select,
        FormFieldType::Checkbox,
        FormFieldType::Date,
        FormFieldType::Number,
    ];
    for ft in types {
        let el = Element::FormField(FormFieldElement {
            eid: eid(),
            field_name: "test".to_string(),
            field_type: ft,
            label: "Test".to_string(),
            required: false,
            default_value: None,
            options: None,
            timestamps: ts(),
        });
        let json = serde_json::to_string(&el).unwrap();
        let _: Element = serde_json::from_str(&json).unwrap();
    }
}

#[test]
fn variable_placeholder_element_serde() {
    let el = Element::VariablePlaceholder(VariablePlaceholderElement {
        eid: eid(),
        variable_name: "client_name".to_string(),
        default_value: Some("Acme Corp".to_string()),
        timestamps: ts(),
    });
    let json = serde_json::to_string(&el).unwrap();
    assert!(json.contains("client_name"));
    let _: Element = serde_json::from_str(&json).unwrap();
}

// ---------- Type-level tests ----------

#[test]
fn document_state_transitions() {
    assert!(DocumentState::Draft.can_transition_to(&DocumentState::Review));
    assert!(DocumentState::Review.can_transition_to(&DocumentState::Draft));
    assert!(DocumentState::Review.can_transition_to(&DocumentState::Signed));
    assert!(DocumentState::Signed.can_transition_to(&DocumentState::Certified));

    assert!(!DocumentState::Draft.can_transition_to(&DocumentState::Signed));
    assert!(!DocumentState::Draft.can_transition_to(&DocumentState::Certified));
    assert!(!DocumentState::Signed.can_transition_to(&DocumentState::Draft));
    assert!(!DocumentState::Signed.can_transition_to(&DocumentState::Review));
    assert!(!DocumentState::Certified.can_transition_to(&DocumentState::Draft));
    assert!(!DocumentState::Certified.can_transition_to(&DocumentState::Signed));
}

#[test]
fn document_state_serde_screaming_snake() {
    let json = serde_json::to_string(&DocumentState::Draft).unwrap();
    assert_eq!(json, r#""DRAFT""#);
    let json = serde_json::to_string(&DocumentState::Signed).unwrap();
    assert_eq!(json, r#""SIGNED""#);
}

#[test]
fn element_id_format() {
    let id = ElementId::new();
    assert!(id.0.starts_with("el-"), "ElementId should start with el-: {}", id.0);
}

#[test]
fn document_id_format() {
    let id = DocumentId::new();
    assert!(id.0.starts_with("spdf-"), "DocumentId should start with spdf-: {}", id.0);
}

#[test]
fn spdf_version_display() {
    assert_eq!(SpdfVersion::V1_0.to_string(), "1.0");
}

#[test]
fn text_direction_serde() {
    assert_eq!(serde_json::to_string(&TextDirection::Ltr).unwrap(), r#""LTR""#);
    assert_eq!(serde_json::to_string(&TextDirection::Rtl).unwrap(), r#""RTL""#);
}

#[test]
fn source_format_serde() {
    assert_eq!(serde_json::to_string(&SourceFormat::Native).unwrap(), r#""NATIVE""#);
    assert_eq!(serde_json::to_string(&SourceFormat::ConvertedFromPdf).unwrap(), r#""CONVERTED_FROM_PDF""#);
    assert_eq!(serde_json::to_string(&SourceFormat::Template).unwrap(), r#""TEMPLATE""#);
}

// ---------- Multi-page document ----------

#[test]
fn multi_page_document_serde() {
    let doc = Document {
        version: SpdfVersion::V1_0,
        document_id: DocumentId::new(),
        title: "Multi-page".to_string(),
        locale: "en-US".to_string(),
        direction: TextDirection::Ltr,
        document_state: DocumentState::Review,
        pages: vec![
            Page {
                eid: eid(),
                page_number: 1,
                elements: vec![Element::Heading(HeadingElement {
                    eid: eid(),
                    level: 1,
                    text: "Page 1".to_string(),
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
                    text: "Page 2 content".to_string(),
                    font_family: None,
                    font_size: None,
                    color: None,
                    timestamps: ts(),
                })],
            },
        ],
    };

    let json = serde_json::to_string_pretty(&doc).unwrap();
    let rt: Document = serde_json::from_str(&json).unwrap();
    assert_eq!(rt.pages.len(), 2);
    assert_eq!(rt.document_state, DocumentState::Review);
}

// ---------- Full invoice document ----------

#[test]
fn full_invoice_document_serde() {
    let doc = Document {
        version: SpdfVersion::V1_0,
        document_id: DocumentId::new(),
        title: "Invoice INV-2026-042".to_string(),
        locale: "en-IN".to_string(),
        direction: TextDirection::Ltr,
        document_state: DocumentState::Draft,
        pages: vec![Page {
            eid: eid(),
            page_number: 1,
            elements: vec![
                Element::InvoiceHeader(InvoiceHeaderElement {
                    eid: eid(),
                    invoice_number: "INV-2026-042".to_string(),
                    issue_date: "2026-03-25".to_string(),
                    due_date: "2026-04-25".to_string(),
                    vendor: PartyInfo {
                        name: "SPDF Corp".to_string(),
                        address: Some("HSR Layout, Bangalore".to_string()),
                        gstin: Some("29AABCU9603R1ZM".to_string()),
                    },
                    client: PartyInfo {
                        name: "Acme Industries".to_string(),
                        address: Some("MG Road, Mumbai".to_string()),
                        gstin: Some("27AADCA1234B1ZK".to_string()),
                    },
                    currency: Some("INR".to_string()),
                    timestamps: ts(),
                }),
                Element::LineItemTable(LineItemTableElement {
                    eid: eid(),
                    headers: vec!["Item".into(), "Qty".into(), "Rate".into(), "Amount".into()],
                    rows: vec![
                        vec![
                            TableCell { value: "API Integration".into(), spdf_type: None },
                            TableCell { value: "1".into(), spdf_type: Some("integer".into()) },
                            TableCell { value: "75000.00".into(), spdf_type: Some("currency".into()) },
                            TableCell { value: "75000.00".into(), spdf_type: Some("currency".into()) },
                        ],
                    ],
                    timestamps: ts(),
                }),
                Element::PaymentTerms(PaymentTermsElement {
                    eid: eid(),
                    subtotal: "75000.00".to_string(),
                    discount: None,
                    tax_label: Some("IGST 18%".to_string()),
                    tax_amount: Some("13500.00".to_string()),
                    total: "88500.00".to_string(),
                    payment_method: Some("NEFT".to_string()),
                    timestamps: ts(),
                }),
                Element::SignatureBlock(SignatureBlockElement {
                    eid: eid(),
                    signer_name: "Deepak Sahu".to_string(),
                    signer_title: Some("Director".to_string()),
                    signed_at: None,
                    certificate_fingerprint: None,
                    locked: false,
                    timestamps: ts(),
                }),
            ],
        }],
    };

    let json = serde_json::to_string_pretty(&doc).unwrap();
    let rt: Document = serde_json::from_str(&json).unwrap();

    assert_eq!(rt.title, "Invoice INV-2026-042");
    assert_eq!(rt.pages[0].elements.len(), 4);

    // Verify financial values survived as strings
    assert!(json.contains(r#""88500.00"#));
    assert!(json.contains(r#""13500.00"#));
}
