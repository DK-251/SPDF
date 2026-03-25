# Check Results

## Run Info
- **Version:** 0.1.0-snapshot.3
- **Commit:** a3ae28b
- **Branch:** main
- **Date:** 2026-03-25T15:37:54Z
- **Machine:** TUF_WARRIOR_DK
- **Overall:** FAIL

## Steps
- [ ] cargo fmt --check: FAIL
- [x] cargo clippy: PASS
- [x] cargo test: PASS

### cargo fmt --check (last 80 lines)
````
     let id = DocumentId::new();
[31m-    assert!(id.0.starts_with("spdf-"), "DocumentId should start with spdf-: {}", id.0);
[0m[32m+    assert!(
[0m[32m+        id.0.starts_with("spdf-"),
[0m[32m+        "DocumentId should start with spdf-: {}",
[0m[32m+        id.0
[0m[32m+    );
[0m }
 
 #[test]
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core\tests\dom_tests.rs:454:
 
 #[test]
 fn text_direction_serde() {
[31m-    assert_eq!(serde_json::to_string(&TextDirection::Ltr).unwrap(), r#""LTR""#);
[0m[31m-    assert_eq!(serde_json::to_string(&TextDirection::Rtl).unwrap(), r#""RTL""#);
[0m[32m+    assert_eq!(
[0m[32m+        serde_json::to_string(&TextDirection::Ltr).unwrap(),
[0m[32m+        r#""LTR""#
[0m[32m+    );
[0m[32m+    assert_eq!(
[0m[32m+        serde_json::to_string(&TextDirection::Rtl).unwrap(),
[0m[32m+        r#""RTL""#
[0m[32m+    );
[0m }
 
 #[test]
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core\tests\dom_tests.rs:462:
 fn source_format_serde() {
[31m-    assert_eq!(serde_json::to_string(&SourceFormat::Native).unwrap(), r#""NATIVE""#);
[0m[31m-    assert_eq!(serde_json::to_string(&SourceFormat::ConvertedFromPdf).unwrap(), r#""CONVERTED_FROM_PDF""#);
[0m[31m-    assert_eq!(serde_json::to_string(&SourceFormat::Template).unwrap(), r#""TEMPLATE""#);
[0m[32m+    assert_eq!(
[0m[32m+        serde_json::to_string(&SourceFormat::Native).unwrap(),
[0m[32m+        r#""NATIVE""#
[0m[32m+    );
[0m[32m+    assert_eq!(
[0m[32m+        serde_json::to_string(&SourceFormat::ConvertedFromPdf).unwrap(),
[0m[32m+        r#""CONVERTED_FROM_PDF""#
[0m[32m+    );
[0m[32m+    assert_eq!(
[0m[32m+        serde_json::to_string(&SourceFormat::Template).unwrap(),
[0m[32m+        r#""TEMPLATE""#
[0m[32m+    );
[0m }
 
 // ---------- Multi-page document ----------
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core\tests\dom_tests.rs:547:
                 Element::LineItemTable(LineItemTableElement {
                     eid: eid(),
                     headers: vec!["Item".into(), "Qty".into(), "Rate".into(), "Amount".into()],
[31m-                    rows: vec![
[0m[31m-                        vec![
[0m[31m-                            TableCell { value: "API Integration".into(), spdf_type: None },
[0m[31m-                            TableCell { value: "1".into(), spdf_type: Some("integer".into()) },
[0m[31m-                            TableCell { value: "75000.00".into(), spdf_type: Some("currency".into()) },
[0m[31m-                            TableCell { value: "75000.00".into(), spdf_type: Some("currency".into()) },
[0m[31m-                        ],
[0m[31m-                    ],
[0m[32m+                    rows: vec![vec![
[0m[32m+                        TableCell {
[0m[32m+                            value: "API Integration".into(),
[0m[32m+                            spdf_type: None,
[0m[32m+                        },
[0m[32m+                        TableCell {
[0m[32m+                            value: "1".into(),
[0m[32m+                            spdf_type: Some("integer".into()),
[0m[32m+                        },
[0m[32m+                        TableCell {
[0m[32m+                            value: "75000.00".into(),
[0m[32m+                            spdf_type: Some("currency".into()),
[0m[32m+                        },
[0m[32m+                        TableCell {
[0m[32m+                            value: "75000.00".into(),
[0m[32m+                            spdf_type: Some("currency".into()),
[0m[32m+                        },
[0m[32m+                    ]],
[0m                     timestamps: ts(),
                 }),
                 Element::PaymentTerms(PaymentTermsElement {
`````n
