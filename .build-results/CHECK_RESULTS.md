# Check Results

## Run Info
- **Version:** 0.1.0-snapshot.3
- **Commit:** a9aea2b
- **Branch:** main
- **Date:** 2026-03-25T15:25:35Z
- **Machine:** TUF_WARRIOR_DK
- **Overall:** FAIL

## Steps
- [ ] cargo fmt --check: FAIL
- [x] cargo clippy: PASS
- [ ] cargo test: FAIL

### cargo fmt --check (last 80 lines)
````
[0m[32m+                            TableCell {
[0m[32m+                                value: "1".into(),
[0m[32m+                                spdf_type: Some("integer".into()),
[0m[32m+                            },
[0m[32m+                            TableCell {
[0m[32m+                                value: "75000.00".into(),
[0m[32m+                                spdf_type: Some("currency".into()),
[0m[32m+                            },
[0m[32m+                            TableCell {
[0m[32m+                                value: "75000.00".into(),
[0m[32m+                                spdf_type: Some("currency".into()),
[0m[32m+                            },
[0m                         ],
                         vec![
[31m-                            TableCell { value: "Custom PDF Templates (5)".into(), spdf_type: None },
[0m[31m-                            TableCell { value: "5".into(), spdf_type: Some("integer".into()) },
[0m[31m-                            TableCell { value: "10000.00".into(), spdf_type: Some("currency".into()) },
[0m[31m-                            TableCell { value: "50000.00".into(), spdf_type: Some("currency".into()) },
[0m[32m+                            TableCell {
[0m[32m+                                value: "Custom PDF Templates (5)".into(),
[0m[32m+                                spdf_type: None,
[0m[32m+                            },
[0m[32m+                            TableCell {
[0m[32m+                                value: "5".into(),
[0m[32m+                                spdf_type: Some("integer".into()),
[0m[32m+                            },
[0m[32m+                            TableCell {
[0m[32m+                                value: "10000.00".into(),
[0m[32m+                                spdf_type: Some("currency".into()),
[0m[32m+                            },
[0m[32m+                            TableCell {
[0m[32m+                                value: "50000.00".into(),
[0m[32m+                                spdf_type: Some("currency".into()),
[0m[32m+                            },
[0m                         ],
                     ],
                     timestamps: ts(),
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer\tests\integration_test.rs:114:
 
     // 2. Validate the document structure
     let doc_report = validate_document(&doc);
[31m-    assert!(doc_report.is_valid(), "Document validation failed: {:?}", doc_report.errors);
[0m[32m+    assert!(
[0m[32m+        doc_report.is_valid(),
[0m[32m+        "Document validation failed: {:?}",
[0m[32m+        doc_report.errors
[0m[32m+    );
[0m 
     // 3. Serialize DOM to JSON (semantic layer)
     let semantic_json = serde_json::to_vec_pretty(&doc).expect("DOM serialization failed");
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer\tests\integration_test.rs:122:
 
     // 4. Render to PDF
     let pdf_bytes = render_to_pdf(&doc).expect("PDF rendering failed");
[31m-    assert!(pdf_bytes.len() > 100, "PDF too small: {} bytes", pdf_bytes.len());
[0m[32m+    assert!(
[0m[32m+        pdf_bytes.len() > 100,
[0m[32m+        "PDF too small: {} bytes",
[0m[32m+        pdf_bytes.len()
[0m[32m+    );
[0m     // Verify it starts with %PDF
[31m-    assert_eq!(&pdf_bytes[..5], b"%PDF-", "rendered output is not a valid PDF");
[0m[32m+    assert_eq!(
[0m[32m+        &pdf_bytes[..5],
[0m[32m+        b"%PDF-",
[0m[32m+        "rendered output is not a valid PDF"
[0m[32m+    );
[0m 
     // 5. Build container layers
     let layers = ContainerLayers {
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer\tests\integration_test.rs:135:
         metadata: serde_json::to_vec(&serde_json::json!({
             "title": doc.title,
             "locale": doc.locale,
[31m-        })).unwrap(),
[0m[32m+        }))
[0m[32m+        .unwrap(),
[0m         audit: serde_json::to_vec(&serde_json::json!({"events": []})).unwrap(),
     };
 
`````n

### cargo test (last 80 lines)
````
cargo :    Compiling spdf-core v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core)
At line:1 char:1
+ cargo test --workspace *> ".build-results\check-test.log" 2>&1
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling sp...ates\spdf-core):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   Compiling spdf-renderer v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.99s
     Running unittests src\lib.rs (target\debug\deps\spdf_core-c9d433ce6c9b1045.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\container_tests.rs (target\debug\deps\container_tests-0798c7706bb5367a.exe)

running 14 tests
test read_invalid_zip_data ... ok
test read_zip_missing_manifest ... ok
test manifest_format_and_version ... ok
test manifest_hash_is_populated ... ok
test document_id_preserved ... ok
test read_zip_missing_layer ... ok
test write_then_read_round_trip ... ok
test empty_layers_round_trip ... ok
test manifest_checksums_match_layer_content ... ok
test round_trip_with_single_asset ... ok
test round_trip_with_multiple_assets ... ok
test corrupted_layer_detected ... ok
test same_input_produces_same_checksums ... ok
test large_layer_round_trip ... FAILED

failures:

---- large_layer_round_trip stdout ----

thread 'large_layer_round_trip' (10052) panicked at crates\spdf-core\tests\container_tests.rs:249:44:
called `Result::unwrap()` on an `Err` value: DecompressionBomb { ratio: 186.56835555555554, max: 100.0 }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    large_layer_round_trip

test result: FAILED. 13 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s

error: test failed, to rerun pass `-p spdf-core --test container_tests`
`````n
