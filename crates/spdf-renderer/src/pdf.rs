//! PDF generation from SPDF DOM elements using lopdf.
//!
//! Renders a basic but complete PDF with text, tables, and page structure.
//! Uses the 14 standard PDF fonts (no embedding needed).

use lopdf::content::{Content, Operation};
use lopdf::{dictionary, Document as PdfDocument, Object, Stream};

use spdf_core::dom::{Document, Element};
use spdf_core::error::{SpdfError, SpdfResult};

/// Page dimensions in points (A4).
const PAGE_WIDTH: f32 = 595.28;
const PAGE_HEIGHT: f32 = 841.89;
const MARGIN_LEFT: f32 = 50.0;
const MARGIN_TOP: f32 = 50.0;
const MARGIN_BOTTOM: f32 = 50.0;
const LINE_HEIGHT: f32 = 16.0;

/// Font sizes for different element types.
const HEADING_SIZES: [f32; 6] = [24.0, 20.0, 16.0, 14.0, 12.0, 10.0];
const BODY_FONT_SIZE: f32 = 11.0;
const TABLE_FONT_SIZE: f32 = 10.0;

/// Render an SPDF Document to PDF bytes.
pub fn render_to_pdf(doc: &Document) -> SpdfResult<Vec<u8>> {
    let mut pdf = PdfDocument::with_version("1.7");

    let pages_id = pdf.new_object_id();
    let font_id = pdf.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    });
    let font_bold_id = pdf.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica-Bold",
    });

    let resources_id = pdf.add_object(dictionary! {
        "Font" => dictionary! {
            "F1" => font_id,
            "F2" => font_bold_id,
        },
    });

    let mut page_ids = Vec::new();

    for page in &doc.pages {
        let mut ops: Vec<Operation> = Vec::new();
        let mut cursor_y = PAGE_HEIGHT - MARGIN_TOP;

        for element in &page.elements {
            if cursor_y < MARGIN_BOTTOM + LINE_HEIGHT {
                break;
            }
            render_element(element, &mut ops, &mut cursor_y);
        }

        let content = Content { operations: ops };
        let content_bytes = content.encode()
            .map_err(|e| SpdfError::Io(std::io::Error::other(e.to_string())))?;

        let content_id = pdf.add_object(Stream::new(dictionary! {}, content_bytes));

        let page_id = pdf.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "MediaBox" => vec![0.into(), 0.into(), PAGE_WIDTH.into(), PAGE_HEIGHT.into()],
            "Contents" => content_id,
            "Resources" => resources_id,
        });
        page_ids.push(page_id);
    }

    // If no pages in document, create one empty page
    if page_ids.is_empty() {
        let content = Content { operations: vec![] };
        let content_bytes = content.encode()
            .map_err(|e| SpdfError::Io(std::io::Error::other(e.to_string())))?;
        let content_id = pdf.add_object(Stream::new(dictionary! {}, content_bytes));
        let page_id = pdf.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "MediaBox" => vec![0.into(), 0.into(), PAGE_WIDTH.into(), PAGE_HEIGHT.into()],
            "Contents" => content_id,
            "Resources" => resources_id,
        });
        page_ids.push(page_id);
    }

    let page_refs: Vec<Object> = page_ids.iter().map(|&id| id.into()).collect();
    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => page_refs,
        "Count" => page_ids.len() as u32,
    };
    pdf.objects.insert(pages_id, Object::Dictionary(pages));

    let catalog_id = pdf.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    pdf.trailer.set("Root", catalog_id);

    let mut buf = Vec::new();
    pdf.save_to(&mut buf)
        .map_err(|e| SpdfError::Io(std::io::Error::other(e.to_string())))?;

    Ok(buf)
}

