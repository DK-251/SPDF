//! Validation rules for SPDF documents and manifests.
//!
//! Error codes:
//! - F_001: Document has no pages
//! - F_002: Page has no elements
//! - F_003: Manifest format is not "SPDF"
//! - F_004: Manifest layer checksum is empty
//! - F_005: Manifest hash is empty
//! - E_001: Document title is empty
//! - E_002: Document locale is empty
//! - E_003: Duplicate element ID within document
//! - E_004: Page number is zero
//! - E_005: Page numbers are not sequential
//! - E_006: Heading level out of range (must be 1–6)
//! - E_007: Table has no headers
//! - E_008: Table row cell count does not match header count
//! - E_009: Invoice number is empty
//! - E_010: Payment total is empty
//! - E_011: Redaction references empty element ID
//! - E_012: FormField of type Select has no options
//! - E_013: VariablePlaceholder has empty variable_name

use std::collections::HashSet;

use spdf_core::dom::*;
use spdf_core::manifest::Manifest;
use spdf_core::types::ElementId;

use crate::{Severity, ValidationError};

fn fatal(code: &'static str, message: String, path: Option<String>) -> ValidationError {
    ValidationError { code, severity: Severity::Fatal, message, path }
}

fn error(code: &'static str, message: String, path: Option<String>) -> ValidationError {
    ValidationError { code, severity: Severity::Error, message, path }
}

pub fn run_document_rules(doc: &Document, errors: &mut Vec<ValidationError>) {
    check_document_metadata(doc, errors);
    check_pages(doc, errors);
    check_duplicate_eids(doc, errors);
}

pub fn run_manifest_rules(manifest: &Manifest, errors: &mut Vec<ValidationError>) {
    if manifest.format != "SPDF" {
        errors.push(fatal(
            "F_003",
            format!("manifest format must be 'SPDF', got '{}'", manifest.format),
            Some("manifest.format".to_string()),
        ));
    }

    let layer_names = [
        ("semantic", &manifest.layers.semantic),
        ("layout", &manifest.layers.layout),
        ("styles", &manifest.layers.styles),
        ("render", &manifest.layers.render),
        ("metadata", &manifest.layers.metadata),
        ("audit", &manifest.layers.audit),
    ];

    for (name, checksum) in layer_names {
        if checksum.is_empty() {
            errors.push(fatal(
                "F_004",
                format!("layer checksum for '{name}' is empty"),
                Some(format!("manifest.layers.{name}")),
            ));
        }
    }

    if manifest.manifest_hash.is_empty() {
        errors.push(fatal(
            "F_005",
            "manifest_hash is empty".to_string(),
            Some("manifest.manifest_hash".to_string()),
        ));
    }
}

fn check_document_metadata(doc: &Document, errors: &mut Vec<ValidationError>) {
    if doc.title.trim().is_empty() {
        errors.push(error(
            "E_001",
            "document title is empty".to_string(),
            Some("document.title".to_string()),
        ));
    }

    if doc.locale.trim().is_empty() {
        errors.push(error(
            "E_002",
            "document locale is empty".to_string(),
            Some("document.locale".to_string()),
        ));
    }

    if doc.pages.is_empty() {
        errors.push(fatal(
            "F_001",
            "document has no pages".to_string(),
            Some("document.pages".to_string()),
        ));
    }
}

fn check_pages(doc: &Document, errors: &mut Vec<ValidationError>) {
    let mut prev_page_num: Option<u32> = None;

    for (i, page) in doc.pages.iter().enumerate() {
        let page_path = format!("pages[{i}]");

        if page.page_number == 0 {
            errors.push(error(
                "E_004",
                format!("page number is zero at {page_path}"),
                Some(format!("{page_path}.page_number")),
            ));
        }

        if let Some(prev) = prev_page_num {
            if page.page_number != prev + 1 {
                errors.push(error(
                    "E_005",
                    format!(
                        "page numbers not sequential: expected {}, got {} at {page_path}",
                        prev + 1,
                        page.page_number
                    ),
                    Some(format!("{page_path}.page_number")),
                ));
            }
        }
        prev_page_num = Some(page.page_number);

        if page.elements.is_empty() {
            errors.push(fatal(
                "F_002",
                format!("page has no elements at {page_path}"),
                Some(format!("{page_path}.elements")),
            ));
        }

        for (j, element) in page.elements.iter().enumerate() {
            let elem_path = format!("{page_path}.elements[{j}]");
            check_element(element, &elem_path, errors);
        }
    }
}

