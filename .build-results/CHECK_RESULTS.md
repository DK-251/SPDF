# Check Results

## Run Info
- **Version:** 0.1.0-snapshot.3
- **Commit:** f90b841
- **Branch:** main
- **Date:** 2026-03-25T15:47:20Z
- **Machine:** TUF_WARRIOR_DK
- **Overall:** FAIL

## Steps
- [ ] cargo fmt --check: FAIL
- [x] cargo clippy: PASS
- [x] cargo test: PASS

### cargo fmt --check (last 80 lines)
````
     assert_eq!(extracted.manifest.layers.render, sha256_hex(&layers.render));
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core\tests\container_tests.rs:58:
[31m-    assert_eq!(extracted.manifest.layers.metadata, sha256_hex(&layers.metadata));
[0m[32m+    assert_eq!(
[0m[32m+        extracted.manifest.layers.metadata,
[0m[32m+        sha256_hex(&layers.metadata)
[0m[32m+    );
[0m     assert_eq!(extracted.manifest.layers.audit, sha256_hex(&layers.audit));
 }
 
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core\tests\container_tests.rs:149:
 
     // Write a new container where manifest checksums don't match actual layers
     let mut bad_manifest = extracted.manifest.clone();
[31m-    bad_manifest.layers.semantic = "0000000000000000000000000000000000000000000000000000000000000000".to_string();
[0m[32m+    bad_manifest.layers.semantic =
[0m[32m+        "0000000000000000000000000000000000000000000000000000000000000000".to_string();
[0m     // Don't re-finalize ΓÇö keep bad checksum
 
     let bad_bytes = build_container_raw(&bad_manifest, &layers, &[]);
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core\tests\container_tests.rs:156:
     let result = read_container(&bad_bytes);
     assert!(result.is_err(), "should detect checksum mismatch");
     let err = format!("{}", result.unwrap_err());
[31m-    assert!(err.contains("checksum mismatch"), "error should mention checksum: {err}");
[0m[32m+    assert!(
[0m[32m+        err.contains("checksum mismatch"),
[0m[32m+        "error should mention checksum: {err}"
[0m[32m+    );
[0m }
 
 #[test]
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core\tests\container_tests.rs:201:
     let result = read_container(&buf);
     assert!(result.is_err());
     let err = format!("{}", result.unwrap_err());
[31m-    assert!(err.contains("manifest.json"), "should mention missing manifest: {err}");
[0m[32m+    assert!(
[0m[32m+        err.contains("manifest.json"),
[0m[32m+        "should mention missing manifest: {err}"
[0m[32m+    );
[0m }
 
 #[test]
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core\tests\container_tests.rs:286:
     let mut zip = zip::ZipWriter::new(Cursor::new(&mut buf));
     let options = zip::write::SimpleFileOptions::default()
         .compression_method(zip::CompressionMethod::Deflated);
[31m-    let store = zip::write::SimpleFileOptions::default()
[0m[31m-        .compression_method(zip::CompressionMethod::Stored);
[0m[32m+    let store =
[0m[32m+        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
[0m 
     zip.start_file("manifest.json", options).unwrap();
     let manifest_json = serde_json::to_vec_pretty(manifest).unwrap();
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core\tests\dom_tests.rs:171:
 
 #[test]
 fn page_break_element_serde() {
[31m-    let el = Element::PageBreak(PageBreakElement {
[0m[31m-        eid: eid(),
[0m[31m-    });
[0m[32m+    let el = Element::PageBreak(PageBreakElement { eid: eid() });
[0m     let json = serde_json::to_string(&el).unwrap();
     let _: Element = serde_json::from_str(&json).unwrap();
 }
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core\tests\dom_tests.rs:228:
 fn line_item_table_element_serde() {
     let el = Element::LineItemTable(LineItemTableElement {
         eid: eid(),
[31m-        headers: vec!["Description".into(), "Qty".into(), "Rate".into(), "Amount".into()],
[0m[32m+        headers: vec![
[0m[32m+            "Description".into(),
[0m[32m+            "Qty".into(),
[0m[32m+            "Rate".into(),
[0m[32m+            "Amount".into(),
[0m[32m+        ],
[0m         rows: vec![vec![
             TableCell {
                 value: "Consulting".to_string(),
`````n