fn render_element(element: &Element, ops: &mut Vec<Operation>, cursor_y: &mut f32) {
    match element {
        Element::Heading(h) => {
            let size = h.font_size.unwrap_or_else(|| {
                let idx = (h.level as usize).saturating_sub(1).min(5);
                HEADING_SIZES[idx]
            });
            render_text(ops, cursor_y, &h.text, "F2", size);
            *cursor_y -= size * 0.5; // extra spacing after heading
        }
        Element::Paragraph(p) => {
            let size = p.font_size.unwrap_or(BODY_FONT_SIZE);
            render_text(ops, cursor_y, &p.text, "F1", size);
        }
        Element::Table(t) => {
            // Render headers in bold
            if !t.headers.is_empty() {
                let header_line = t.headers.join("  |  ");
                render_text(ops, cursor_y, &header_line, "F2", TABLE_FONT_SIZE);
                // Separator
                render_text(ops, cursor_y, &"-".repeat(header_line.len()), "F1", TABLE_FONT_SIZE);
            }
            for row in &t.rows {
                let line: String = row.cells.iter().map(|c| c.value.as_str()).collect::<Vec<_>>().join("  |  ");
                render_text(ops, cursor_y, &line, "F1", TABLE_FONT_SIZE);
            }
            *cursor_y -= LINE_HEIGHT * 0.5;
        }
        Element::InvoiceHeader(ih) => {
            render_text(ops, cursor_y, &format!("Invoice: {}", ih.invoice_number), "F2", 16.0);
            render_text(ops, cursor_y, &format!("Date: {}  Due: {}", ih.issue_date, ih.due_date), "F1", BODY_FONT_SIZE);
            render_text(ops, cursor_y, &format!("From: {}", ih.vendor.name), "F1", BODY_FONT_SIZE);
            render_text(ops, cursor_y, &format!("To: {}", ih.client.name), "F1", BODY_FONT_SIZE);
            *cursor_y -= LINE_HEIGHT * 0.5;
        }
        Element::LineItemTable(lt) => {
            if !lt.headers.is_empty() {
                let header_line = lt.headers.join("  |  ");
                render_text(ops, cursor_y, &header_line, "F2", TABLE_FONT_SIZE);
                render_text(ops, cursor_y, &"-".repeat(header_line.len()), "F1", TABLE_FONT_SIZE);
            }
            for row in &lt.rows {
                let line: String = row.iter().map(|c| c.value.as_str()).collect::<Vec<_>>().join("  |  ");
                render_text(ops, cursor_y, &line, "F1", TABLE_FONT_SIZE);
            }
            *cursor_y -= LINE_HEIGHT * 0.5;
        }
        Element::PaymentTerms(pt) => {
            render_text(ops, cursor_y, &format!("Subtotal: {}", pt.subtotal), "F1", BODY_FONT_SIZE);
            if let Some(ref disc) = pt.discount {
                render_text(ops, cursor_y, &format!("Discount: {disc}"), "F1", BODY_FONT_SIZE);
            }
            if let (Some(ref label), Some(ref amount)) = (&pt.tax_label, &pt.tax_amount) {
                render_text(ops, cursor_y, &format!("{label}: {amount}"), "F1", BODY_FONT_SIZE);
            }
            render_text(ops, cursor_y, &format!("Total: {}", pt.total), "F2", 14.0);
            *cursor_y -= LINE_HEIGHT;
        }
        Element::SignatureBlock(sb) => {
            *cursor_y -= LINE_HEIGHT;
            render_text(ops, cursor_y, &format!("Signed by: {}", sb.signer_name), "F1", BODY_FONT_SIZE);
            if let Some(ref title) = sb.signer_title {
                render_text(ops, cursor_y, title, "F1", 9.0);
            }
        }
        Element::CodeBlock(cb) => {
            render_text(ops, cursor_y, &format!("[{}]", cb.language), "F2", 9.0);
            for line in cb.code.lines() {
                render_text(ops, cursor_y, line, "F1", 9.0);
            }
            *cursor_y -= LINE_HEIGHT * 0.5;
        }
        Element::HorizontalRule(_) => {
            *cursor_y -= LINE_HEIGHT;
        }
        Element::PageBreak(_) => {
            *cursor_y = MARGIN_BOTTOM; // force next element to be skipped
        }
        Element::Annotation(a) => {
            render_text(ops, cursor_y, &format!("[Note by {}]: {}", a.author, a.content), "F1", 9.0);
        }
        _ => {
            // Image, VectorImage, Attachment, Stamp, Redaction, FormField, VariablePlaceholder
            // rendered as placeholder text for now
        }
    }
}

fn render_text(
    ops: &mut Vec<Operation>,
    cursor_y: &mut f32,
    text: &str,
    font: &str,
    size: f32,
) {
    if *cursor_y < MARGIN_BOTTOM {
        return;
    }

    ops.push(Operation::new("BT", vec![]));
    ops.push(Operation::new(
        "Tf",
        vec![font.into(), size.into()],
    ));
    ops.push(Operation::new(
        "Td",
        vec![MARGIN_LEFT.into(), (*cursor_y).into()],
    ));
    // Escape parentheses in text for PDF string safety
    let escaped = text.replace('\\', "\\\\").replace('(', "\\(").replace(')', "\\)");
    ops.push(Operation::new("Tj", vec![Object::string_literal(escaped)]));
    ops.push(Operation::new("ET", vec![]));

    *cursor_y -= size + (LINE_HEIGHT - size).max(2.0);
}
