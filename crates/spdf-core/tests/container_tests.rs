use std::io::{Cursor, Write};

use spdf_core::container::{read_container, write_container, ContainerLayers};
use spdf_core::manifest::{sha256_hex, Manifest};
use spdf_core::types::{DocumentId, GeneratorInfo};

fn test_generator() -> GeneratorInfo {
    GeneratorInfo {
        name: "spdf-test".to_string(),
        version: "0.0.1".to_string(),
    }
}

fn test_layers() -> ContainerLayers {
    ContainerLayers {
        semantic: br#"{"pages":[]}"#.to_vec(),
        layout: br#"{"layout":"default"}"#.to_vec(),
        styles: br#"{"styles":{}}"#.to_vec(),
        render: b"%PDF-1.4 fake render".to_vec(),
        metadata: br#"{"author":"test"}"#.to_vec(),
        audit: br#"{"events":[]}"#.to_vec(),
    }
}

// ---------- Happy-path round-trip ----------

#[test]
fn write_then_read_round_trip() {
    let doc_id = DocumentId::new();
    let mut manifest = Manifest::new(doc_id.clone(), test_generator());
    let layers = test_layers();
    let assets: Vec<(String, Vec<u8>)> = vec![];

    let bytes = write_container(&mut manifest, &layers, &assets).expect("write_container failed");
    let extracted = read_container(&bytes).expect("read_container failed");

    assert_eq!(extracted.semantic, layers.semantic);
    assert_eq!(extracted.layout, layers.layout);
    assert_eq!(extracted.styles, layers.styles);
    assert_eq!(extracted.render, layers.render);
    assert_eq!(extracted.metadata, layers.metadata);
    assert_eq!(extracted.audit, layers.audit);
    assert!(extracted.assets.is_empty());
}

#[test]
fn manifest_checksums_match_layer_content() {
    let mut manifest = Manifest::new(DocumentId::new(), test_generator());
    let layers = test_layers();

    let bytes = write_container(&mut manifest, &layers, &[]).unwrap();
    let extracted = read_container(&bytes).unwrap();

    assert_eq!(extracted.manifest.layers.semantic, sha256_hex(&layers.semantic));
    assert_eq!(extracted.manifest.layers.layout, sha256_hex(&layers.layout));
    assert_eq!(extracted.manifest.layers.styles, sha256_hex(&layers.styles));
    assert_eq!(extracted.manifest.layers.render, sha256_hex(&layers.render));
    assert_eq!(extracted.manifest.layers.metadata, sha256_hex(&layers.metadata));
    assert_eq!(extracted.manifest.layers.audit, sha256_hex(&layers.audit));
}

#[test]
fn manifest_hash_is_populated() {
    let mut manifest = Manifest::new(DocumentId::new(), test_generator());
    let layers = test_layers();

    let bytes = write_container(&mut manifest, &layers, &[]).unwrap();
    let extracted = read_container(&bytes).unwrap();

    assert!(!extracted.manifest.manifest_hash.is_empty());
    assert_eq!(extracted.manifest.manifest_hash.len(), 64); // SHA-256 hex
}

#[test]
fn manifest_format_and_version() {
    let mut manifest = Manifest::new(DocumentId::new(), test_generator());
    let layers = test_layers();

    let bytes = write_container(&mut manifest, &layers, &[]).unwrap();
    let extracted = read_container(&bytes).unwrap();

    assert_eq!(extracted.manifest.format, "SPDF");
    assert_eq!(extracted.manifest.version.major, 1);
    assert_eq!(extracted.manifest.version.minor, 0);
}

#[test]
fn document_id_preserved() {
    let doc_id = DocumentId::new();
    let mut manifest = Manifest::new(doc_id.clone(), test_generator());
    let layers = test_layers();

    let bytes = write_container(&mut manifest, &layers, &[]).unwrap();
    let extracted = read_container(&bytes).unwrap();

    assert_eq!(extracted.manifest.document_id, doc_id);
}

// ---------- Assets ----------