fn check_element(element: &Element, path: &str, errors: &mut Vec<ValidationError>) {
    match element {
        Element::Heading(h) => {
            if h.level == 0 || h.level > 6 {
                errors.push(error(
                    "E_006",
                    format!("heading level must be 1–6, got {} at {path}", h.level),
                    Some(format!("{path}.level")),
                ));
            }
        }
        Element::Table(t) => {
            let row_slices: Vec<&[TableCell]> =
                t.rows.iter().map(|r| r.cells.as_slice()).collect();
            check_table_headers(&t.headers, &row_slices, path, errors);
        }
        Element::InvoiceHeader(ih) => {
            if ih.invoice_number.trim().is_empty() {
                errors.push(error(
                    "E_009",
                    format!("invoice_number is empty at {path}"),
                    Some(format!("{path}.invoice_number")),
                ));
            }
        }
        Element::LineItemTable(lt) => {
            let row_slices: Vec<&[TableCell]> =
                lt.rows.iter().map(|r| r.as_slice()).collect();
            check_table_headers(&lt.headers, &row_slices, path, errors);
        }
        Element::PaymentTerms(pt) => {
            if pt.total.trim().is_empty() {
                errors.push(error(
                    "E_010",
                    format!("payment total is empty at {path}"),
                    Some(format!("{path}.total")),
                ));
            }
        }
        Element::Redaction(r) => {
            if r.redacted_eid.0.trim().is_empty() {
                errors.push(error(
                    "E_011",
                    format!("redacted_eid is empty at {path}"),
                    Some(format!("{path}.redacted_eid")),
                ));
            }
        }
        Element::FormField(f) => {
            if matches!(f.field_type, FormFieldType::Select) {
                let has_options = f.options.as_ref().map_or(false, |o| !o.is_empty());
                if !has_options {
                    errors.push(error(
                        "E_012",
                        format!("Select field has no options at {path}"),
                        Some(format!("{path}.options")),
                    ));
                }
            }
        }
        Element::VariablePlaceholder(v) => {
            if v.variable_name.trim().is_empty() {
                errors.push(error(
                    "E_013",
                    format!("variable_name is empty at {path}"),
                    Some(format!("{path}.variable_name")),
                ));
            }
        }
        _ => {}
    }
}

fn check_table_headers(
    headers: &[String],
    rows: &[&[TableCell]],
    path: &str,
    errors: &mut Vec<ValidationError>,
) {
    if headers.is_empty() {
        errors.push(error(
            "E_007",
            format!("table has no headers at {path}"),
            Some(format!("{path}.headers")),
        ));
        return;
    }

    for (i, row) in rows.iter().enumerate() {
        if row.len() != headers.len() {
            errors.push(error(
                "E_008",
                format!(
                    "row {i} has {} cells but {} headers at {path}",
                    row.len(),
                    headers.len()
                ),
                Some(format!("{path}.rows[{i}]")),
            ));
        }
    }
}

fn check_duplicate_eids(doc: &Document, errors: &mut Vec<ValidationError>) {
    let mut seen = HashSet::new();
    for page in &doc.pages {
        check_eid_unique(&page.eid, &mut seen, errors);
        for element in &page.elements {
            let eid = element_eid(element);
            check_eid_unique(eid, &mut seen, errors);
        }
    }
}

fn check_eid_unique(
    eid: &ElementId,
    seen: &mut HashSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    if !seen.insert(eid.0.clone()) {
        errors.push(error(
            "E_003",
            format!("duplicate element ID: {}", eid.0),
            Some(eid.0.clone()),
        ));
    }
}

fn element_eid(element: &Element) -> &ElementId {
    match element {
        Element::Heading(e) => &e.eid,
        Element::Paragraph(e) => &e.eid,
        Element::Table(e) => &e.eid,
        Element::Image(e) => &e.eid,
        Element::VectorImage(e) => &e.eid,
        Element::CodeBlock(e) => &e.eid,
        Element::HorizontalRule(e) => &e.eid,
        Element::PageBreak(e) => &e.eid,
        Element::Attachment(e) => &e.eid,
        Element::InvoiceHeader(e) => &e.eid,
        Element::LineItemTable(e) => &e.eid,
        Element::PaymentTerms(e) => &e.eid,
        Element::SignatureBlock(e) => &e.eid,
        Element::Stamp(e) => &e.eid,
        Element::Annotation(e) => &e.eid,
        Element::Redaction(e) => &e.eid,
        Element::FormField(e) => &e.eid,
        Element::VariablePlaceholder(e) => &e.eid,
    }
}
