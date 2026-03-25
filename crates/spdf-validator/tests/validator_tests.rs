use spdf_core::dom::*;
use spdf_core::manifest::Manifest;
use spdf_core::types::*;
use spdf_validator::{validate_document, validate_manifest, Severity};

fn ts() -> Timestamps {
    Timestamps::now()
}

fn eid() -> ElementId {
    ElementId::new()
}

fn valid_doc() -> Document {
    Document {
        version: SpdfVersion::V1_0,
        document_id: DocumentId::new(),
        title: "Test Doc".to_string(),
        locale: "en-US".to_string(),
        direction: TextDirection::Ltr,
        document_state: DocumentState::Draft,
        pages: vec![Page {
            eid: eid(),
            page_number: 1,
            elements: vec![Element::Paragraph(ParagraphElement {
                eid: eid(),
                text: "Hello".to_string(),
                font_family: None,
                font_size: None,
                color: None,
                timestamps: ts(),
            })],
        }],
    }
}

// ---------- Valid document ----------

#[test]
fn valid_document_passes() {
    let report = validate_document(&valid_doc());
    assert!(report.is_valid(), "errors: {:?}", report.errors);
}

// ---------- Fatal errors ----------

#[test]
fn f001_no_pages() {
    let mut doc = valid_doc();
    doc.pages.clear();
    let report = validate_document(&doc);
    assert!(report.has_fatal());
    assert!(report.errors.iter().any(|e| e.code == "F_001"));
}

#[test]
fn f002_empty_page() {
    let mut doc = valid_doc();
    doc.pages[0].elements.clear();
    let report = validate_document(&doc);
    assert!(report.has_fatal());
    assert!(report.errors.iter().any(|e| e.code == "F_002"));
}

// ---------- Document metadata errors ----------

#[test]
fn e001_empty_title() {
    let mut doc = valid_doc();
    doc.title = "".to_string();
    let report = validate_document(&doc);
    assert!(report.errors.iter().any(|e| e.code == "E_001"));
}

#[test]
fn e001_whitespace_title() {
    let mut doc = valid_doc();
    doc.title = "   ".to_string();
    let report = validate_document(&doc);
    assert!(report.errors.iter().any(|e| e.code == "E_001"));
}

#[test]
fn e002_empty_locale() {
    let mut doc = valid_doc();
    doc.locale = "".to_string();
    let report = validate_document(&doc);
    assert!(report.errors.iter().any(|e| e.code == "E_002"));
}

// ---------- Page errors ----------

#[test]
fn e004_page_number_zero() {
    let mut doc = valid_doc();
    doc.pages[0].page_number = 0;
    let report = validate_document(&doc);
    assert!(report.errors.iter().any(|e| e.code == "E_004"));
}

#[test]
fn e005_non_sequential_pages() {
    let mut doc = valid_doc();
    doc.pages.push(Page {
        eid: eid(),
        page_number: 5, // should be 2
        elements: vec![Element::Paragraph(ParagraphElement {
            eid: eid(),
            text: "Page 5?".to_string(),
            font_family: None,
            font_size: None,
            color: None,
            timestamps: ts(),
        })],
    });
    let report = validate_document(&doc);
    assert!(report.errors.iter().any(|e| e.code == "E_005"));
}

// ---------- Element errors ----------

#[test]
fn e006_heading_level_zero() {
    let mut doc = valid_doc();
    doc.pages[0].elements = vec![Element::Heading(HeadingElement {
        eid: eid(),
        level: 0,
        text: "Bad".to_string(),
        font_family: None,
        font_size: None,
        color: None,
        timestamps: ts(),
    })];
    let report = validate_document(&doc);
    assert!(report.errors.iter().any(|e| e.code == "E_006"));
}

#[test]
fn e006_heading_level_seven() {
    let mut doc = valid_doc();
    doc.pages[0].elements = vec![Element::Heading(HeadingElement {
        eid: eid(),
        level: 7,
        text: "Bad".to_string(),
        font_family: None,
        font_size: None,
        color: None,
        timestamps: ts(),
    })];
    let report = validate_document(&doc);
    assert!(report.errors.iter().any(|e| e.code == "E_006"));
}

#[test]
fn e007_table_no_headers() {
    let mut doc = valid_doc();
    doc.pages[0].elements = vec![Element::Table(TableElement {
        eid: eid(),
        headers: vec![],
        rows: vec![],
        timestamps: ts(),
    })];
    let report = validate_document(&doc);
    assert!(report.errors.iter().any(|e| e.code == "E_007"));
}

#[test]
fn e008_table_row_cell_mismatch() {
    let mut doc = valid_doc();
    doc.pages[0].elements = vec![Element::Table(TableElement {
        eid: eid(),
        headers: vec!["A".into(), "B".into()],
        rows: vec![TableRow {
            cells: vec![TableCell { value: "only one".into(), spdf_type: None }],
        }],
        timestamps: ts(),
    })];
    let report = validate_document(&doc);
    assert!(report.errors.iter().any(|e| e.code == "E_008"));
}

