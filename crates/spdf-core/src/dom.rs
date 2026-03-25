//! Document Object Model: the 25 semantic element types.
//!
//! Each element carries its own `eid` (ElementId) and typed payload.
//! The DOM is a tree: Document -> Pages -> Elements.

use serde::{Deserialize, Serialize};

use crate::types::{DocumentId, DocumentState, ElementId, SpdfVersion, TextDirection, Timestamps};

/// Root of an SPDF semantic layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    #[serde(rename = "spdf:version")]
    pub version: SpdfVersion,
    pub document_id: DocumentId,
    pub title: String,
    pub locale: String,
    pub direction: TextDirection,
    pub document_state: DocumentState,
    pub pages: Vec<Page>,
}

/// A single page containing elements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub eid: ElementId,
    pub page_number: u32,
    pub elements: Vec<Element>,
}

/// Union of all 25 semantic element types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "element_type")]
pub enum Element {
    // --- Content elements ---
    Heading(HeadingElement),
    Paragraph(ParagraphElement),
    Table(TableElement),
    Image(ImageElement),
    VectorImage(VectorImageElement),
    CodeBlock(CodeBlockElement),
    HorizontalRule(HorizontalRuleElement),
    PageBreak(PageBreakElement),
    Attachment(AttachmentElement),

    // --- Domain-specific elements ---
    InvoiceHeader(InvoiceHeaderElement),
    LineItemTable(LineItemTableElement),
    PaymentTerms(PaymentTermsElement),

    // --- Trust elements ---
    SignatureBlock(SignatureBlockElement),
    Stamp(StampElement),
    Annotation(AnnotationElement),
    Redaction(RedactionElement),

    // --- Interactive elements ---
    FormField(FormFieldElement),
    VariablePlaceholder(VariablePlaceholderElement),
}

// --- Content element structs ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingElement {
    pub eid: ElementId,
    pub level: u8,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParagraphElement {
    pub eid: ElementId,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableElement {
    pub eid: ElementId,
    pub headers: Vec<String>,
    pub rows: Vec<TableRow>,
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCell {
    pub value: String,
    #[serde(rename = "spdf:type", skip_serializing_if = "Option::is_none")]
    pub spdf_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageElement {
    pub eid: ElementId,
    pub asset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    pub width: f32,
    pub height: f32,
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorImageElement {
    pub eid: ElementId,
    pub asset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    pub width: f32,
    pub height: f32,
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlockElement {
    pub eid: ElementId,
    pub language: String,
    pub code: String,
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorizontalRuleElement {
    pub eid: ElementId,
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBreakElement {
    pub eid: ElementId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentElement {
    pub eid: ElementId,
    pub asset_id: String,
    pub file_name: String,
    pub mime_type: String,
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

// --- Domain-specific element structs ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartyInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gstin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceHeaderElement {
    pub eid: ElementId,
    pub invoice_number: String,
    pub issue_date: String,
    pub due_date: String,
    pub vendor: PartyInfo,
    pub client: PartyInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineItemTableElement {
    pub eid: ElementId,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<TableCell>>,
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTermsElement {
    pub eid: ElementId,
    pub subtotal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<String>,
    pub total: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

// --- Trust element structs ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureBlockElement {
    pub eid: ElementId,
    pub signer_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_fingerprint: Option<String>,
    pub locked: bool,
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StampElement {
    pub eid: ElementId,
    pub stamp_type: String,
    pub asset_id: String,
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationElement {
    pub eid: ElementId,
    pub author: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_eid: Option<ElementId>,
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedactionElement {
    pub eid: ElementId,
    pub redacted_eid: ElementId,
    pub reason: String,
    pub erasure_proof_hash: String,
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

// --- Interactive element structs ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormFieldType {
    Text,
    Textarea,
    Select,
    Checkbox,
    Date,
    Number,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormFieldElement {
    pub eid: ElementId,
    pub field_name: String,
    pub field_type: FormFieldType,
    pub label: String,
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(flatten)]
    pub timestamps: Timestamps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariablePlaceholderElement {
    pub eid: ElementId,
    pub variable_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(flatten)]
    pub timestamps: Timestamps,
}