#[test]
fn round_trip_with_single_asset() {
    let mut manifest = Manifest::new(DocumentId::new(), test_generator());
    let layers = test_layers();
    let asset_data = b"PNG fake image data".to_vec();
    let assets = vec![("logo.png".to_string(), asset_data.clone())];

    let bytes = write_container(&mut manifest, &layers, &assets).unwrap();
    let extracted = read_container(&bytes).unwrap();

    assert_eq!(extracted.assets.len(), 1);
    assert_eq!(extracted.assets[0].0, "logo.png");
    assert_eq!(extracted.assets[0].1, asset_data);
}

#[test]
fn round_trip_with_multiple_assets() {
    let mut manifest = Manifest::new(DocumentId::new(), test_generator());
    let layers = test_layers();
    let assets = vec![
        ("logo.png".to_string(), b"PNG data".to_vec()),
        ("signature.svg".to_string(), b"<svg/>".to_vec()),
        ("attachment.pdf".to_string(), b"%PDF-1.4".to_vec()),
    ];

    let bytes = write_container(&mut manifest, &layers, &assets).unwrap();
    let extracted = read_container(&bytes).unwrap();

    assert_eq!(extracted.assets.len(), 3);
    let names: Vec<&str> = extracted.assets.iter().map(|(n, _)| n.as_str()).collect();
    assert!(names.contains(&"logo.png"));
    assert!(names.contains(&"signature.svg"));
    assert!(names.contains(&"attachment.pdf"));
}

// ---------- Checksum integrity ----------

#[test]
fn corrupted_layer_detected() {
    let mut manifest = Manifest::new(DocumentId::new(), test_generator());
    let layers = test_layers();

    let bytes = write_container(&mut manifest, &layers, &[]).unwrap();

    // Tamper: modify a byte in the ZIP to corrupt a layer.
    // Re-create with bad checksum by manually building a container
    // with mismatched manifest checksums.
    let extracted = read_container(&bytes).unwrap();

    // Write a new container where manifest checksums don't match actual layers
    let mut bad_manifest = extracted.manifest.clone();
    bad_manifest.layers.semantic = "0000000000000000000000000000000000000000000000000000000000000000".to_string();
    // Don't re-finalize — keep bad checksum

    let bad_bytes = build_container_raw(&bad_manifest, &layers, &[]);
    let result = read_container(&bad_bytes);
    assert!(result.is_err(), "should detect checksum mismatch");
    let err = format!("{}", result.unwrap_err());
    assert!(err.contains("checksum mismatch"), "error should mention checksum: {err}");
}

#[test]
fn empty_layers_round_trip() {
    let mut manifest = Manifest::new(DocumentId::new(), test_generator());
    let layers = ContainerLayers {
        semantic: b"{}".to_vec(),
        layout: b"{}".to_vec(),
        styles: b"{}".to_vec(),
        render: b"".to_vec(),
        metadata: b"{}".to_vec(),
        audit: b"{}".to_vec(),
    };

    let bytes = write_container(&mut manifest, &layers, &[]).unwrap();
    let extracted = read_container(&bytes).unwrap();

    assert_eq!(extracted.semantic, b"{}");
    assert_eq!(extracted.render, b"");
}

// ---------- Error cases ----------

#[test]
fn read_invalid_zip_data() {
    let garbage = b"this is not a zip file";
    let result = read_container(garbage);
    assert!(result.is_err());
}

#[test]
fn read_zip_missing_manifest() {
    // Create a valid ZIP but without manifest.json
    let mut buf = Vec::new();
    {
        let mut zip = zip::ZipWriter::new(Cursor::new(&mut buf));
        let options = zip::write::SimpleFileOptions::default();
        zip.start_file("layers/semantic.json", options).unwrap();
        zip.write_all(b"{}").unwrap();
        zip.finish().unwrap();
    }
    let result = read_container(&buf);
    assert!(result.is_err());
    let err = format!("{}", result.unwrap_err());
    assert!(err.contains("manifest.json"), "should mention missing manifest: {err}");
}

