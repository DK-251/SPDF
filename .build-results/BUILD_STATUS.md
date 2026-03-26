# Build Status

## Last Run
- **Version:** 0.1.0-snapshot.13
- **Date:** 2026-03-26T07:16:44Z
- **Commit:** 9db24da
- **Branch:** main
- **Machine:** TUF_WARRIOR_DK

## Rust Core (crates/)
- [x] cargo build: PASS
- [x] cargo test: PASS
- [x] cargo clippy: PASS
- [x] cargo fmt --check: PASS

## Python API (api/)
- [x] pip install: PASS
- [x] pytest: PASS

## Studio Frontend (studio/)
- [-] npm install: SKIP
- [-] npm build: SKIP

## Error Logs
### python-install.log
```n  Using cached bcrypt-5.0.0-cp39-abi3-win_amd64.whl.metadata (10 kB)
Collecting PyJWT>=2.8.0 (from spdf-api==0.1.0)
  Using cached pyjwt-2.12.1-py3-none-any.whl.metadata (4.1 kB)
Requirement already satisfied: pytest>=8.0 in d:\tools\lib\site-packages (from spdf-api==0.1.0) (9.0.2)
Requirement already satisfied: httpx>=0.27.0 in d:\tools\lib\site-packages (from spdf-api==0.1.0) (0.28.1)
Requirement already satisfied: starlette>=0.46.0 in d:\tools\lib\site-packages (from fastapi>=0.115.0->spdf-api==0.1.0) (1.0.0)
Requirement already satisfied: typing-extensions>=4.8.0 in d:\tools\lib\site-packages (from fastapi>=0.115.0->spdf-api==0.1.0) (4.15.0)
Requirement already satisfied: typing-inspection>=0.4.2 in d:\tools\lib\site-packages (from fastapi>=0.115.0->spdf-api==0.1.0) (0.4.2)
Requirement already satisfied: annotated-doc>=0.0.2 in d:\tools\lib\site-packages (from fastapi>=0.115.0->spdf-api==0.1.0) (0.0.4)
Requirement already satisfied: anyio in d:\tools\lib\site-packages (from httpx>=0.27.0->spdf-api==0.1.0) (4.13.0)
Requirement already satisfied: certifi in d:\tools\lib\site-packages (from httpx>=0.27.0->spdf-api==0.1.0) (2026.2.25)
Requirement already satisfied: httpcore==1.* in d:\tools\lib\site-packages (from httpx>=0.27.0->spdf-api==0.1.0) (1.0.9)
Requirement already satisfied: idna in d:\tools\lib\site-packages (from httpx>=0.27.0->spdf-api==0.1.0) (3.11)
Requirement already satisfied: h11>=0.16 in d:\tools\lib\site-packages (from httpcore==1.*->httpx>=0.27.0->spdf-api==0.1.0) (0.16.0)
Requirement already satisfied: annotated-types>=0.6.0 in d:\tools\lib\site-packages (from pydantic>=2.10.0->spdf-api==0.1.0) (0.7.0)
Requirement already satisfied: pydantic-core==2.41.5 in d:\tools\lib\site-packages (from pydantic>=2.10.0->spdf-api==0.1.0) (2.41.5)
Requirement already satisfied: colorama>=0.4 in d:\tools\lib\site-packages (from pytest>=8.0->spdf-api==0.1.0) (0.4.6)
Requirement already satisfied: iniconfig>=1.0.1 in d:\tools\lib\site-packages (from pytest>=8.0->spdf-api==0.1.0) (2.3.0)
Requirement already satisfied: packaging>=22 in d:\tools\lib\site-packages (from pytest>=8.0->spdf-api==0.1.0) (26.0)
Requirement already satisfied: pluggy<2,>=1.5 in d:\tools\lib\site-packages (from pytest>=8.0->spdf-api==0.1.0) (1.6.0)
Requirement already satisfied: pygments>=2.7.2 in d:\tools\lib\site-packages (from pytest>=8.0->spdf-api==0.1.0) (2.19.2)
Requirement already satisfied: click>=7.0 in d:\tools\lib\site-packages (from uvicorn>=0.32.0->uvicorn[standard]>=0.32.0->spdf-api==0.1.0) (8.3.1)
Requirement already satisfied: httptools>=0.6.3 in d:\tools\lib\site-packages (from uvicorn[standard]>=0.32.0->spdf-api==0.1.0) (0.7.1)
Requirement already satisfied: python-dotenv>=0.13 in d:\tools\lib\site-packages (from uvicorn[standard]>=0.32.0->spdf-api==0.1.0) (1.2.2)
Requirement already satisfied: pyyaml>=5.1 in d:\tools\lib\site-packages (from uvicorn[standard]>=0.32.0->spdf-api==0.1.0) (6.0.3)
Requirement already satisfied: watchfiles>=0.20 in d:\tools\lib\site-packages (from uvicorn[standard]>=0.32.0->spdf-api==0.1.0) (1.1.1)
Requirement already satisfied: websockets>=10.4 in d:\tools\lib\site-packages (from uvicorn[standard]>=0.32.0->spdf-api==0.1.0) (16.0)
Using cached bcrypt-5.0.0-cp39-abi3-win_amd64.whl (150 kB)
Using cached pyjwt-2.12.1-py3-none-any.whl (29 kB)
Building wheels for collected packages: spdf-api
  Building editable for spdf-api (pyproject.toml): started
  Building editable for spdf-api (pyproject.toml): finished with status 'done'
  Created wheel for spdf-api: filename=spdf_api-0.1.0-0.editable-py3-none-any.whl size=2832 sha256=645dbe8e97203f32990318f834fe6a827a57594fe5e6cf0fea64eb26cec451ac
  Stored in directory: C:\Users\mrdee\AppData\Local\Temp\pip-ephem-wheel-cache-ikzed5ao\wheels\81\c1\f1\60acab9f8958ddc6729230800ba7ca79403af75240dad99d81
Successfully built spdf-api
Installing collected packages: PyJWT, bcrypt, spdf-api
  Attempting uninstall: spdf-api
    Found existing installation: spdf-api 0.1.0
    Uninstalling spdf-api-0.1.0:
      Successfully uninstalled spdf-api-0.1.0
Successfully installed PyJWT-2.12.1 bcrypt-5.0.0 spdf-api-0.1.0
pip.exe : 
At D:\SPDF DEVELOPMENT\SPDF\scripts\build-status.ps1:34 char:9
+         & $args_list[0] $args_list[1..($args_list.Length-1)] *> $logP ...
+         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
[notice] A new release of pip is available: 25.0.1 -> 26.0.1
[notice] To update, run: python.exe -m pip install --upgrade pip
```

