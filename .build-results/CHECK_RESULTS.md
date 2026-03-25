# Check Results

## Run Info
- **Version:** 0.1.0-snapshot.4
- **Commit:** e951ff4
- **Branch:** main
- **Date:** 2026-03-25T16:34:23Z
- **Machine:** TUF_WARRIOR_DK
- **Overall:** FAIL

## Steps
- [ ] cargo fmt --check: FAIL
- [ ] cargo clippy: FAIL
- [ ] cargo test: FAIL

### cargo fmt --check (last 80 lines)
````
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python\src\lib.rs:26:
 
     let manifest_report = spdf_validator::validate_manifest(&extracted.manifest);
 
[31m-    let doc: Document =
[0m[31m-        serde_json::from_slice(&extracted.semantic).map_err(SpdfError::from)?;
[0m[32m+    let doc: Document = serde_json::from_slice(&extracted.semantic).map_err(SpdfError::from)?;
[0m     let document_report = spdf_validator::validate_document(&doc);
 
     let combined = json!({
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python\src\lib.rs:53:
     metadata_json: &str,
     audit_json: &str,
 ) -> PyResult<Vec<u8>> {
[31m-    let doc: Document =
[0m[31m-        serde_json::from_str(semantic_json).map_err(SpdfError::from)?;
[0m[32m+    let doc: Document = serde_json::from_str(semantic_json).map_err(SpdfError::from)?;
[0m 
     let pdf_bytes = spdf_renderer::render_to_pdf(&doc)?;
 
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python\src\lib.rs:84:
 fn render_to_pdf(spdf_bytes: &[u8]) -> PyResult<Vec<u8>> {
     let extracted = container::read_container(spdf_bytes)?;
 
[31m-    let doc: Document =
[0m[31m-        serde_json::from_slice(&extracted.semantic).map_err(SpdfError::from)?;
[0m[32m+    let doc: Document = serde_json::from_slice(&extracted.semantic).map_err(SpdfError::from)?;
[0m 
     Ok(spdf_renderer::render_to_pdf(&doc)?)
 }
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python\src\lib.rs:93:
 /// Parse a semantic JSON string, validate its structure, and return the Document as JSON.
 #[pyfunction]
 fn parse_semantic(semantic_json: &str) -> PyResult<String> {
[31m-    let doc: Document =
[0m[31m-        serde_json::from_str(semantic_json).map_err(SpdfError::from)?;
[0m[32m+    let doc: Document = serde_json::from_str(semantic_json).map_err(SpdfError::from)?;
[0m 
     let json = serde_json::to_string_pretty(&doc).map_err(SpdfError::from)?;
     Ok(json)
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python\src\lib.rs:108:
 fn extract_invoice_data(spdf_bytes: &[u8]) -> PyResult<String> {
     let extracted = container::read_container(spdf_bytes)?;
 
[31m-    let doc: Document =
[0m[31m-        serde_json::from_slice(&extracted.semantic).map_err(SpdfError::from)?;
[0m[32m+    let doc: Document = serde_json::from_slice(&extracted.semantic).map_err(SpdfError::from)?;
[0m 
     let mut invoice_header = None;
     let mut line_item_table = None;
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python\src\lib.rs:135:
                         .iter()
                         .enumerate()
                         .map(|(i, cell)| {
[31m-                            let header = lt
[0m[31m-                                .headers
[0m[31m-                                .get(i)
[0m[31m-                                .map(|h| h.as_str())
[0m[31m-                                .unwrap_or("unknown");
[0m[32m+                            let header = lt.headers.get(i).map(|h| h.as_str()).unwrap_or("unknown");
[0m                             json!({ "header": header, "value": cell.value, "type": cell.spdf_type })
                         })
                         .collect();
`````n

### cargo clippy (last 80 lines)
````
cargo :     Checking spdf-python v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python)
At line:1 char:1
+ cargo clippy --workspace -- -D warnings *> ".build-results\check-clip ...
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Checking sp...es\spdf-python):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
  --> crates\spdf-python\src\lib.rs:16:1
   |
16 | impl From<SpdfError> for PyErr {
   | ^^^^^---------------^^^^^-----
   |      |                   |
   |      |                   `pyo3::PyErr` is not defined in the current crate
   |      `spdf_core::SpdfError` is not defined in the current crate
   |
   = note: impl doesn't have any local type before any uncovered type parameters
   = note: for more information see https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules
   = note: define and implement a trait or new type instead

For more information about this error, try `rustc --explain E0117`.
error: could not compile `spdf-python` (lib) due to 1 previous error
`````n

### cargo test (last 80 lines)
````
cargo :    Compiling spdf-renderer v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer)
At line:1 char:1
+ cargo test --workspace *> ".build-results\check-test.log" 2>&1
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling sp...\spdf-renderer):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   Compiling spdf-python v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python)
error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
  --> crates\spdf-python\src\lib.rs:16:1
   |
16 | impl From<SpdfError> for PyErr {
   | ^^^^^---------------^^^^^-----
   |      |                   |
   |      |                   `pyo3::PyErr` is not defined in the current crate
   |      `SpdfError` is not defined in the current crate
   |
   = note: impl doesn't have any local type before any uncovered type parameters
   = note: for more information see https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules
   = note: define and implement a trait or new type instead

For more information about this error, try `rustc --explain E0117`.
error: could not compile `spdf-python` (lib test) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
`````n
