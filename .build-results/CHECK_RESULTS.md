# Check Results

## Run Info
- **Version:** 0.1.0-snapshot.3
- **Commit:** c1c4453
- **Branch:** main
- **Date:** 2026-03-25T15:22:02Z
- **Machine:** TUF_WARRIOR_DK
- **Overall:** FAIL

## Steps
- [ ] cargo fmt --check: FAIL
- [x] cargo clippy: PASS
- [ ] cargo test: FAIL

### cargo fmt --check (last 80 lines)
````
[0m[32m+            name: "spdf-integration-test".to_string(),
[0m[32m+            version: "0.1.0".to_string(),
[0m[32m+        },
[0m[32m+    );
[0m[32m+    let container_bytes =
[0m[32m+        write_container(&mut manifest, &layers, &[]).expect("Container write failed");
[0m     assert!(container_bytes.len() > 0);
 
     // 7. Validate the manifest
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer\tests\integration_test.rs:152:
     let manifest_report = validate_manifest(&manifest);
[31m-    assert!(manifest_report.is_valid(), "Manifest validation failed: {:?}", manifest_report.errors);
[0m[32m+    assert!(
[0m[32m+        manifest_report.is_valid(),
[0m[32m+        "Manifest validation failed: {:?}",
[0m[32m+        manifest_report.errors
[0m[32m+    );
[0m 
     // 8. Read container back
     let extracted = read_container(&container_bytes).expect("Container read failed");
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer\tests\integration_test.rs:160:
     assert_eq!(extracted.manifest.format, "SPDF");
 
     // 10. Deserialize the semantic layer back to a Document
[31m-    let rt_doc: Document = serde_json::from_slice(&extracted.semantic)
[0m[31m-        .expect("DOM deserialization failed");
[0m[32m+    let rt_doc: Document =
[0m[32m+        serde_json::from_slice(&extracted.semantic).expect("DOM deserialization failed");
[0m     assert_eq!(rt_doc.title, "Invoice INV-2026-100");
     assert_eq!(rt_doc.pages.len(), 1);
     assert_eq!(rt_doc.pages[0].elements.len(), 7);
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer\tests\integration_test.rs:168:
 
     // 11. Validate the round-tripped document
     let rt_report = validate_document(&rt_doc);
[31m-    assert!(rt_report.is_valid(), "Round-tripped doc validation failed: {:?}", rt_report.errors);
[0m[32m+    assert!(
[0m[32m+        rt_report.is_valid(),
[0m[32m+        "Round-tripped doc validation failed: {:?}",
[0m[32m+        rt_report.errors
[0m[32m+    );
[0m 
     // 12. Verify the rendered PDF layer starts with %PDF-
     assert_eq!(&extracted.render[..5], b"%PDF-");
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer\tests\integration_test.rs:255:
         audit: b"{}".to_vec(),
     };
 
[31m-    let mut manifest = Manifest::new(doc.document_id.clone(), GeneratorInfo {
[0m[31m-        name: "test".to_string(),
[0m[31m-        version: "0.1.0".to_string(),
[0m[31m-    });
[0m[32m+    let mut manifest = Manifest::new(
[0m[32m+        doc.document_id.clone(),
[0m[32m+        GeneratorInfo {
[0m[32m+            name: "test".to_string(),
[0m[32m+            version: "0.1.0".to_string(),
[0m[32m+        },
[0m[32m+    );
[0m     let bytes = write_container(&mut manifest, &layers, &[]).unwrap();
     let extracted = read_container(&bytes).unwrap();
     let rt_doc: Document = serde_json::from_slice(&extracted.semantic).unwrap();
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer\tests\integration_test.rs:305:
     };
 
     let assets = vec![("logo-001.png".to_string(), logo_data.clone())];
[31m-    let mut manifest = Manifest::new(doc.document_id.clone(), GeneratorInfo {
[0m[31m-        name: "test".to_string(),
[0m[31m-        version: "0.1.0".to_string(),
[0m[31m-    });
[0m[32m+    let mut manifest = Manifest::new(
[0m[32m+        doc.document_id.clone(),
[0m[32m+        GeneratorInfo {
[0m[32m+            name: "test".to_string(),
[0m[32m+            version: "0.1.0".to_string(),
[0m[32m+        },
[0m[32m+    );
[0m 
     let bytes = write_container(&mut manifest, &layers, &assets).unwrap();
     let extracted = read_container(&bytes).unwrap();
`````n

### cargo test (last 80 lines)
````
cargo :    Compiling spdf-validator v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-validator)
At line:1 char:1
+ cargo test --workspace *> ".build-results\check-test.log" 2>&1
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling sp...spdf-validator):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   Compiling spdf-core v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core)
   Compiling spdf-python v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python)
   Compiling spdf-renderer v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.75s
     Running unittests src\lib.rs (target\debug\deps\spdf_core-c9d433ce6c9b1045.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\container_tests.rs (target\debug\deps\container_tests-0798c7706bb5367a.exe)

running 14 tests
test read_invalid_zip_data ... ok
test read_zip_missing_manifest ... ok
test manifest_format_and_version ... ok
test read_zip_missing_layer ... ok
test manifest_checksums_match_layer_content ... ok
test manifest_hash_is_populated ... ok
test empty_layers_round_trip ... ok
test round_trip_with_single_asset ... ok
test document_id_preserved ... ok
test write_then_read_round_trip ... ok
test round_trip_with_multiple_assets ... ok
test same_input_produces_same_checksums ... ok
test corrupted_layer_detected ... ok
test large_layer_round_trip ... FAILED

failures:

---- large_layer_round_trip stdout ----

thread 'large_layer_round_trip' (10660) panicked at crates\spdf-core\tests\container_tests.rs:247:44:
called `Result::unwrap()` on an `Err` value: DecompressionBomb { ratio: 186.50204371778923, max: 100.0 }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    large_layer_round_trip

test result: FAILED. 13 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s

error: test failed, to rerun pass `-p spdf-core --test container_tests`
`````n
