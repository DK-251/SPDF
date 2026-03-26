use spdf_core::container::{write_container, ContainerLayers};
use spdf_core::diff::{diff_documents, ChangeType, SemanticImpact};
use spdf_core::dom::*;
use spdf_core::manifest::Manifest;
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

fn make_doc_with_elements(elements: Vec<Element>) -> Document {
    Document {
        version: SpdfVersion::V1_0,
        document_id: DocumentId::new(),
        title: "Test Document".to_string(),
        locale: "en-US".to_string(),
        direction: TextDirection::Ltr,
        document_state: DocumentState::Draft,
        pages: vec![Page {
            eid: eid(),
            page_number: 1,
            elements,
        }],
    }
}

// ---------- Identical documents ----------

#[test]
fn identical_documents_no_changes() {
    let shared_eid = eid();
    let doc = make_doc_with_elements(vec![Element::Heading(HeadingElement {
        eid: shared_eid.clone(),
        level: 1,
        text: "Hello".to_string(),
        font_family: None,
        font_size: None,
        color: None,
        timestamps: ts(),
    })]);

    // Must use same doc_id and same element for true identity
    let bytes = build_spdf(&doc);
    let report = diff_documents(&bytes, &bytes).unwrap();
    assert_eq!(report.summary.total_changes, 0);
    assert_eq!(report.summary.highest_impact, SemanticImpact::None);
}

// ---------- Metadata changes ----------

#[test]
fn title_change_detected() {
    let mut doc_a = make_doc_with_elements(vec![]);
    doc_a.title = "Original".to_string();
    let mut doc_b = doc_a.clone();
    doc_b.title = "Updated".to_string();

    let bytes_a = build_spdf(&doc_a);
    let bytes_b = build_spdf(&doc_b);
    let report = diff_documents(&bytes_a, &bytes_b).unwrap();

    assert_eq!(report.metadata_changes.len(), 1);
    assert_eq!(report.metadata_changes[0].field.as_deref(), Some("title"));
    assert_eq!(report.metadata_changes[0].impact, SemanticImpact::Moderate);
}

#[test]
fn state_change_is_critical() {
    let mut doc_a = make_doc_with_elements(vec![]);
    doc_a.document_state = DocumentState::Draft;
    let mut doc_b = doc_a.clone();
    doc_b.document_state = DocumentState::Review;

    let bytes_a = build_spdf(&doc_a);
    let bytes_b = build_spdf(&doc_b);
    let report = diff_documents(&bytes_a, &bytes_b).unwrap();

    assert_eq!(report.metadata_changes.len(), 1);
    assert_eq!(report.metadata_changes[0].impact, SemanticImpact::Critical);
}

// ---------- Element added/removed ----------

#[test]
fn element_added_detected() {
    let shared_eid = eid();
    let doc_a = make_doc_with_elements(vec![Element::Heading(HeadingElement {
        eid: shared_eid.clone(),
        level: 1,
        text: "Hello".to_string(),
        font_family: None,
        font_size: None,
        color: None,
        timestamps: ts(),
    })]);

    let new_eid = eid();
    let mut doc_b = doc_a.clone();
    doc_b.pages[0]
        .elements
        .push(Element::Paragraph(ParagraphElement {
            eid: new_eid,
            text: "New paragraph".to_string(),
            font_family: None,
            font_size: None,
            color: None,
            timestamps: ts(),
        }));

    let bytes_a = build_spdf(&doc_a);
    let bytes_b = build_spdf(&doc_b);
    let report = diff_documents(&bytes_a, &bytes_b).unwrap();

    assert_eq!(report.summary.added, 1);
    let added = report
        .element_changes
        .iter()
        .find(|c| c.change_type == ChangeType::Added)
        .unwrap();
    assert_eq!(added.element_type, "Paragraph");
}

#[test]
fn element_removed_detected() {
    let shared_eid = eid();
    let extra_eid = eid();
    let doc_a = make_doc_with_elements(vec![
        Element::Heading(HeadingElement {
            eid: shared_eid.clone(),
            level: 1,
            text: "Hello".to_string(),
            font_family: None,
            font_size: None,
            color: None,
            timestamps: ts(),
        }),
        Element::Paragraph(ParagraphElement {
            eid: extra_eid,
            text: "Will be removed".to_string(),
            font_family: None,
            font_size: None,
            color: None,
            timestamps: ts(),
        }),
    ]);

    let mut doc_b = doc_a.clone();
    doc_b.pages[0].elements.pop(); // Remove last element

    let bytes_a = build_spdf(&doc_a);
    let bytes_b = build_spdf(&doc_b);
    let report = diff_documents(&bytes_a, &bytes_b).unwrap();

    assert_eq!(report.summary.removed, 1);
}

// ---------- Element modified ----------

