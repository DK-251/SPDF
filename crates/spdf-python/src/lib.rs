//! PyO3 bindings exposing SPDF core engine to Python (FastAPI backend).
//!
//! Module name: `spdf_native`
//! Functions: validate_spdf, generate_spdf, render_to_pdf, parse_semantic, extract_invoice_data
#![allow(clippy::useless_conversion)]

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use serde_json::json;
use spdf_core::container::{self, ContainerLayers};
use spdf_core::diff;
use spdf_core::dom::{Document, Element};
use spdf_core::manifest::Manifest;
use spdf_core::redaction;
use spdf_core::signing;
use spdf_core::types::{DocumentState, GeneratorInfo};
use spdf_core::SpdfError;

fn spdf_err(e: SpdfError) -> PyErr {
    PyValueError::new_err(e.to_string())
}

/// Validate an SPDF container (bytes). Returns ValidationReport as JSON string.
#[pyfunction]
fn validate_spdf(spdf_bytes: &[u8]) -> PyResult<String> {
    let extracted = container::read_container(spdf_bytes).map_err(spdf_err)?;

    let manifest_report = spdf_validator::validate_manifest(&extracted.manifest);

    let doc: Document =
        serde_json::from_slice(&extracted.semantic).map_err(|e| spdf_err(SpdfError::Json(e)))?;
    let document_report = spdf_validator::validate_document(&doc);

    let combined = json!({
        "valid": manifest_report.is_valid() && document_report.is_valid(),
        "manifest_errors": manifest_report.errors,
        "document_errors": document_report.errors,
        "error_count": manifest_report.error_count() + document_report.error_count(),
        "fatal_count": manifest_report.fatal_count() + document_report.fatal_count(),
    });

    serde_json::to_string(&combined).map_err(|e| spdf_err(SpdfError::Json(e)))
}

