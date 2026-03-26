# CHECK RESULTS

## Run Info
| Field | Value |
|-------|-------|
| Version | 0.1.0-snapshot.13 |
| Commit | `9db24da` |
| Branch | main |
| Date | 2026-03-26T07:19:23Z |
| Machine | TUF_WARRIOR_DK |
| Overall | **FAILING** (2 pass, 4 fail, 0 skip / 6 steps) |

## Steps

| # | Section | Step | Result |
|---|---------|------|--------|
| 1 | Rust | `cargo fmt --check` | **FAIL** |
| 2 | Rust | `cargo clippy` | **FAIL** |
| 3 | Rust | `cargo test` | **FAIL** |
| 4 | Python | `pip install api[dev]` | PASS |
| 5 | Python | `maturin develop` | PASS |
| 6 | Python | `pytest` | **FAIL** |

---

## Rust Test Breakdown

| Module | Passed | Failed |
|--------|--------|--------|
| `tests\container_tests.rs` | 14 | 0 |
| `tests\diff_tests.rs` | 11 | 0 |
| `tests\dom_tests.rs` | 31 | 0 |
| `tests\redaction_tests.rs` | 11 | 0 |
| `tests\signing_tests.rs` | 14 | **1** |
| **Total** | **81** | **1** |

---

## Python Test Breakdown

| Module | Passed | Failed | Skipped |
|--------|--------|--------|---------|
| `test_account.py` | 16 | 0 | 0 |
| `test_billing.py` | 11 | 0 | 0 |
| `test_diff.py` | 9 | 0 | 0 |
| `test_documents.py` | 32 | 0 | 0 |
| `test_e2e.py` | 8 | 0 | 0 |
| `test_hardening.py` | 9 | **4** | 0 |
| `test_jwt_auth.py` | 11 | 0 | 0 |
| `test_rate_limit.py` | 13 | 0 | 0 |
| `test_redaction.py` | 7 | **1** | 0 |
| `test_signing.py` | 8 | **3** | 0 |
| `test_templates.py` | 15 | 0 | 0 |
| `test_webhooks.py` | 10 | 0 | 0 |
| **Total** | **149** | **8** | **0** |

### Python Warnings (24)

```text
tests/test_e2e.py::test_jwt_auth_template_crud
tests/test_jwt_auth.py::test_jwt_auth_returns_200
tests/test_jwt_auth.py::test_jwt_auth_resolves_correct_user
tests/test_jwt_auth.py::test_jwt_auth_rate_limit_headers
tests/test_jwt_auth.py::test_both_auth_methods_on_same_endpoint
tests/test_jwt_auth.py::test_expired_jwt_returns_401
tests/test_jwt_auth.py::test_unknown_email_returns_401
tests/test_jwt_auth.py::test_wrong_issuer_returns_401
D:\SPDF DEVELOPMENT\SPDF\.venv\Lib\site-packages\jwt\api_jwt.py:147: InsecureKeyLengthWarning: The HMAC key is 20 bytes long, which is below the minimum recommended length of 32 bytes for SHA256. See RFC 7518 Section 3.2.
return self._jws.encode(
tests/test_e2e.py::test_jwt_auth_template_crud
tests/test_jwt_auth.py::test_jwt_auth_returns_200
tests/test_jwt_auth.py::test_jwt_auth_resolves_correct_user
tests/test_jwt_auth.py::test_jwt_auth_rate_limit_headers
tests/test_jwt_auth.py::test_both_auth_methods_on_same_endpoint
tests/test_jwt_auth.py::test_expired_jwt_returns_401
tests/test_jwt_auth.py::test_bad_signature_returns_401
tests/test_jwt_auth.py::test_unknown_email_returns_401
tests/test_jwt_auth.py::test_wrong_issuer_returns_401
D:\SPDF DEVELOPMENT\SPDF\.venv\Lib\site-packages\jwt\api_jwt.py:365: InsecureKeyLengthWarning: The HMAC key is 20 bytes long, which is below the minimum recommended length of 32 bytes for SHA256. See RFC 7518 Section 3.2.
decoded = self.decode_complete(
tests/test_jwt_auth.py::test_bad_signature_returns_401
D:\SPDF DEVELOPMENT\SPDF\.venv\Lib\site-packages\jwt\api_jwt.py:147: InsecureKeyLengthWarning: The HMAC key is 12 bytes long, which is below the minimum recommended length of 32 bytes for SHA256. See RFC 7518 Section 3.2.
return self._jws.encode(
```

