# Check Results

## Run Info
- **Version:** 0.1.0-snapshot.2
- **Commit:** e2d0b50
- **Branch:** main
- **Date:** 2026-03-25T15:00:29Z
- **Machine:** TUF_WARRIOR_DK
- **Overall:** FAIL

## Steps
- [ ] cargo fmt --check: FAIL
- [ ] cargo clippy: FAIL
- [ ] cargo test: FAIL

### cargo fmt --check (last 80 lines)
````
[0m[32m+            .count()
[0m     }
 }
 
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-validator\src\rules.rs:29:
 use crate::{Severity, ValidationError};
 
 fn fatal(code: &'static str, message: String, path: Option<String>) -> ValidationError {
[31m-    ValidationError { code, severity: Severity::Fatal, message, path }
[0m[32m+    ValidationError {
[0m[32m+        code,
[0m[32m+        severity: Severity::Fatal,
[0m[32m+        message,
[0m[32m+        path,
[0m[32m+    }
[0m }
 
 fn error(code: &'static str, message: String, path: Option<String>) -> ValidationError {
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-validator\src\rules.rs:36:
[31m-    ValidationError { code, severity: Severity::Error, message, path }
[0m[32m+    ValidationError {
[0m[32m+        code,
[0m[32m+        severity: Severity::Error,
[0m[32m+        message,
[0m[32m+        path,
[0m[32m+    }
[0m }
 
 pub fn run_document_rules(doc: &Document, errors: &mut Vec<ValidationError>) {
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-validator\src\rules.rs:161:
             }
         }
         Element::Table(t) => {
[31m-            let row_slices: Vec<&[TableCell]> =
[0m[31m-                t.rows.iter().map(|r| r.cells.as_slice()).collect();
[0m[32m+            let row_slices: Vec<&[TableCell]> = t.rows.iter().map(|r| r.cells.as_slice()).collect();
[0m             check_table_headers(&t.headers, &row_slices, path, errors);
         }
         Element::InvoiceHeader(ih) => {
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-validator\src\rules.rs:175:
             }
         }
         Element::LineItemTable(lt) => {
[31m-            let row_slices: Vec<&[TableCell]> =
[0m[31m-                lt.rows.iter().map(|r| r.as_slice()).collect();
[0m[32m+            let row_slices: Vec<&[TableCell]> = lt.rows.iter().map(|r| r.as_slice()).collect();
[0m             check_table_headers(&lt.headers, &row_slices, path, errors);
         }
         Element::PaymentTerms(pt) => {
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-validator\tests\validator_tests.rs:171:
         eid: eid(),
         headers: vec!["A".into(), "B".into()],
         rows: vec![TableRow {
[31m-            cells: vec![TableCell { value: "only one".into(), spdf_type: None }],
[0m[32m+            cells: vec![TableCell {
[0m[32m+                value: "only one".into(),
[0m[32m+                spdf_type: None,
[0m[32m+            }],
[0m         }],
         timestamps: ts(),
     })];
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-validator\tests\validator_tests.rs:187:
         invoice_number: "".to_string(),
         issue_date: "2026-01-01".to_string(),
         due_date: "2026-02-01".to_string(),
[31m-        vendor: PartyInfo { name: "V".into(), address: None, gstin: None },
[0m[31m-        client: PartyInfo { name: "C".into(), address: None, gstin: None },
[0m[32m+        vendor: PartyInfo {
[0m[32m+            name: "V".into(),
[0m[32m+            address: None,
[0m[32m+            gstin: None,
[0m[32m+        },
[0m[32m+        client: PartyInfo {
[0m[32m+            name: "C".into(),
[0m[32m+            address: None,
[0m[32m+            gstin: None,
[0m[32m+        },
[0m         currency: None,
         timestamps: ts(),
     })];
`````n

### cargo clippy (last 80 lines)
````
cargo :     Checking spdf-validator v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-validator)
At line:1 char:1
+ cargo clippy --workspace -- -D warnings *> ".build-results\check-clip ...
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Checking sp...spdf-validator):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
error: this `map_or` can be simplified
   --> crates\spdf-validator\src\rules.rs:202:35
    |
202 |                 let has_options = f.options.as_ref().map_or(false, |o| !o.is_empty());
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#unnecessary_map_or
    = note: `-D clippy::unnecessary-map-or` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::unnecessary_map_or)]`
help: use is_some_and instead
    |
202 -                 let has_options = f.options.as_ref().map_or(false, |o| !o.is_empty());
202 +                 let has_options = f.options.as_ref().is_some_and(|o| !o.is_empty());
    |

error: could not compile `spdf-validator` (lib) due to 1 previous error
`````n

### cargo test (last 80 lines)
````
cargo : warning: unused import: `Severity`
At line:1 char:1
+ cargo test --workspace *> ".build-results\check-test.log" 2>&1
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: unused import: `Severity`:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
 --> crates\spdf-validator\tests\validator_tests.rs:4:60
  |
4 | use spdf_validator::{validate_document, validate_manifest, Severity};
  |                                                            ^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `spdf-validator` (test "validator_tests") generated 1 warning (run `cargo fix --test "validator_tests" -p spdf-validator` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.92s
     Running unittests src\lib.rs (target\debug\deps\spdf_core-c9d433ce6c9b1045.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\container_tests.rs (target\debug\deps\container_tests-0798c7706bb5367a.exe)

running 14 tests
test read_invalid_zip_data ... ok
test read_zip_missing_manifest ... ok
test manifest_hash_is_populated ... ok
test document_id_preserved ... ok
test read_zip_missing_layer ... ok
test manifest_format_and_version ... ok
test write_then_read_round_trip ... ok
test manifest_checksums_match_layer_content ... ok
test round_trip_with_single_asset ... ok
test empty_layers_round_trip ... ok
test round_trip_with_multiple_assets ... ok
test corrupted_layer_detected ... ok
test same_input_produces_same_checksums ... ok
test large_layer_round_trip ... FAILED

failures:

---- large_layer_round_trip stdout ----

thread 'large_layer_round_trip' (17080) panicked at crates\spdf-core\tests\container_tests.rs:247:44:
called `Result::unwrap()` on an `Err` value: DecompressionBomb { ratio: 462.10788199031265, max: 100.0 }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    large_layer_round_trip

test result: FAILED. 13 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s

error: test failed, to rerun pass `-p spdf-core --test container_tests`
`````n
