# Build Status

## Last Run
- **Version:** 0.1.0-snapshot.1
- **Date:** 2026-03-25T14:04:35Z
- **Commit:** d64deba
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
```ncargo.exe :    Compiling spdf-core v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core)
At D:\SPDF DEVELOPMENT\SPDF\scripts\build-status.ps1:34 char:9
+         & $args_list[0] $args_list[1..($args_list.Length-1)] *> $logP ...
+         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling sp...ates\spdf-core):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   Compiling spdf-validator v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-validator)
   Compiling spdf-renderer v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer)
   Compiling spdf-python v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python)
   Compiling spdf-wasm v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-wasm)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 10.32s
```

### rust-clippy.log
```ncargo.exe :     Checking spdf-core v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core)
At D:\SPDF DEVELOPMENT\SPDF\scripts\build-status.ps1:34 char:9
+         & $args_list[0] $args_list[1..($args_list.Length-1)] *> $logP ...
+         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Checking sp...ates\spdf-core):String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
    Checking serde-wasm-bindgen v0.6.5
    Checking spdf-renderer v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer)
    Checking spdf-validator v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-validator)
    Checking spdf-wasm v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-wasm)
    Checking spdf-python v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.23s
```

### rust-test.log
```ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\lib.rs (target\debug\deps\spdf_python-7aa428dc40f62cd4.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\lib.rs (target\debug\deps\spdf_renderer-65dc6ee0c25dfa72.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\lib.rs (target\debug\deps\spdf_validator-f607e20da6fa05a5.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\lib.rs (target\debug\deps\spdf_wasm-4988da22b8988948.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests spdf_core

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests spdf_renderer

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests spdf_validator

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests spdf_wasm

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

