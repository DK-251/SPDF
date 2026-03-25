# Check Results

## Run Info
- **Version:** 0.1.0-snapshot.3
- **Commit:** 8b979fe
- **Branch:** main
- **Date:** 2026-03-25T15:29:20Z
- **Machine:** TUF_WARRIOR_DK
- **Overall:** FAIL

## Steps
- [ ] cargo fmt --check: FAIL
- [x] cargo clippy: PASS
- [ ] cargo test: FAIL

### cargo fmt --check (last 80 lines)
````
[0m             }
             if let (Some(ref label), Some(ref amount)) = (&pt.tax_label, &pt.tax_amount) {
[31m-                render_text(ops, cursor_y, &format!("{label}: {amount}"), "F1", BODY_FONT_SIZE);
[0m[32m+                render_text(
[0m[32m+                    ops,
[0m[32m+                    cursor_y,
[0m[32m+                    &format!("{label}: {amount}"),
[0m[32m+                    "F1",
[0m[32m+                    BODY_FONT_SIZE,
[0m[32m+                );
[0m             }
             render_text(ops, cursor_y, &format!("Total: {}", pt.total), "F2", 14.0);
             *cursor_y -= LINE_HEIGHT;
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer\src\pdf.rs:171:
         }
         Element::SignatureBlock(sb) => {
             *cursor_y -= LINE_HEIGHT;
[31m-            render_text(ops, cursor_y, &format!("Signed by: {}", sb.signer_name), "F1", BODY_FONT_SIZE);
[0m[32m+            render_text(
[0m[32m+                ops,
[0m[32m+                cursor_y,
[0m[32m+                &format!("Signed by: {}", sb.signer_name),
[0m[32m+                "F1",
[0m[32m+                BODY_FONT_SIZE,
[0m[32m+            );
[0m             if let Some(ref title) = sb.signer_title {
                 render_text(ops, cursor_y, title, "F1", 9.0);
             }
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer\src\pdf.rs:190:
             *cursor_y = MARGIN_BOTTOM; // force next element to be skipped
         }
         Element::Annotation(a) => {
[31m-            render_text(ops, cursor_y, &format!("[Note by {}]: {}", a.author, a.content), "F1", 9.0);
[0m[32m+            render_text(
[0m[32m+                ops,
[0m[32m+                cursor_y,
[0m[32m+                &format!("[Note by {}]: {}", a.author, a.content),
[0m[32m+                "F1",
[0m[32m+                9.0,
[0m[32m+            );
[0m         }
         _ => {
             // Image, VectorImage, Attachment, Stamp, Redaction, FormField, VariablePlaceholder
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer\src\pdf.rs:199:
     }
 }
 
[31m-fn render_text(
[0m[31m-    ops: &mut Vec<Operation>,
[0m[31m-    cursor_y: &mut f32,
[0m[31m-    text: &str,
[0m[31m-    font: &str,
[0m[31m-    size: f32,
[0m[31m-) {
[0m[32m+fn render_text(ops: &mut Vec<Operation>, cursor_y: &mut f32, text: &str, font: &str, size: f32) {
[0m     if *cursor_y < MARGIN_BOTTOM {
         return;
     }
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer\src\pdf.rs:212:
 
     ops.push(Operation::new("BT", vec![]));
[32m+    ops.push(Operation::new("Tf", vec![font.into(), size.into()]));
[0m     ops.push(Operation::new(
[31m-        "Tf",
[0m[31m-        vec![font.into(), size.into()],
[0m[31m-    ));
[0m[31m-    ops.push(Operation::new(
[0m         "Td",
         vec![MARGIN_LEFT.into(), (*cursor_y).into()],
     ));
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer\src\pdf.rs:222:
     // Escape parentheses in text for PDF string safety
[31m-    let escaped = text.replace('\\', "\\\\").replace('(', "\\(").replace(')', "\\)");
[0m[32m+    let escaped = text
[0m[32m+        .replace('\\', "\\\\")
[0m[32m+        .replace('(', "\\(")
[0m[32m+        .replace(')', "\\)");
[0m     ops.push(Operation::new("Tj", vec![Object::string_literal(escaped)]));
     ops.push(Operation::new("ET", vec![]));
 
`````n

### cargo test (last 80 lines)
````
test round_trip_with_multiple_assets ... ok
test same_input_produces_same_checksums ... ok
test corrupted_layer_detected ... ok
test large_layer_round_trip ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s

     Running tests\dom_tests.rs (target\debug\deps\dom_tests-69ee641a033174e2.exe)

running 31 tests
test element_id_format ... ok
test document_id_format ... ok
test document_state_serde_screaming_snake ... ok
test attachment_element_serde ... ok
test annotation_without_target_serde ... ok
test annotation_element_serde ... ok
test code_block_element_serde ... ok
test document_serialize_deserialize ... ok
test form_field_select_with_options_serde ... ok
test form_field_all_types_serde ... ok
test form_field_text_serde ... ok
test heading_element_serde ... ok
test document_state_transitions ... ok
test full_invoice_document_serde ... ok
test horizontal_rule_element_serde ... ok
test image_element_serde ... ok
test invoice_header_element_serde ... ok
test line_item_table_element_serde ... ok
test multi_page_document_serde ... ok
test page_break_element_serde ... ok
test paragraph_element_serde ... ok
test payment_terms_element_serde ... ok
test redaction_element_serde ... ok
test signature_block_element_serde ... ok
test spdf_version_display ... ok
test source_format_serde ... ok
test stamp_element_serde ... ok
test text_direction_serde ... ok
test table_element_serde ... ok
test vector_image_element_serde ... ok
test variable_placeholder_element_serde ... ok

test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running unittests src\lib.rs (target\debug\deps\spdf_python-7aa428dc40f62cd4.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\lib.rs (target\debug\deps\spdf_renderer-ab573d2a90844bb1.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\integration_test.rs (target\debug\deps\integration_test-5876ac67288f76c0.exe)

running 3 tests
test multi_page_round_trip ... ok
test round_trip_with_assets ... ok
test full_round_trip_invoice ... FAILED

failures:

---- full_round_trip_invoice stdout ----

thread 'full_round_trip_invoice' (19924) panicked at crates\spdf-renderer\tests\integration_test.rs:211:5:
assertion `left == right` failed
  left: 6
 right: 7
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    full_round_trip_invoice

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p spdf-renderer --test integration_test`
`````n