**Summary:** `================= 8 failed, 149 passed, 18 warnings in 9.39s ==================`

---

## Grand Total

| | Passed | Failed | Skipped | Total |
|--|--------|--------|---------|-------|
| Rust | 81 | 1 | 0 | 82 |
| Python | 149 | 8 | 0 | 157 |
| **Total** | **230** | **9** | **0** | **239** |

---

## Failure Details

### Rust: cargo fmt --check
Exit code: 1

```text
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core\src\redaction.rs:32:
 ///
 /// Replaces the target element with a `RedactionElement` containing the
 /// SHA-256 erasure proof hash of the original element's JSON serialization.
[31m-pub fn redact_element(
[0m[31m-    spdf_bytes: &[u8],
[0m[31m-    target_eid: &str,
[0m[31m-    reason: &str,
[0m[31m-) -> SpdfResult<Vec<u8>> {
[0m[32m+pub fn redact_element(spdf_bytes: &[u8], target_eid: &str, reason: &str) -> SpdfResult<Vec<u8>> {
[0m     let extracted = read_container(spdf_bytes)?;
     let mut doc: Document = serde_json::from_slice(&extracted.semantic)?;
 
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core\src\signing.rs:243:
 ///
 /// Validates the transition is allowed, updates the semantic layer,
 /// and appends a `STATE_CHANGED` audit event.
[31m-pub fn transition_document(
[0m[31m-    spdf_bytes: &[u8],
[0m[31m-    target_state: DocumentState,
[0m[31m-) -> SpdfResult<Vec<u8>> {
[0m[32m+pub fn transition_document(spdf_bytes: &[u8], target_state: DocumentState) -> SpdfResult<Vec<u8>> {
[0m     let extracted = read_container(spdf_bytes)?;
     let mut doc: Document = serde_json::from_slice(&extracted.semantic)?;
 
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core\tests\diff_tests.rs:85:
     let report = diff_documents(&bytes_a, &bytes_b).unwrap();
 
     assert_eq!(report.metadata_changes.len(), 1);
[31m-    assert_eq!(
[0m[31m-        report.metadata_changes[0].field.as_deref(),
[0m[31m-        Some("title")
[0m[31m-    );
[0m[32m+    assert_eq!(report.metadata_changes[0].field.as_deref(), Some("title"));
[0m     assert_eq!(report.metadata_changes[0].impact, SemanticImpact::Moderate);
 }
 
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core\tests\signing_tests.rs:240:
 
     let mut buf = Vec::new();
     let mut zip_w = zip::ZipWriter::new(std::io::Cursor::new(&mut buf));
[31m-    let opts =
[0m[31m-        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
[0m[32m+    let opts = zip::write::SimpleFileOptions::default()
[0m[32m+        .compression_method(zip::CompressionMethod::Deflated);
[0m     let stored =
         zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
 
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-wasm\src\lib.rs:18:
 
     let manifest_report = spdf_validator::validate_manifest(&extracted.manifest);
 
[31m-    let doc: Document =
[0m[31m-        serde_json::from_slice(&extracted.semantic).map_err(|e| e.to_string())?;
[0m[32m+    let doc: Document = serde_json::from_slice(&extracted.semantic).map_err(|e| e.to_string())?;
[0m     let document_report = spdf_validator::validate_document(&doc);
 
     let combined = serde_json::json!({
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-wasm\src\lib.rs:36:
 pub fn get_document_info_internal(spdf_bytes: &[u8]) -> Result<String, String> {
     let extracted = container::read_container(spdf_bytes).map_err(|e| e.to_string())?;
 
[31m-    let doc: Document =
[0m[31m-        serde_json::from_slice(&extracted.semantic).map_err(|e| e.to_string())?;
[0m[32m+    let doc: Document = serde_json::from_slice(&extracted.semantic).map_err(|e| e.to_string())?;
[0m 
     let total_elements: usize = doc.pages.iter().map(|p| p.elements.len()).sum();
 
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-wasm\src\lib.rs:72:
 pub fn extract_invoice_internal(spdf_bytes: &[u8]) -> Result<String, String> {
     let extracted = container::read_container(spdf_bytes).map_err(|e| e.to_string())?;
 
[31m-    let doc: Document =
[0m[31m-        serde_json::from_slice(&extracted.semantic).map_err(|e| e.to_string())?;
[0m[32m+    let doc: Document = serde_json::from_slice(&extracted.semantic).map_err(|e| e.to_string())?;
[0m 
     let mut invoice_header = None;
     let mut payment_terms = None;
```