#[test]
fn e009_empty_invoice_number() {
    let mut doc = valid_doc();
    doc.pages[0].elements = vec![Element::InvoiceHeader(InvoiceHeaderElement {
        eid: eid(),
        invoice_number: "".to_string(),
        issue_date: "2026-01-01".to_string(),
        due_date: "2026-02-01".to_string(),
        vendor: PartyInfo { name: "V".into(), address: None, gstin: None },
        client: PartyInfo { name: "C".into(), address: None, gstin: None },
        currency: None,
        timestamps: ts(),
    })];
    let report = validate_document(&doc);
    assert!(report.errors.iter().any(|e| e.code == "E_009"));
}

#[test]
fn e010_empty_payment_total() {
    let mut doc = valid_doc();
    doc.pages[0].elements = vec![Element::PaymentTerms(PaymentTermsElement {
        eid: eid(),
        subtotal: "100.00".to_string(),
        discount: None,
        tax_label: None,
        tax_amount: None,
        total: "".to_string(),
        payment_method: None,
        timestamps: ts(),
    })];
    let report = validate_document(&doc);
    assert!(report.errors.iter().any(|e| e.code == "E_010"));
}

#[test]
fn e012_select_without_options() {
    let mut doc = valid_doc();
    doc.pages[0].elements = vec![Element::FormField(FormFieldElement {
        eid: eid(),
        field_name: "choice".to_string(),
        field_type: FormFieldType::Select,
        label: "Pick one".to_string(),
        required: true,
        default_value: None,
        options: None,
        timestamps: ts(),
    })];
    let report = validate_document(&doc);
    assert!(report.errors.iter().any(|e| e.code == "E_012"));
}

#[test]
fn e013_empty_variable_name() {
    let mut doc = valid_doc();
    doc.pages[0].elements = vec![Element::VariablePlaceholder(VariablePlaceholderElement {
        eid: eid(),
        variable_name: "".to_string(),
        default_value: None,
        timestamps: ts(),
    })];
    let report = validate_document(&doc);
    assert!(report.errors.iter().any(|e| e.code == "E_013"));
}

// ---------- Duplicate EID ----------

#[test]
fn e003_duplicate_element_id() {
    let shared_eid = eid();
    let mut doc = valid_doc();
    doc.pages[0].elements = vec![
        Element::Paragraph(ParagraphElement {
            eid: shared_eid.clone(),
            text: "First".to_string(),
            font_family: None,
            font_size: None,
            color: None,
            timestamps: ts(),
        }),
        Element::Paragraph(ParagraphElement {
            eid: shared_eid,
            text: "Duplicate".to_string(),
            font_family: None,
            font_size: None,
            color: None,
            timestamps: ts(),
        }),
    ];
    let report = validate_document(&doc);
    assert!(report.errors.iter().any(|e| e.code == "E_003"));
}

// ---------- Manifest validation ----------

#[test]
fn valid_manifest_passes() {
    let mut manifest = Manifest::new(DocumentId::new(), GeneratorInfo {
        name: "test".to_string(),
        version: "0.1.0".to_string(),
    });
    manifest.layers.semantic = "a".repeat(64);
    manifest.layers.layout = "b".repeat(64);
    manifest.layers.styles = "c".repeat(64);
    manifest.layers.render = "d".repeat(64);
    manifest.layers.metadata = "e".repeat(64);
    manifest.layers.audit = "f".repeat(64);
    manifest.finalize();

    let report = validate_manifest(&manifest);
    assert!(report.is_valid(), "errors: {:?}", report.errors);
}

#[test]
fn f003_wrong_manifest_format() {
    let mut manifest = Manifest::new(DocumentId::new(), GeneratorInfo {
        name: "test".to_string(),
        version: "0.1.0".to_string(),
    });
    manifest.format = "PDF".to_string();
    manifest.finalize();

    let report = validate_manifest(&manifest);
    assert!(report.errors.iter().any(|e| e.code == "F_003"));
}

#[test]
fn f004_empty_layer_checksum() {
    let manifest = Manifest::new(DocumentId::new(), GeneratorInfo {
        name: "test".to_string(),
        version: "0.1.0".to_string(),
    });
    // All layer checksums are empty by default (no finalize)
    let report = validate_manifest(&manifest);
    assert!(report.errors.iter().any(|e| e.code == "F_004"));
    assert_eq!(report.errors.iter().filter(|e| e.code == "F_004").count(), 6);
}

#[test]
fn f005_empty_manifest_hash() {
    let mut manifest = Manifest::new(DocumentId::new(), GeneratorInfo {
        name: "test".to_string(),
        version: "0.1.0".to_string(),
    });
    manifest.layers.semantic = "a".repeat(64);
    manifest.layers.layout = "b".repeat(64);
    manifest.layers.styles = "c".repeat(64);
    manifest.layers.render = "d".repeat(64);
    manifest.layers.metadata = "e".repeat(64);
    manifest.layers.audit = "f".repeat(64);
    // Don't finalize — manifest_hash stays empty

    let report = validate_manifest(&manifest);
    assert!(report.errors.iter().any(|e| e.code == "F_005"));
}

// ---------- Report helpers ----------

#[test]
fn report_counts() {
    let mut doc = valid_doc();
    doc.title = "".to_string(); // E_001
    doc.locale = "".to_string(); // E_002
    doc.pages.clear(); // F_001

    let report = validate_document(&doc);
    assert_eq!(report.error_count(), 2);
    assert_eq!(report.fatal_count(), 1);
    assert!(!report.is_valid());
    assert!(report.has_fatal());
}