#[test]
fn read_zip_missing_layer() {
    // Create a ZIP with manifest but missing a required layer
    let mut manifest = Manifest::new(DocumentId::new(), test_generator());
    let layers = test_layers();
    // Write a full container to get a valid manifest, then strip a layer
    let _ = write_container(&mut manifest, &layers, &[]).unwrap();

    let mut buf = Vec::new();
    {
        let mut zip = zip::ZipWriter::new(Cursor::new(&mut buf));
        let options = zip::write::SimpleFileOptions::default();
        zip.start_file("manifest.json", options).unwrap();
        let manifest_json = serde_json::to_vec_pretty(&manifest).unwrap();
        zip.write_all(&manifest_json).unwrap();
        // Only write semantic layer, skip the rest
        zip.start_file("layers/semantic.json", options).unwrap();
        zip.write_all(&layers.semantic).unwrap();
        zip.finish().unwrap();
    }
    let result = read_container(&buf);
    assert!(result.is_err(), "should fail when layers are missing");
}

// ---------- Large data ----------

#[test]
fn large_layer_round_trip() {
    let mut manifest = Manifest::new(DocumentId::new(), test_generator());
    let big_data: Vec<u8> = (0u32..262144)
        .flat_map(|i| {
            let h = i.wrapping_mul(0x9E3779B9);
            let h = h ^ (h >> 16);
            h.to_le_bytes()
        })
        .collect(); // 1 MB of pseudo-random data
    let layers = ContainerLayers {
        semantic: big_data.clone(),
        layout: b"{}".to_vec(),
        styles: b"{}".to_vec(),
        render: b"".to_vec(),
        metadata: b"{}".to_vec(),
        audit: b"{}".to_vec(),
    };

    let bytes = write_container(&mut manifest, &layers, &[]).unwrap();
    let extracted = read_container(&bytes).unwrap();

    assert_eq!(extracted.semantic.len(), 1024 * 1024);
    assert_eq!(extracted.semantic, big_data);
}

// ---------- Determinism ----------

#[test]
fn same_input_produces_same_checksums() {
    let layers = test_layers();
    let doc_id = DocumentId::new();

    let mut m1 = Manifest::new(doc_id.clone(), test_generator());
    let bytes1 = write_container(&mut m1, &layers, &[]).unwrap();
    let ext1 = read_container(&bytes1).unwrap();

    let mut m2 = Manifest::new(doc_id, test_generator());
    let bytes2 = write_container(&mut m2, &layers, &[]).unwrap();
    let ext2 = read_container(&bytes2).unwrap();

    assert_eq!(ext1.manifest.layers.semantic, ext2.manifest.layers.semantic);
    assert_eq!(ext1.manifest.layers.render, ext2.manifest.layers.render);
}

// ---------- Helper: build a raw container with pre-set manifest ----------

fn build_container_raw(
    manifest: &Manifest,
    layers: &ContainerLayers,
    assets: &[(String, Vec<u8>)],
) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut zip = zip::ZipWriter::new(Cursor::new(&mut buf));
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    let store = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored);

    zip.start_file("manifest.json", options).unwrap();
    let manifest_json = serde_json::to_vec_pretty(manifest).unwrap();
    zip.write_all(&manifest_json).unwrap();

    zip.start_file("layers/semantic.json", options).unwrap();
    zip.write_all(&layers.semantic).unwrap();
    zip.start_file("layers/layout.json", options).unwrap();
    zip.write_all(&layers.layout).unwrap();
    zip.start_file("layers/styles.json", options).unwrap();
    zip.write_all(&layers.styles).unwrap();
    zip.start_file("layers/render.pdf", store).unwrap();
    zip.write_all(&layers.render).unwrap();
    zip.start_file("layers/metadata.json", options).unwrap();
    zip.write_all(&layers.metadata).unwrap();
    zip.start_file("layers/audit.json", options).unwrap();
    zip.write_all(&layers.audit).unwrap();

    for (path, data) in assets {
        zip.start_file(format!("assets/{path}"), options).unwrap();
        zip.write_all(data).unwrap();
    }

    zip.set_comment("SPDF/1.0");
    zip.finish().unwrap();
    buf
}
