# Build Status

## Last Run
- **Version:** 0.1.0-snapshot.2
- **Date:** 2026-03-25T14:59:54Z
- **Commit:** e2d0b50
- **Branch:** main
- **Machine:** TUF_WARRIOR_DK

## Rust Core (crates/)
- [x] cargo build: PASS
- [x] cargo test: PASS
- [x] cargo clippy: PASS
- [x] cargo fmt --check: PASS

## Python API (api/)
- [-] pip install: SKIP
- [-] pytest: SKIP

## Studio Frontend (studio/)
- [-] npm install: SKIP
- [-] npm build: SKIP

## Error Logs
### rust-build.log
```ncargo.exe :    Compiling spdf-python v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python)
At D:\SPDF DEVELOPMENT\SPDF\scripts\build-status.ps1:34 char:9
+         & $args_list[0] $args_list[1..($args_list.Length-1)] *> $logP ...
+         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling sp...es\spdf-python):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.99s
```

### rust-clippy.log
```ncargo.exe :     Checking spdf-validator v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-validator)
At D:\SPDF DEVELOPMENT\SPDF\scripts\build-status.ps1:34 char:9
+         & $args_list[0] $args_list[1..($args_list.Length-1)] *> $logP ...
+         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
```

### rust-fmt.log
```n             }
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
```

### rust-test.log
```n    + FullyQualifiedErrorId : NativeCommandError
 
 --> crates\spdf-validator\tests\validator_tests.rs:4:60
  |
4 | use spdf_validator::{validate_document, validate_manifest, Severity};
  |                                                            ^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `spdf-validator` (test "validator_tests") generated 1 warning (run `cargo fix --test "validator_tests" -p spdf-validator` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.71s
     Running unittests src\lib.rs (target\debug\deps\spdf_core-c9d433ce6c9b1045.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\container_tests.rs (target\debug\deps\container_tests-0798c7706bb5367a.exe)

running 14 tests
test read_invalid_zip_data ... ok
test read_zip_missing_manifest ... ok
test read_zip_missing_layer ... ok
test document_id_preserved ... ok
test manifest_hash_is_populated ... ok
test manifest_format_and_version ... ok
test write_then_read_round_trip ... ok
test manifest_checksums_match_layer_content ... ok
test empty_layers_round_trip ... ok
test round_trip_with_single_asset ... ok
test round_trip_with_multiple_assets ... ok
test corrupted_layer_detected ... ok
test same_input_produces_same_checksums ... ok
test large_layer_round_trip ... FAILED

failures:

---- large_layer_round_trip stdout ----

thread 'large_layer_round_trip' (12316) panicked at crates\spdf-core\tests\container_tests.rs:247:44:
called `Result::unwrap()` on an `Err` value: DecompressionBomb { ratio: 461.9044894366197, max: 100.0 }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    large_layer_round_trip

test result: FAILED. 13 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s

error: test failed, to rerun pass `-p spdf-core --test container_tests`
```

