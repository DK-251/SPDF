# Build Status

## Last Run
- **Version:** 0.1.0-snapshot.1
- **Date:** 2026-03-25T13:56:59Z
- **Commit:** a28f62e
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
```n   Compiling encoding_rs v0.8.35
   Compiling nom v7.1.3
   Compiling rayon v1.11.0
   Compiling rangemap v1.7.1
   Compiling weezl v0.1.12
   Compiling unindent v0.2.4
   Compiling indoc v2.0.7
   Compiling wasm-bindgen-macro-support v0.2.114
   Compiling bzip2 v0.5.2
   Compiling serde_derive v1.0.228
   Compiling thiserror-impl v2.0.18
   Compiling zeroize_derive v1.4.3
   Compiling displaydoc v0.2.5
   Compiling zeroize v1.8.2
   Compiling zstd v0.13.3
   Compiling wasm-bindgen-macro v0.2.114
   Compiling pyo3-macros v0.22.6
   Compiling xz2 v0.1.7
   Compiling chrono v0.4.44
   Compiling js-sys v0.3.91
   Compiling spdf-core v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core)
   Compiling lopdf v0.34.0
error[E0283]: type annotations needed for `FileOptions<'_, _>`
   --> crates\spdf-core\src\container.rs:59:9
    |
 59 |     let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    |         ^^^^^^^   ----------- type must be known at this point
    |
    = note: cannot satisfy `_: FileOptionExtension`
help: the following types implement trait `FileOptionExtension`
   --> C:\Users\mrdee\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\zip-2.4.2\src\write.rs:210:5
    |
210 |     impl FileOptionExtension for () {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `()`
...
220 |     impl FileOptionExtension for ExtendedFileOptions {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ExtendedFileOptions`
note: required by a bound in `FileOptions`
   --> C:\Users\mrdee\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\zip-2.4.2\src\write.rs:260:31
    |
260 | pub struct FileOptions<'k, T: FileOptionExtension> {
    |                               ^^^^^^^^^^^^^^^^^^^ required by this bound in `FileOptions`
help: consider giving `options` an explicit type, where the type for type parameter `T` is specified
    |
 59 |     let options: FileOptions<'_, T> = FileOptions::default().compression_method(CompressionMethod::Deflated);
    |                ++++++++++++++++++++

For more information about this error, try `rustc --explain E0283`.
error: could not compile `spdf-core` (lib) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
```

### rust-clippy.log
```n   Compiling bzip2-sys v0.1.13+1.0.8
   Compiling lzma-sys v0.1.20
    Checking bzip2 v0.5.2
   Compiling wasm-bindgen-macro-support v0.2.114
   Compiling zstd-safe v7.2.4
    Checking zstd v0.13.3
   Compiling serde_derive v1.0.228
   Compiling zeroize_derive v1.4.3
   Compiling thiserror-impl v2.0.18
   Compiling displaydoc v0.2.5
    Checking zeroize v1.8.2
    Checking xz2 v0.1.7
    Checking thiserror v2.0.18
    Checking zip v2.4.2
   Compiling wasm-bindgen-macro v0.2.114
   Compiling pyo3-macros v0.22.6
    Checking wasm-bindgen v0.2.114
    Checking serde v1.0.228
    Checking chrono v0.4.44
    Checking js-sys v0.3.91
    Checking spdf-core v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core)
    Checking lopdf v0.34.0
error[E0283]: type annotations needed for `zip::write::FileOptions<'_, _>`
   --> crates\spdf-core\src\container.rs:59:9
    |
 59 |     let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    |         ^^^^^^^   ----------- type must be known at this point
    |
    = note: cannot satisfy `_: zip::write::FileOptionExtension`
help: the following types implement trait `zip::write::FileOptionExtension`
   --> C:\Users\mrdee\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\zip-2.4.2\src\write.rs:210:5
    |
210 |     impl FileOptionExtension for () {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `()`
...
220 |     impl FileOptionExtension for ExtendedFileOptions {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `zip::write::ExtendedFileOptions`
note: required by a bound in `zip::write::FileOptions`
   --> C:\Users\mrdee\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\zip-2.4.2\src\write.rs:260:31
    |
260 | pub struct FileOptions<'k, T: FileOptionExtension> {
    |                               ^^^^^^^^^^^^^^^^^^^ required by this bound in `FileOptions`
help: consider giving `options` an explicit type, where the type for type parameter `T` is specified
    |
 59 |     let options: zip::write::FileOptions<'_, T> = FileOptions::default().compression_method(CompressionMethod::Deflated);
    |                ++++++++++++++++++++++++++++++++

For more information about this error, try `rustc --explain E0283`.
error: could not compile `spdf-core` (lib) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
```

### rust-test.log
```ncargo.exe :    Compiling spdf-core v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core)
At D:\SPDF DEVELOPMENT\SPDF\scripts\build-status.ps1:34 char:9
+         & $args_list[0] $args_list[1..($args_list.Length-1)] *> $logP ...
+         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling sp...ates\spdf-core):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   Compiling windows-sys v0.61.2
   Compiling fastrand v2.3.0
   Compiling serde-wasm-bindgen v0.6.5
error[E0283]: type annotations needed for `FileOptions<'_, _>`
   --> crates\spdf-core\src\container.rs:59:9
    |
 59 |     let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    |         ^^^^^^^   ----------- type must be known at this point
    |
    = note: cannot satisfy `_: FileOptionExtension`
help: the following types implement trait `FileOptionExtension`
   --> C:\Users\mrdee\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\zip-2.4.2\src\write.rs:210:5
    |
210 |     impl FileOptionExtension for () {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `()`
...
220 |     impl FileOptionExtension for ExtendedFileOptions {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ExtendedFileOptions`
note: required by a bound in `FileOptions`
   --> C:\Users\mrdee\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\zip-2.4.2\src\write.rs:260:31
    |
260 | pub struct FileOptions<'k, T: FileOptionExtension> {
    |                               ^^^^^^^^^^^^^^^^^^^ required by this bound in `FileOptions`
help: consider giving `options` an explicit type, where the type for type parameter `T` is specified
    |
 59 |     let options: FileOptions<'_, T> = FileOptions::default().compression_method(CompressionMethod::Deflated);
    |                ++++++++++++++++++++

   Compiling tempfile v3.27.0
For more information about this error, try `rustc --explain E0283`.
error: could not compile `spdf-core` (lib) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
```