#[test]
fn element_text_modification_detected() {
    let shared_eid = eid();
    let doc_a = make_doc_with_elements(vec![Element::Heading(HeadingElement {
        eid: shared_eid.clone(),
        level: 1,
        text: "Original".to_string(),
        font_family: None,
        font_size: None,
        color: None,
        timestamps: ts(),
    })]);

    let mut doc_b = doc_a.clone();
    if let Element::Heading(h) = &mut doc_b.pages[0].elements[0] {
        h.text = "Modified".to_string();
    }

    let bytes_a = build_spdf(&doc_a);
    let bytes_b = build_spdf(&doc_b);
    let report = diff_documents(&bytes_a, &bytes_b).unwrap();

    assert!(report.summary.modified > 0);
    let text_change = report
        .element_changes
        .iter()
        .find(|c| c.field.as_deref() == Some("text"))
        .unwrap();
    assert_eq!(text_change.impact, SemanticImpact::Moderate);
}

#[test]
fn financial_field_change_is_major() {
    let shared_eid = eid();
    let doc_a = make_doc_with_elements(vec![Element::PaymentTerms(PaymentTermsElement {
        eid: shared_eid.clone(),
        subtotal: "1000.00".to_string(),
        discount: None,
        tax_label: None,
        tax_amount: None,
        total: "1000.00".to_string(),
        payment_method: None,
        timestamps: ts(),
    })]);

    let mut doc_b = doc_a.clone();
    if let Element::PaymentTerms(pt) = &mut doc_b.pages[0].elements[0] {
        pt.total = "2000.00".to_string();
    }

    let bytes_a = build_spdf(&doc_a);
    let bytes_b = build_spdf(&doc_b);
    let report = diff_documents(&bytes_a, &bytes_b).unwrap();

    let total_change = report
        .element_changes
        .iter()
        .find(|c| c.field.as_deref() == Some("total"))
        .unwrap();
    assert_eq!(total_change.impact, SemanticImpact::Major);
}

// ---------- Summary ----------

#[test]
fn summary_counts_correct() {
    let eid_keep = eid();
    let eid_remove = eid();
    let doc_a = make_doc_with_elements(vec![
        Element::Heading(HeadingElement {
            eid: eid_keep.clone(),
            level: 1,
            text: "Keep".to_string(),
            font_family: None,
            font_size: None,
            color: None,
            timestamps: ts(),
        }),
        Element::Paragraph(ParagraphElement {
            eid: eid_remove,
            text: "Remove me".to_string(),
            font_family: None,
            font_size: None,
            color: None,
            timestamps: ts(),
        }),
    ]);

    let eid_add = eid();
    let mut doc_b = doc_a.clone();
    doc_b.pages[0].elements.pop(); // Remove paragraph
    if let Element::Heading(h) = &mut doc_b.pages[0].elements[0] {
        h.text = "Modified".to_string(); // Modify heading
    }
    doc_b.pages[0]
        .elements
        .push(Element::CodeBlock(CodeBlockElement {
            eid: eid_add,
            language: "rust".to_string(),
            code: "fn main() {}".to_string(),
            timestamps: ts(),
        }));

    let bytes_a = build_spdf(&doc_a);
    let bytes_b = build_spdf(&doc_b);
    let report = diff_documents(&bytes_a, &bytes_b).unwrap();

    assert_eq!(report.summary.added, 1);
    assert_eq!(report.summary.removed, 1);
    assert!(report.summary.modified > 0);
    assert!(report.summary.total_changes > 0);
}

#[test]
fn highest_impact_reflects_worst_change() {
    let shared_eid = eid();
    let mut doc_a = make_doc_with_elements(vec![Element::Heading(HeadingElement {
        eid: shared_eid.clone(),
        level: 1,
        text: "Hello".to_string(),
        font_family: None,
        font_size: None,
        color: None,
        timestamps: ts(),
    })]);
    doc_a.document_state = DocumentState::Draft;

    let mut doc_b = doc_a.clone();
    doc_b.document_state = DocumentState::Review;

    let bytes_a = build_spdf(&doc_a);
    let bytes_b = build_spdf(&doc_b);
    let report = diff_documents(&bytes_a, &bytes_b).unwrap();

    assert_eq!(report.summary.highest_impact, SemanticImpact::Critical);
}

#[test]
fn locale_change_is_minor() {
    let mut doc_a = make_doc_with_elements(vec![]);
    doc_a.locale = "en-US".to_string();
    let mut doc_b = doc_a.clone();
    doc_b.locale = "en-IN".to_string();

    let bytes_a = build_spdf(&doc_a);
    let bytes_b = build_spdf(&doc_b);
    let report = diff_documents(&bytes_a, &bytes_b).unwrap();

    assert_eq!(report.metadata_changes[0].impact, SemanticImpact::Minor);
}

#[test]
fn direction_change_detected() {
    let mut doc_a = make_doc_with_elements(vec![]);
    doc_a.direction = TextDirection::Ltr;
    let mut doc_b = doc_a.clone();
    doc_b.direction = TextDirection::Rtl;

    let bytes_a = build_spdf(&doc_a);
    let bytes_b = build_spdf(&doc_b);
    let report = diff_documents(&bytes_a, &bytes_b).unwrap();

    assert_eq!(report.metadata_changes.len(), 1);
    assert_eq!(
        report.metadata_changes[0].field.as_deref(),
        Some("direction")
    );
}
