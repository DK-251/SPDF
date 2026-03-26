//! WASM bindings for SPDF core (used in Studio frontend).
//!
//! Internal helper functions are pure Rust and tested via `cargo test -p spdf-wasm`.
//! `#[wasm_bindgen]` entry points are thin wrappers that convert to/from JsValue.

use wasm_bindgen::prelude::*;

use spdf_core::container;
use spdf_core::diff;
use spdf_core::dom::Document;
use spdf_core::redaction;
use spdf_core::signing;

// --- Internal helpers (testable via rlib) ---

pub fn validate_internal(spdf_bytes: &[u8]) -> Result<String, String> {
    let extracted = container::read_container(spdf_bytes).map_err(|e| e.to_string())?;

    let manifest_report = spdf_validator::validate_manifest(&extracted.manifest);

    let doc: Document =
        serde_json::from_slice(&extracted.semantic).map_err(|e| e.to_string())?;
    let document_report = spdf_validator::validate_document(&doc);

    let combined = serde_json::json!({
        "valid": manifest_report.is_valid() && document_report.is_valid(),
        "manifest_errors": manifest_report.errors,
        "document_errors": document_report.errors,
        "error_count": manifest_report.error_count() + document_report.error_count(),
        "fatal_count": manifest_report.fatal_count() + document_report.fatal_count(),
    });

    serde_json::to_string(&combined).map_err(|e| e.to_string())
}

pub fn get_document_info_internal(spdf_bytes: &[u8]) -> Result<String, String> {
    let extracted = container::read_container(spdf_bytes).map_err(|e| e.to_string())?;

    let doc: Document =
        serde_json::from_slice(&extracted.semantic).map_err(|e| e.to_string())?;

    let total_elements: usize = doc.pages.iter().map(|p| p.elements.len()).sum();

    let info = serde_json::json!({
        "document_id": doc.document_id.0,
        "title": doc.title,
        "locale": doc.locale,
        "state": doc.document_state,
        "page_count": doc.pages.len(),
        "element_count": total_elements,
        "version": format!("{}", doc.version),
    });

    serde_json::to_string(&info).map_err(|e| e.to_string())
}

pub fn verify_internal(spdf_bytes: &[u8]) -> Result<String, String> {
    let report = signing::verify_document_simple(spdf_bytes).map_err(|e| e.to_string())?;
    serde_json::to_string(&report).map_err(|e| e.to_string())
}

pub fn diff_internal(doc_a_bytes: &[u8], doc_b_bytes: &[u8]) -> Result<String, String> {
    let report = diff::diff_documents(doc_a_bytes, doc_b_bytes).map_err(|e| e.to_string())?;
    serde_json::to_string(&report).map_err(|e| e.to_string())
}

pub fn list_redactions_internal(spdf_bytes: &[u8]) -> Result<String, String> {
    let entries = redaction::list_redactions(spdf_bytes).map_err(|e| e.to_string())?;
    serde_json::to_string(&entries).map_err(|e| e.to_string())
}

pub fn extract_invoice_internal(spdf_bytes: &[u8]) -> Result<String, String> {
    let extracted = container::read_container(spdf_bytes).map_err(|e| e.to_string())?;

    let doc: Document =
        serde_json::from_slice(&extracted.semantic).map_err(|e| e.to_string())?;

    let mut invoice_header = None;
    let mut payment_terms = None;

    for page in &doc.pages {
        for element in &page.elements {
            match element {
                spdf_core::dom::Element::InvoiceHeader(ih) => invoice_header = Some(ih),
                spdf_core::dom::Element::PaymentTerms(pt) => payment_terms = Some(pt),
                _ => {}
            }
        }
    }

    let result = serde_json::json!({
        "has_invoice": invoice_header.is_some(),
        "invoice_number": invoice_header.map(|h| &h.invoice_number),
        "total": payment_terms.map(|p| &p.total),
        "currency": invoice_header.and_then(|h| h.currency.as_ref()),
    });

    serde_json::to_string(&result).map_err(|e| e.to_string())
}

// --- WASM entry points ---

#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[wasm_bindgen]
pub fn validate(spdf_bytes: &[u8]) -> Result<String, String> {
    validate_internal(spdf_bytes)
}

#[wasm_bindgen]
pub fn get_document_info(spdf_bytes: &[u8]) -> Result<String, String> {
    get_document_info_internal(spdf_bytes)
}

#[wasm_bindgen]
pub fn verify(spdf_bytes: &[u8]) -> Result<String, String> {
    verify_internal(spdf_bytes)
}

#[wasm_bindgen]
pub fn diff_documents(doc_a_bytes: &[u8], doc_b_bytes: &[u8]) -> Result<String, String> {
    diff_internal(doc_a_bytes, doc_b_bytes)
}

#[wasm_bindgen]
pub fn list_redactions(spdf_bytes: &[u8]) -> Result<String, String> {
    list_redactions_internal(spdf_bytes)
}

#[wasm_bindgen]
pub fn extract_invoice(spdf_bytes: &[u8]) -> Result<String, String> {
    extract_invoice_internal(spdf_bytes)
}