### Rust: cargo clippy
Exit code: 101

```text
cargo :     Checking spdf-core v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core)
At line:1 char:1
+ cargo clippy --workspace -- -D warnings 2>&1
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Checking sp...ates\spdf-core):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   Compiling pyo3-macros v0.22.6
error: unused import: `crate::types::ElementId`
  --> crates\spdf-core\src\diff.rs:10:5
   |
10 | use crate::types::ElementId;
   |     ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(unused_imports)]`

    Checking pyo3 v0.22.6
error: the borrowed expression implements the required traits
   --> crates\spdf-core\src\diff.rs:212:50
    |
212 |             old_value: Some(serde_json::to_value(&doc_a.direction).unwrap_or_default()),
    |                                                  ^^^^^^^^^^^^^^^^ help: change this to: `doc_a.direction`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#needless_borrows_for_generic_args
    = note: `-D clippy::needless-borrows-for-generic-args` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::needless_borrows_for_generic_args)]`

error: the borrowed expression implements the required traits
   --> crates\spdf-core\src\diff.rs:213:50
    |
213 |             new_value: Some(serde_json::to_value(&doc_b.direction).unwrap_or_default()),
    |                                                  ^^^^^^^^^^^^^^^^ help: change this to: `doc_b.direction`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#needless_borrows_for_generic_args

error: the borrowed expression implements the required traits
   --> crates\spdf-core\src\diff.rs:223:50
    |
223 |             old_value: Some(serde_json::to_value(&doc_a.document_state).unwrap_or_default()),
    |                                                  ^^^^^^^^^^^^^^^^^^^^^ help: change this to: `doc_a.document_state`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#needless_borrows_for_generic_args

error: the borrowed expression implements the required traits
   --> crates\spdf-core\src\diff.rs:224:50
    |
224 |             new_value: Some(serde_json::to_value(&doc_b.document_state).unwrap_or_default()),
    |                                                  ^^^^^^^^^^^^^^^^^^^^^ help: change this to: `doc_b.document_state`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#needless_borrows_for_generic_args

error: could not compile `spdf-core` (lib) due to 5 previous errors
warning: build failed, waiting for other jobs to finish...
```

### Rust: cargo test
Exit code: 101

*(truncated to last 100 lines)*

```text
test locale_change_is_minor ... ok
test element_removed_detected ... ok
test element_text_modification_detected ... ok
test element_added_detected ... ok
test identical_documents_no_changes ... ok
test highest_impact_reflects_worst_change ... ok
test summary_counts_correct ... ok
test state_change_is_critical ... ok
test financial_field_change_is_major ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

     Running tests\dom_tests.rs (target\debug\deps\dom_tests-69ee641a033174e2.exe)

running 31 tests
test document_state_transitions ... ok
test document_state_serde_screaming_snake ... ok
test code_block_element_serde ... ok
test annotation_without_target_serde ... ok
test form_field_select_with_options_serde ... ok
test element_id_format ... ok
test document_serialize_deserialize ... ok
test annotation_element_serde ... ok
test form_field_all_types_serde ... ok
test attachment_element_serde ... ok
test form_field_text_serde ... ok
test document_id_format ... ok
test full_invoice_document_serde ... ok
test heading_element_serde ... ok
test horizontal_rule_element_serde ... ok
test image_element_serde ... ok
test invoice_header_element_serde ... ok
test line_item_table_element_serde ... ok
test multi_page_document_serde ... ok
test page_break_element_serde ... ok
test paragraph_element_serde ... ok
test payment_terms_element_serde ... ok
test source_format_serde ... ok
test signature_block_element_serde ... ok
test redaction_element_serde ... ok
test spdf_version_display ... ok
test stamp_element_serde ... ok
test table_element_serde ... ok
test text_direction_serde ... ok
test variable_placeholder_element_serde ... ok
test vector_image_element_serde ... ok

test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests\redaction_tests.rs (target\debug\deps\redaction_tests-228bd204f5ed5be9.exe)

running 11 tests
test list_redactions_empty_on_clean_doc ... ok
test redact_nonexistent_element_fails ... ok
test verify_redaction_not_found ... ok
test redact_element_succeeds ... ok
test redact_appends_audit_event ... ok
test redacted_element_replaced_with_redaction ... ok
test list_redactions_shows_redacted_elements ... ok
test redact_preserves_other_elements ... ok
test redacted_element_has_proof_hash ... ok
test verify_redaction_found ... ok
test multiple_redactions ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

     Running tests\signing_tests.rs (target\debug\deps\signing_tests-76c4fb2b310ba9dd.exe)

running 15 tests
test sign_draft_document_fails_wrong_state ... ok
test transition_invalid_state_fails ... ok
test transition_appends_audit_event ... ok
test signature_record_has_correct_fields ... ok
test signed_document_state_is_signed ... ok
test sign_review_document_succeeds ... ok
test signed_document_has_signature_entry ... ok
test signed_document_locks_signature_blocks ... ok
test transition_draft_to_review ... ok
test verify_reports_signer_details ... ok
test signed_document_has_audit_event ... ok
test sign_already_signed_fails ... ok
test verify_unsigned_document_reports_no_signatures ... ok
test verify_signed_document_is_valid ... FAILED
test verify_tampered_document_detects_tampering ... ok

failures:

---- verify_signed_document_is_valid stdout ----

thread 'verify_signed_document_is_valid' (18892) panicked at crates\spdf-core\tests\signing_tests.rs:199:5:
assertion failed: report.valid
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    verify_signed_document_is_valid

test result: FAILED. 14 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

error: test failed, to rerun pass `-p spdf-core --test signing_tests`
```

