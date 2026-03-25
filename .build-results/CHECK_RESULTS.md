# Check Results

## Run Info
- **Version:** 0.1.0-snapshot.4
- **Commit:** db01d87
- **Branch:** main
- **Date:** 2026-03-25T16:45:24Z
- **Machine:** TUF_WARRIOR_DK
- **Overall:** FAIL

## Steps
- [ ] cargo fmt --check: FAIL
- [x] cargo clippy: PASS
- [x] cargo test: PASS

### cargo fmt --check (last 80 lines)
````
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python\src\lib.rs:54:
     let doc: Document =
         serde_json::from_str(semantic_json).map_err(|e| spdf_err(SpdfError::Json(e)))?;
 
[31m-    let pdf_bytes =
[0m[31m-        spdf_renderer::render_to_pdf(&doc).map_err(spdf_err)?;
[0m[32m+    let pdf_bytes = spdf_renderer::render_to_pdf(&doc).map_err(spdf_err)?;
[0m 
     let doc_id = doc.document_id.clone();
     let mut manifest = Manifest::new(
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python\src\lib.rs:133:
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
