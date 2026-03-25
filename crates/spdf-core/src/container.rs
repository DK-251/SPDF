//! Container: read and write .spdf files (ZIP archives).
//!
//! Write order: manifest.json must be the first entry.
//! Read: validates manifest integrity and layer checksums.

use std::io::{Cursor, Read, Write};

use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use crate::error::{SpdfError, SpdfResult};
use crate::manifest::{sha256_hex, Manifest};

/// Maximum allowed compressed file size (100 MB).
const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024;

/// Maximum decompression ratio to prevent ZIP bombs.
/// Real ZIP bombs exceed 1M:1; legitimate compressed data rarely exceeds 1000:1.
const MAX_DECOMPRESSION_RATIO: f64 = 1000.0;

/// Layers bundled for writing into a container.
pub struct ContainerLayers {
    pub semantic: Vec<u8>,
    pub layout: Vec<u8>,
    pub styles: Vec<u8>,
    pub render: Vec<u8>,
    pub metadata: Vec<u8>,
    pub audit: Vec<u8>,
}

/// Layers extracted from reading a container.
#[derive(Debug)]
pub struct ExtractedLayers {
    pub manifest: Manifest,
    pub semantic: Vec<u8>,
    pub layout: Vec<u8>,
    pub styles: Vec<u8>,
    pub render: Vec<u8>,
    pub metadata: Vec<u8>,
    pub audit: Vec<u8>,
    pub assets: Vec<(String, Vec<u8>)>,
}

/// Write an SPDF container to bytes.
pub fn write_container(
    manifest: &mut Manifest,
    layers: &ContainerLayers,
    assets: &[(String, Vec<u8>)],
) -> SpdfResult<Vec<u8>> {
    // Compute layer checksums
    manifest.layers.semantic = sha256_hex(&layers.semantic);
    manifest.layers.layout = sha256_hex(&layers.layout);
    manifest.layers.styles = sha256_hex(&layers.styles);
    manifest.layers.render = sha256_hex(&layers.render);
    manifest.layers.metadata = sha256_hex(&layers.metadata);
    manifest.layers.audit = sha256_hex(&layers.audit);
    manifest.finalize();

    let mut buf = Vec::new();
    let mut zip = ZipWriter::new(Cursor::new(&mut buf));
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    // Manifest must be first entry
    zip.start_file("manifest.json", options)?;
    let manifest_json = serde_json::to_vec_pretty(manifest)?;
    zip.write_all(&manifest_json)?;

    // Layers
    zip.start_file("layers/semantic.json", options)?;
    zip.write_all(&layers.semantic)?;

    zip.start_file("layers/layout.json", options)?;
    zip.write_all(&layers.layout)?;

    zip.start_file("layers/styles.json", options)?;
    zip.write_all(&layers.styles)?;

    // Render layer (PDF) — store without compression, already compressed
    let store_options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    zip.start_file("layers/render.pdf", store_options)?;
    zip.write_all(&layers.render)?;

    zip.start_file("layers/metadata.json", options)?;
    zip.write_all(&layers.metadata)?;

    zip.start_file("layers/audit.json", options)?;
    zip.write_all(&layers.audit)?;

    // Assets
    for (path, data) in assets {
        zip.start_file(format!("assets/{path}"), options)?;
        zip.write_all(data)?;
    }

    // Archive comment for format identification
    zip.set_comment("SPDF/1.0");
    zip.finish()?;

    Ok(buf)
}

/// Read and validate an SPDF container from bytes.
pub fn read_container(data: &[u8]) -> SpdfResult<ExtractedLayers> {
    let compressed_size = data.len() as u64;
    if compressed_size > MAX_FILE_SIZE {
        return Err(SpdfError::FileSizeExceeded {
            size: compressed_size,
            max: MAX_FILE_SIZE,
        });
    }

    let cursor = Cursor::new(data);
    let mut archive = ZipArchive::new(cursor)?;

    // Track total decompressed size for ZIP bomb detection
    let mut total_decompressed: u64 = 0;

    let read_entry = |archive: &mut ZipArchive<Cursor<&[u8]>>,
                      name: &str,
                      total: &mut u64|
     -> SpdfResult<Vec<u8>> {
        let mut entry = archive
            .by_name(name)
            .map_err(|_| SpdfError::MissingEntry(name.to_string()))?;

        let mut buf = Vec::new();
        entry.read_to_end(&mut buf)?;
        *total += buf.len() as u64;

        if compressed_size > 0 {
            let ratio = *total as f64 / compressed_size as f64;
            if ratio > MAX_DECOMPRESSION_RATIO {
                return Err(SpdfError::DecompressionBomb {
                    ratio,
                    max: MAX_DECOMPRESSION_RATIO,
                });
            }
        }
        Ok(buf)
    };

    // Read manifest first
    let manifest_bytes = read_entry(&mut archive, "manifest.json", &mut total_decompressed)?;
    let manifest: Manifest = serde_json::from_slice(&manifest_bytes)?;

    if manifest.format != "SPDF" {
        return Err(SpdfError::Manifest(format!(
            "expected format 'SPDF', got '{}'",
            manifest.format
        )));
    }

    // Read layers
    let semantic = read_entry(
        &mut archive,
        "layers/semantic.json",
        &mut total_decompressed,
    )?;
    let layout = read_entry(&mut archive, "layers/layout.json", &mut total_decompressed)?;
    let styles = read_entry(&mut archive, "layers/styles.json", &mut total_decompressed)?;
    let render = read_entry(&mut archive, "layers/render.pdf", &mut total_decompressed)?;
    let metadata = read_entry(
        &mut archive,
        "layers/metadata.json",
        &mut total_decompressed,
    )?;
    let audit = read_entry(&mut archive, "layers/audit.json", &mut total_decompressed)?;

    // Verify layer checksums
    verify_checksum("semantic", &manifest.layers.semantic, &semantic)?;
    verify_checksum("layout", &manifest.layers.layout, &layout)?;
    verify_checksum("styles", &manifest.layers.styles, &styles)?;
    verify_checksum("render", &manifest.layers.render, &render)?;
    verify_checksum("metadata", &manifest.layers.metadata, &metadata)?;
    verify_checksum("audit", &manifest.layers.audit, &audit)?;

    // Read assets
    let mut assets = Vec::new();
    for i in 0..archive.len() {
        let entry = archive.by_index(i)?;
        let name = entry.name().to_string();
        if name.starts_with("assets/") && !name.ends_with('/') {
            drop(entry);
            let asset_data = read_entry(&mut archive, &name, &mut total_decompressed)?;
            let relative = name.strip_prefix("assets/").unwrap_or(&name).to_string();
            assets.push((relative, asset_data));
        }
    }

    Ok(ExtractedLayers {
        manifest,
        semantic,
        layout,
        styles,
        render,
        metadata,
        audit,
        assets,
    })
}

fn verify_checksum(layer: &str, expected: &str, data: &[u8]) -> SpdfResult<()> {
    let actual = sha256_hex(data);
    if actual != expected {
        return Err(SpdfError::ChecksumMismatch {
            expected: format!("{layer}: {expected}"),
            actual: format!("{layer}: {actual}"),
        });
    }
    Ok(())
}