### python-test.log
```npython.exe : ImportError while loading conftest 'D:\SPDF DEVELOPMENT\SPDF\api\tests\conftest.py'.
At D:\SPDF DEVELOPMENT\SPDF\scripts\build-status.ps1:34 char:9
+         & $args_list[0] $args_list[1..($args_list.Length-1)] *> $logP ...
+         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (ImportError whi...s\conftest.py'.:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
api\tests\conftest.py:14: in <module>
    from app.main import app
api\app\main.py:14: in <module>
    from app.routers import account, billing, documents, templates, webhooks
api\app\routers\documents.py:23: in <module>
    from app.services.spdf_engine import SpdfEngine
api\app\services\spdf_engine.py:12: in <module>
    import spdf_native
E   ModuleNotFoundError: No module named 'spdf_native'
```

### rust-build.log
```ncargo.exe :    Compiling pyo3-build-config v0.22.6
At D:\SPDF DEVELOPMENT\SPDF\scripts\build-status.ps1:34 char:9
+         & $args_list[0] $args_list[1..($args_list.Length-1)] *> $logP ...
+         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling pyo3-build-config v0.22.6:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
warning: unused import: `crate::types::ElementId`
  --> crates\spdf-core\src\diff.rs:10:5
   |
10 | use crate::types::ElementId;
   |     ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: `spdf-core` (lib) generated 1 warning (run `cargo fix --lib -p spdf-core` to apply 1 suggestion)
   Compiling pyo3-ffi v0.22.6
   Compiling pyo3-macros-backend v0.22.6
   Compiling pyo3 v0.22.6
   Compiling pyo3-macros v0.22.6
   Compiling spdf-python v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 19.54s
```

### rust-clippy.log
```n    Checking spdf-core v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core)
    Checking serde-wasm-bindgen v0.6.5
error: unused import: `crate::types::ElementId`
  --> crates\spdf-core\src\diff.rs:10:5
   |
10 | use crate::types::ElementId;
   |     ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(unused_imports)]`

   Compiling pyo3-macros-backend v0.22.6
   Compiling pyo3-ffi v0.22.6
   Compiling pyo3 v0.22.6
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

### rust-test.log
```n
running 11 tests
test list_redactions_empty_on_clean_doc ... ok
test redact_nonexistent_element_fails ... ok
test verify_redaction_not_found ... ok
test redact_element_succeeds ... ok
test redact_preserves_other_elements ... ok
test redacted_element_has_proof_hash ... ok
test redacted_element_replaced_with_redaction ... ok
test verify_redaction_found ... ok
test list_redactions_shows_redacted_elements ... ok
test redact_appends_audit_event ... ok
test multiple_redactions ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

     Running tests\signing_tests.rs (target\debug\deps\signing_tests-76c4fb2b310ba9dd.exe)

running 15 tests
test sign_draft_document_fails_wrong_state ... ok
test transition_invalid_state_fails ... ok
test signed_document_has_signature_entry ... ok
test signature_record_has_correct_fields ... ok
test sign_review_document_succeeds ... ok
test transition_appends_audit_event ... ok
test signed_document_state_is_signed ... ok
test signed_document_locks_signature_blocks ... ok
test sign_already_signed_fails ... ok
test transition_draft_to_review ... ok
test signed_document_has_audit_event ... ok
test verify_reports_signer_details ... ok
test verify_unsigned_document_reports_no_signatures ... ok
test verify_signed_document_is_valid ... FAILED
test verify_tampered_document_detects_tampering ... ok

failures:

---- verify_signed_document_is_valid stdout ----

thread 'verify_signed_document_is_valid' (5820) panicked at crates\spdf-core\tests\signing_tests.rs:199:5:
assertion failed: report.valid
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    verify_signed_document_is_valid

test result: FAILED. 14 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

error: test failed, to rerun pass `-p spdf-core --test signing_tests`
```

