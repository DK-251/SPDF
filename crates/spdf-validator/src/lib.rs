//! SPDF Validator: enforces format rules, element constraints, and state transitions.
//!
//! Produces structured error codes:
//! - `E_xxx` — validation errors (document is non-conforming but recoverable)
//! - `F_xxx` — fatal errors (document is structurally broken)

pub mod rules;

use serde::Serialize;
use spdf_core::dom::Document;
use spdf_core::manifest::Manifest;

/// Severity of a validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Severity {
    Error,
    Fatal,
}

/// A single validation finding with a typed error code.
#[derive(Debug, Clone, Serialize)]
pub struct ValidationError {
    pub code: &'static str,
    pub severity: Severity,
    pub message: String,
    pub path: Option<String>,
}

/// Result of running all validation rules.
#[derive(Debug, Clone, Serialize)]
pub struct ValidationReport {
    pub errors: Vec<ValidationError>,
}

impl ValidationReport {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn has_fatal(&self) -> bool {
        self.errors.iter().any(|e| e.severity == Severity::Fatal)
    }

    pub fn error_count(&self) -> usize {
        self.errors.iter().filter(|e| e.severity == Severity::Error).count()
    }

    pub fn fatal_count(&self) -> usize {
        self.errors.iter().filter(|e| e.severity == Severity::Fatal).count()
    }
}

/// Validate a parsed SPDF document (semantic layer).
pub fn validate_document(doc: &Document) -> ValidationReport {
    let mut errors = Vec::new();
    rules::run_document_rules(doc, &mut errors);
    ValidationReport { errors }
}

/// Validate a manifest for structural correctness.
pub fn validate_manifest(manifest: &Manifest) -> ValidationReport {
    let mut errors = Vec::new();
    rules::run_manifest_rules(manifest, &mut errors);
    ValidationReport { errors }
}