/// Build a complete SPDF container from layer JSON strings. Returns raw .spdf bytes.
///
/// The render layer (PDF) is automatically generated from the semantic layer.
#[pyfunction]
fn generate_spdf(
    semantic_json: &str,
    layout_json: &str,
    styles_json: &str,
    metadata_json: &str,
    audit_json: &str,
) -> PyResult<Vec<u8>> {
    let doc: Document =
        serde_json::from_str(semantic_json).map_err(|e| spdf_err(SpdfError::Json(e)))?;

    let pdf_bytes = spdf_renderer::render_to_pdf(&doc).map_err(spdf_err)?;

    let doc_id = doc.document_id.clone();
    let mut manifest = Manifest::new(
        doc_id,
        GeneratorInfo {
            name: "spdf-python".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
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

    container::write_container(&mut manifest, &layers, &[]).map_err(spdf_err)
}

/// Read an SPDF container and render its semantic layer to PDF bytes.
#[pyfunction]
fn render_to_pdf(spdf_bytes: &[u8]) -> PyResult<Vec<u8>> {
    let extracted = container::read_container(spdf_bytes).map_err(spdf_err)?;

    let doc: Document =
        serde_json::from_slice(&extracted.semantic).map_err(|e| spdf_err(SpdfError::Json(e)))?;

    spdf_renderer::render_to_pdf(&doc).map_err(spdf_err)
}

/// Parse a semantic JSON string, validate its structure, and return the Document as JSON.
#[pyfunction]
fn parse_semantic(semantic_json: &str) -> PyResult<String> {
    let doc: Document =
        serde_json::from_str(semantic_json).map_err(|e| spdf_err(SpdfError::Json(e)))?;

    serde_json::to_string_pretty(&doc).map_err(|e| spdf_err(SpdfError::Json(e)))
}

/// Extract structured invoice data from an SPDF container.
///
/// Finds InvoiceHeader, LineItemTable, and PaymentTerms elements and returns
/// a structured JSON dict with invoice fields.
#[pyfunction]
fn extract_invoice_data(spdf_bytes: &[u8]) -> PyResult<String> {
    let extracted = container::read_container(spdf_bytes).map_err(spdf_err)?;

    let doc: Document =
        serde_json::from_slice(&extracted.semantic).map_err(|e| spdf_err(SpdfError::Json(e)))?;

    let mut invoice_header = None;
    let mut line_item_table = None;
    let mut payment_terms = None;

    for page in &doc.pages {
        for element in &page.elements {
            match element {
                Element::InvoiceHeader(ih) => invoice_header = Some(ih),
                Element::LineItemTable(lt) => line_item_table = Some(lt),
                Element::PaymentTerms(pt) => payment_terms = Some(pt),
                _ => {}
            }
        }
    }

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
                            json!({ "header": header, "value": cell.value, "type": cell.spdf_type })
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

    serde_json::to_string(&result).map_err(|e| spdf_err(SpdfError::Json(e)))
}

/// Sign an SPDF document (must be in Review state). Returns signed .spdf bytes.
#[pyfunction]
fn sign_document(spdf_bytes: &[u8], signer_name: &str, signer_email: &str) -> PyResult<Vec<u8>> {
    signing::sign_document_simple(spdf_bytes, signer_name, signer_email).map_err(spdf_err)
}

/// Verify all signatures in an SPDF document. Returns VerificationReport as JSON string.
#[pyfunction]
fn verify_document(spdf_bytes: &[u8]) -> PyResult<String> {
    let report = signing::verify_document_simple(spdf_bytes).map_err(spdf_err)?;
    serde_json::to_string(&report).map_err(|e| spdf_err(SpdfError::Json(e)))
}

/// Compare two SPDF documents. Returns DiffReport as JSON string.
#[pyfunction]
fn diff_documents(doc_a_bytes: &[u8], doc_b_bytes: &[u8]) -> PyResult<String> {
    let report = diff::diff_documents(doc_a_bytes, doc_b_bytes).map_err(spdf_err)?;
    serde_json::to_string(&report).map_err(|e| spdf_err(SpdfError::Json(e)))
}

/// Redact an element from an SPDF document. Returns updated .spdf bytes.
#[pyfunction]
fn redact_element(spdf_bytes: &[u8], target_eid: &str, reason: &str) -> PyResult<Vec<u8>> {
    redaction::redact_element(spdf_bytes, target_eid, reason).map_err(spdf_err)
}

/// List all redactions in an SPDF document. Returns JSON string.
#[pyfunction]
fn list_redactions(spdf_bytes: &[u8]) -> PyResult<String> {
    let entries = redaction::list_redactions(spdf_bytes).map_err(spdf_err)?;
    serde_json::to_string(&entries).map_err(|e| spdf_err(SpdfError::Json(e)))
}

/// Verify a redaction by EID. Returns JSON string.
#[pyfunction]
fn verify_redaction(spdf_bytes: &[u8], redaction_eid: &str) -> PyResult<String> {
    let result = redaction::verify_redaction(spdf_bytes, redaction_eid).map_err(spdf_err)?;
    serde_json::to_string(&result).map_err(|e| spdf_err(SpdfError::Json(e)))
}

/// Transition a document to a new state. Returns updated .spdf bytes.
#[pyfunction]
fn transition_document(spdf_bytes: &[u8], target_state: &str) -> PyResult<Vec<u8>> {
    let state = match target_state {
        "DRAFT" => DocumentState::Draft,
        "REVIEW" => DocumentState::Review,
        "SIGNED" => DocumentState::Signed,
        "CERTIFIED" => DocumentState::Certified,
        _ => {
            return Err(PyValueError::new_err(format!(
                "invalid state: {target_state}"
            )))
        }
    };
    signing::transition_document(spdf_bytes, state).map_err(spdf_err)
}

#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pymodule]
fn spdf_native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_function(wrap_pyfunction!(validate_spdf, m)?)?;
    m.add_function(wrap_pyfunction!(generate_spdf, m)?)?;
    m.add_function(wrap_pyfunction!(render_to_pdf, m)?)?;
    m.add_function(wrap_pyfunction!(parse_semantic, m)?)?;
    m.add_function(wrap_pyfunction!(extract_invoice_data, m)?)?;
    m.add_function(wrap_pyfunction!(sign_document, m)?)?;
    m.add_function(wrap_pyfunction!(verify_document, m)?)?;
    m.add_function(wrap_pyfunction!(diff_documents, m)?)?;
    m.add_function(wrap_pyfunction!(redact_element, m)?)?;
    m.add_function(wrap_pyfunction!(list_redactions, m)?)?;
    m.add_function(wrap_pyfunction!(verify_redaction, m)?)?;
    m.add_function(wrap_pyfunction!(transition_document, m)?)?;
    Ok(())
}
