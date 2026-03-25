# Build Status

## Last Run
- **Version:** 0.1.0-snapshot.2
- **Date:** 2026-03-25T14:41:59Z
- **Commit:** ac927be
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
```ncargo.exe :    Compiling spdf-validator v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-validator)
At D:\SPDF DEVELOPMENT\SPDF\scripts\build-status.ps1:34 char:9
+         & $args_list[0] $args_list[1..($args_list.Length-1)] *> $logP ...
+         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling sp...spdf-validator):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   Compiling spdf-renderer v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer)
error[E0308]: mismatched types
   --> crates\spdf-validator\src\rules.rs:164:45
    |
164 |             check_table_headers(&t.headers, &t.rows.iter().map(|r| &r.cells).collect::<Vec<_>>(), path, errors);
    |             -------------------             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&[&[TableCell]]`, found 
`&Vec<&Vec<TableCell>>`
    |             |
    |             arguments to this function are incorrect
    |
    = note: expected reference `&[&[spdf_core::dom::TableCell]]`
               found reference `&Vec<&Vec<spdf_core::dom::TableCell>>`
note: function defined here
   --> crates\spdf-validator\src\rules.rs:221:4
    |
221 | fn check_table_headers(
    |    ^^^^^^^^^^^^^^^^^^^
222 |     headers: &[String],
223 |     rows: &[&[TableCell]],
    |     ---------------------

For more information about this error, try `rustc --explain E0308`.
error: could not compile `spdf-validator` (lib) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
```

### rust-clippy.log
```n    |    ^^^^^^^^^^^^^^^^^^^
222 |     headers: &[String],
223 |     rows: &[&[TableCell]],
    |     ---------------------

error: this can be `std::io::Error::other(_)`
  --> crates\spdf-renderer\src\pdf.rs:63:40
   |
63 |             .map_err(|e| SpdfError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#io_other_error
   = note: `-D clippy::io-other-error` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::io_other_error)]`
help: use `std::io::Error::other`
   |
63 -             .map_err(|e| SpdfError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
63 +             .map_err(|e| SpdfError::Io(std::io::Error::other(e.to_string())))?;
   |

error: this can be `std::io::Error::other(_)`
  --> crates\spdf-renderer\src\pdf.rs:81:40
   |
81 |             .map_err(|e| SpdfError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#io_other_error
help: use `std::io::Error::other`
   |
81 -             .map_err(|e| SpdfError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
81 +             .map_err(|e| SpdfError::Io(std::io::Error::other(e.to_string())))?;
   |

error: this can be `std::io::Error::other(_)`
   --> crates\spdf-renderer\src\pdf.rs:109:36
    |
109 |         .map_err(|e| SpdfError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
    |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#io_other_error
help: use `std::io::Error::other`
    |
109 -         .map_err(|e| SpdfError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
109 +         .map_err(|e| SpdfError::Io(std::io::Error::other(e.to_string())))?;
    |

error: could not compile `spdf-renderer` (lib) due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
For more information about this error, try `rustc --explain E0308`.
error: could not compile `spdf-validator` (lib) due to 1 previous error
```

### rust-fmt.log
```n[0m[32m+            name: "test".to_string(),
[0m[32m+            version: "0.1.0".to_string(),
[0m[32m+        },
[0m[32m+    );
[0m     manifest.format = "PDF".to_string();
     manifest.finalize();
 
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-validator\tests\validator_tests.rs:306:
 
 #[test]
 fn f004_empty_layer_checksum() {
[31m-    let manifest = Manifest::new(DocumentId::new(), GeneratorInfo {
[0m[31m-        name: "test".to_string(),
[0m[31m-        version: "0.1.0".to_string(),
[0m[31m-    });
[0m[32m+    let manifest = Manifest::new(
[0m[32m+        DocumentId::new(),
[0m[32m+        GeneratorInfo {
[0m[32m+            name: "test".to_string(),
[0m[32m+            version: "0.1.0".to_string(),
[0m[32m+        },
[0m[32m+    );
[0m     // All layer checksums are empty by default (no finalize)
     let report = validate_manifest(&manifest);
     assert!(report.errors.iter().any(|e| e.code == "F_004"));
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-validator\tests\validator_tests.rs:316:
[31m-    assert_eq!(report.errors.iter().filter(|e| e.code == "F_004").count(), 6);
[0m[32m+    assert_eq!(
[0m[32m+        report.errors.iter().filter(|e| e.code == "F_004").count(),
[0m[32m+        6
[0m[32m+    );
[0m }
 
 #[test]
Diff in \\?\D:\SPDF DEVELOPMENT\SPDF\crates\spdf-validator\tests\validator_tests.rs:320:
 fn f005_empty_manifest_hash() {
[31m-    let mut manifest = Manifest::new(DocumentId::new(), GeneratorInfo {
[0m[31m-        name: "test".to_string(),
[0m[31m-        version: "0.1.0".to_string(),
[0m[31m-    });
[0m[32m+    let mut manifest = Manifest::new(
[0m[32m+        DocumentId::new(),
[0m[32m+        GeneratorInfo {
[0m[32m+            name: "test".to_string(),
[0m[32m+            version: "0.1.0".to_string(),
[0m[32m+        },
[0m[32m+    );
[0m     manifest.layers.semantic = "a".repeat(64);
     manifest.layers.layout = "b".repeat(64);
     manifest.layers.styles = "c".repeat(64);
```

### rust-test.log
```n164 |             check_table_headers(&t.headers, &t.rows.iter().map(|r| &r.cells).collect::<Vec<_>>(), path, errors);
    |             -------------------             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&[&[TableCell]]`, found 
`&Vec<&Vec<TableCell>>`
    |             |
    |             arguments to this function are incorrect
    |
    = note: expected reference `&[&[spdf_core::dom::TableCell]]`
               found reference `&Vec<&Vec<spdf_core::dom::TableCell>>`
note: function defined here
   --> crates\spdf-validator\src\rules.rs:221:4
    |
221 | fn check_table_headers(
    |    ^^^^^^^^^^^^^^^^^^^
222 |     headers: &[String],
223 |     rows: &[&[TableCell]],
    |     ---------------------

warning: unused import: `ExtractedLayers`
 --> crates\spdf-core\tests\container_tests.rs:3:78
  |
3 | use spdf_core::container::{read_container, write_container, ContainerLayers, ExtractedLayers};
  |                                                                              ^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

For more information about this error, try `rustc --explain E0308`.
error: could not compile `spdf-validator` (lib) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
error[E0277]: `ExtractedLayers` doesn't implement `Debug`
   --> crates\spdf-core\tests\container_tests.rs:158:36
    |
158 |     let err = format!("{}", result.unwrap_err());
    |                                    ^^^^^^^^^^ the trait `Debug` is not implemented for `ExtractedLayers`
    |
note: required by a bound in `Result::<T, E>::unwrap_err`
   --> /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library\core\src\result.rs:1324:4

error[E0277]: `ExtractedLayers` doesn't implement `Debug`
   --> crates\spdf-core\tests\container_tests.rs:203:36
    |
203 |     let err = format!("{}", result.unwrap_err());
    |                                    ^^^^^^^^^^ the trait `Debug` is not implemented for `ExtractedLayers`
    |
note: required by a bound in `Result::<T, E>::unwrap_err`
   --> /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library\core\src\result.rs:1324:4

error: could not compile `spdf-validator` (lib test) due to 1 previous error
For more information about this error, try `rustc --explain E0277`.
warning: `spdf-core` (test "container_tests") generated 1 warning
error: could not compile `spdf-core` (test "container_tests") due to 2 previous errors; 1 warning emitted
```

