# Check Results

## Run Info
- **Version:** 0.1.0-snapshot.4
- **Commit:** 6cf531b
- **Branch:** main
- **Date:** 2026-03-25T16:23:05Z
- **Machine:** TUF_WARRIOR_DK
- **Overall:** FAIL

## Steps
- [ ] cargo fmt --check: FAIL
- [ ] cargo clippy: FAIL
- [x] cargo test: PASS

### cargo fmt --check (last 80 lines)
````
[0m[32m+    assert!(
[0m[32m+        manifest_report.is_valid(),
[0m[32m+        "manifest: {:?}",
[0m[32m+        manifest_report.errors
[0m[32m+    );
[0m     assert!(doc_report.is_valid(), "document: {:?}", doc_report.errors);
 }
 
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer\tests\binding_logic_tests.rs:279:
         audit: b"{}".to_vec(),
     };
 
[31m-    let mut manifest = Manifest::new(parsed.document_id.clone(), GeneratorInfo {
[0m[31m-        name: "test".to_string(),
[0m[31m-        version: "0.1.0".to_string(),
[0m[31m-    });
[0m[32m+    let mut manifest = Manifest::new(
[0m[32m+        parsed.document_id.clone(),
[0m[32m+        GeneratorInfo {
[0m[32m+            name: "test".to_string(),
[0m[32m+            version: "0.1.0".to_string(),
[0m[32m+        },
[0m[32m+    );
[0m 
     let container_bytes = container::write_container(&mut manifest, &layers, &[]).unwrap();
     let extracted = container::read_container(&container_bytes).unwrap();
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer\tests\binding_logic_tests.rs:703:
     // E_001: empty title
     let mut doc = sample_invoice_doc();
     doc.title = "".into();
[31m-    assert!(validate_document(&doc).errors.iter().any(|e| e.code == "E_001"));
[0m[32m+    assert!(validate_document(&doc)
[0m[32m+        .errors
[0m[32m+        .iter()
[0m[32m+        .any(|e| e.code == "E_001"));
[0m 
     // E_002: empty locale
     let mut doc = sample_invoice_doc();
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer\tests\binding_logic_tests.rs:710:
     doc.locale = "".into();
[31m-    assert!(validate_document(&doc).errors.iter().any(|e| e.code == "E_002"));
[0m[32m+    assert!(validate_document(&doc)
[0m[32m+        .errors
[0m[32m+        .iter()
[0m[32m+        .any(|e| e.code == "E_002"));
[0m 
     // F_001: no pages
     let mut doc = sample_invoice_doc();
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer\tests\binding_logic_tests.rs:715:
     doc.pages.clear();
[31m-    assert!(validate_document(&doc).errors.iter().any(|e| e.code == "F_001"));
[0m[32m+    assert!(validate_document(&doc)
[0m[32m+        .errors
[0m[32m+        .iter()
[0m[32m+        .any(|e| e.code == "F_001"));
[0m 
     // F_002: empty page
     let mut doc = sample_invoice_doc();
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer\tests\binding_logic_tests.rs:720:
     doc.pages[0].elements.clear();
[31m-    assert!(validate_document(&doc).errors.iter().any(|e| e.code == "F_002"));
[0m[32m+    assert!(validate_document(&doc)
[0m[32m+        .errors
[0m[32m+        .iter()
[0m[32m+        .any(|e| e.code == "F_002"));
[0m 
     // E_006: heading level 0
     let mut doc = sample_invoice_doc();
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer\tests\binding_logic_tests.rs:731:
         color: None,
         timestamps: ts(),
     })];
[31m-    assert!(validate_document(&doc).errors.iter().any(|e| e.code == "E_006"));
[0m[32m+    assert!(validate_document(&doc)
[0m[32m+        .errors
[0m[32m+        .iter()
[0m[32m+        .any(|e| e.code == "E_006"));
[0m }
 
 #[test]
`````n

### cargo clippy (last 80 lines)
````
cargo :     Checking spdf-core v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core)
At line:1 char:1
+ cargo clippy --workspace -- -D warnings *> ".build-results\check-clip ...
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Checking sp...ates\spdf-core):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
    Checking spdf-renderer v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer)
    Checking spdf-validator v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-validator)
    Checking spdf-wasm v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-wasm)
    Checking spdf-python v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python)
error: unused import: `DocumentId`
  --> crates\spdf-python\src\lib.rs:13:24
   |
13 | use spdf_core::types::{DocumentId, GeneratorInfo};
   |                        ^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(unused_imports)]`

error: useless conversion to the same type: `pyo3::PyErr`
  --> crates\spdf-python\src\lib.rs:22:48
   |
22 | fn validate_spdf(spdf_bytes: &[u8]) -> PyResult<String> {
   |                                                ^ help: consider removing
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#useless_conversion
   = note: `-D clippy::useless-conversion` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::useless_conversion)]`

error: useless conversion to the same type: `pyo3::PyErr`
  --> crates\spdf-python\src\lib.rs:52:14
   |
52 | ) -> PyResult<Vec<u8>> {
   |              ^ help: consider removing
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#useless_conversion

error: useless conversion to the same type: `pyo3::PyErr`
  --> crates\spdf-python\src\lib.rs:82:48
   |
82 | fn render_to_pdf(spdf_bytes: &[u8]) -> PyResult<Vec<u8>> {
   |                                                ^ help: consider removing
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#useless_conversion

error: useless conversion to the same type: `pyo3::PyErr`
  --> crates\spdf-python\src\lib.rs:93:51
   |
93 | fn parse_semantic(semantic_json: &str) -> PyResult<String> {
   |                                                   ^ help: consider removing
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#useless_conversion

error: useless conversion to the same type: `pyo3::PyErr`
   --> crates\spdf-python\src\lib.rs:105:55
    |
105 | fn extract_invoice_data(spdf_bytes: &[u8]) -> PyResult<String> {
    |                                                       ^ help: consider removing
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#useless_conversion

error: could not compile `spdf-python` (lib) due to 6 previous errors
`````n
