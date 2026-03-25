//! SPDF Renderer: generates PDF bytes from the semantic + layout + styles layers.
//!
//! Uses `lopdf` for low-level PDF construction.

pub mod pdf;

pub use pdf::render_to_pdf;