### Python: pytest
Exit code: 1

*(truncated to last 100 lines)*

```text
api\tests\test_templates.py::test_create_template_missing_name_returns_422 PASSED [ 85%]
api\tests\test_templates.py::test_create_template_empty_name_returns_422 PASSED [ 86%]
api\tests\test_templates.py::test_create_template_401_without_auth PASSED [ 87%]
api\tests\test_templates.py::test_get_template_by_id PASSED              [ 87%]
api\tests\test_templates.py::test_get_template_not_found PASSED          [ 88%]
api\tests\test_templates.py::test_list_templates_empty PASSED            [ 89%]
api\tests\test_templates.py::test_list_templates_returns_items PASSED    [ 89%]
api\tests\test_templates.py::test_list_templates_pagination PASSED       [ 90%]
api\tests\test_templates.py::test_update_template_name PASSED            [ 91%]
api\tests\test_templates.py::test_update_template_not_found PASSED       [ 91%]
api\tests\test_templates.py::test_delete_template PASSED                 [ 92%]
api\tests\test_templates.py::test_delete_template_not_found PASSED       [ 92%]
api\tests\test_templates.py::test_deleted_template_excluded_from_list PASSED [ 93%]
api\tests\test_webhooks.py::test_subscription_created_updates_tier PASSED [ 94%]
api\tests\test_webhooks.py::test_subscription_updated_changes_plan PASSED [ 94%]
api\tests\test_webhooks.py::test_subscription_deleted_reverts_to_free PASSED [ 95%]
api\tests\test_webhooks.py::test_invoice_paid_sets_active PASSED         [ 96%]
api\tests\test_webhooks.py::test_invoice_payment_failed_sets_past_due PASSED [ 96%]
api\tests\test_webhooks.py::test_unknown_event_type_returns_200 PASSED   [ 97%]
api\tests\test_webhooks.py::test_invalid_json_returns_400 PASSED         [ 98%]
api\tests\test_webhooks.py::test_missing_event_type_returns_400 PASSED   [ 98%]
api\tests\test_webhooks.py::test_webhook_does_not_require_bearer_auth PASSED [ 99%]
api\tests\test_webhooks.py::test_duplicate_event_is_idempotent PASSED    [100%]

================================== FAILURES ===================================
______________________ test_request_id_on_error_response ______________________
api\tests\test_hardening.py:37: in test_request_id_on_error_response
    assert "request_id" in body
E   AssertionError: assert 'request_id' in {'detail': 'Missing or malformed Authorization header.', 'error': 'UNAUTHORIZED'}
________________________ test_request_id_on_auth_error ________________________
api\tests\test_hardening.py:47: in test_request_id_on_auth_error
    assert "x-request-id" in resp.headers
E   AssertionError: assert 'x-request-id' in Headers({'content-type': 'application/json', 'content-length': '59'})
E    +  where Headers({'content-type': 'application/json', 'content-length': '59'}) = <Response [401 Unauthorized]>.headers
__________________ test_error_response_has_request_id_field ___________________
api\tests\test_hardening.py:91: in test_error_response_has_request_id_field
    assert "request_id" in body
E   AssertionError: assert 'request_id' in {'detail': 'Missing or malformed Authorization header.', 'error': 'UNAUTHORIZED'}
____________________ test_rate_limit_error_has_request_id _____________________
api\tests\test_hardening.py:111: in test_rate_limit_error_has_request_id
    assert "request_id" in body
E   AssertionError: assert 'request_id' in {'detail': 'Rate limit exceeded for other. Resets at UTC midnight.', 'error': 'RATE_LIMIT_EXCEEDED'}
_____________________________ test_redact_element _____________________________
api\tests\test_redaction.py:36: in test_redact_element
    spdf_bytes, eid = _get_element_eid(client, sample_semantic, "Paragraph")
                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
api\tests\test_redaction.py:25: in _get_element_eid
    raise ValueError(f"No {element_type} found in fixture")
E   ValueError: No Paragraph found in fixture
_________________________ test_verify_signed_document _________________________
api\tests\test_signing.py:92: in test_verify_signed_document
    assert report["valid"] is True
E   assert False is True
_____________________ test_verify_returns_signer_details ______________________
api\tests\test_signing.py:131: in test_verify_returns_signer_details
    assert sig["valid"] is True
E   assert False is True
______________________ test_sign_then_verify_round_trip _______________________
api\tests\test_signing.py:212: in test_sign_then_verify_round_trip
    assert report["valid"] is True
E   assert False is True
============================== warnings summary ===============================
tests/test_e2e.py::test_jwt_auth_template_crud
tests/test_jwt_auth.py::test_jwt_auth_returns_200
tests/test_jwt_auth.py::test_jwt_auth_resolves_correct_user
tests/test_jwt_auth.py::test_jwt_auth_rate_limit_headers
tests/test_jwt_auth.py::test_both_auth_methods_on_same_endpoint
tests/test_jwt_auth.py::test_expired_jwt_returns_401
tests/test_jwt_auth.py::test_unknown_email_returns_401
tests/test_jwt_auth.py::test_wrong_issuer_returns_401
  D:\SPDF DEVELOPMENT\SPDF\.venv\Lib\site-packages\jwt\api_jwt.py:147: InsecureKeyLengthWarning: The HMAC key is 20 bytes long, which is below the minimum recommended length of 32 bytes for SHA256. See RFC 7518 Section 3.2.
    return self._jws.encode(

tests/test_e2e.py::test_jwt_auth_template_crud
tests/test_jwt_auth.py::test_jwt_auth_returns_200
tests/test_jwt_auth.py::test_jwt_auth_resolves_correct_user
tests/test_jwt_auth.py::test_jwt_auth_rate_limit_headers
tests/test_jwt_auth.py::test_both_auth_methods_on_same_endpoint
tests/test_jwt_auth.py::test_expired_jwt_returns_401
tests/test_jwt_auth.py::test_bad_signature_returns_401
tests/test_jwt_auth.py::test_unknown_email_returns_401
tests/test_jwt_auth.py::test_wrong_issuer_returns_401
  D:\SPDF DEVELOPMENT\SPDF\.venv\Lib\site-packages\jwt\api_jwt.py:365: InsecureKeyLengthWarning: The HMAC key is 20 bytes long, which is below the minimum recommended length of 32 bytes for SHA256. See RFC 7518 Section 3.2.
    decoded = self.decode_complete(

tests/test_jwt_auth.py::test_bad_signature_returns_401
  D:\SPDF DEVELOPMENT\SPDF\.venv\Lib\site-packages\jwt\api_jwt.py:147: InsecureKeyLengthWarning: The HMAC key is 12 bytes long, which is below the minimum recommended length of 32 bytes for SHA256. See RFC 7518 Section 3.2.
    return self._jws.encode(

-- Docs: https://docs.pytest.org/en/stable/how-to/capture-warnings.html
=========================== short test summary info ===========================
FAILED api\tests\test_hardening.py::test_request_id_on_error_response - Asser...
FAILED api\tests\test_hardening.py::test_request_id_on_auth_error - Assertion...
FAILED api\tests\test_hardening.py::test_error_response_has_request_id_field
FAILED api\tests\test_hardening.py::test_rate_limit_error_has_request_id - As...
FAILED api\tests\test_redaction.py::test_redact_element - ValueError: No Para...
FAILED api\tests\test_signing.py::test_verify_signed_document - assert False ...
FAILED api\tests\test_signing.py::test_verify_returns_signer_details - assert...
FAILED api\tests\test_signing.py::test_sign_then_verify_round_trip - assert F...
================= 8 failed, 149 passed, 18 warnings in 9.39s ==================
```
