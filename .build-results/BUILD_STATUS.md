# Build Status

## Last Run
- **Version:** 0.1.0-snapshot.6
- **Date:** 2026-03-25T17:53:18Z
- **Commit:** 80dd4f4
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
```n    return func(self, options, args)
           ^^^^^^^^^^^^^^^^^^^^^^^^^
  File "D:\TOOLS\Lib\site-packages\pip\_internal\commands\install.py", line 386, in run
    requirement_set = resolver.resolve(
                      ^^^^^^^^^^^^^^^^^
  File "D:\TOOLS\Lib\site-packages\pip\_internal\resolution\resolvelib\resolver.py", line 76, in resolve
    collected = self.factory.collect_root_requirements(root_reqs)
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "D:\TOOLS\Lib\site-packages\pip\_internal\resolution\resolvelib\factory.py", line 545, in collect_root_requirements
    reqs = list(
           ^^^^^
  File "D:\TOOLS\Lib\site-packages\pip\_internal\resolution\resolvelib\factory.py", line 501, in _make_requirements_from_install_req
    cand = self._make_base_candidate_from_link(
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "D:\TOOLS\Lib\site-packages\pip\_internal\resolution\resolvelib\factory.py", line 212, in _make_base_candidate_from_link
    self._editable_candidate_cache[link] = EditableCandidate(
                                           ^^^^^^^^^^^^^^^^^^
  File "D:\TOOLS\Lib\site-packages\pip\_internal\resolution\resolvelib\candidates.py", line 329, in __init__
    super().__init__(
  File "D:\TOOLS\Lib\site-packages\pip\_internal\resolution\resolvelib\candidates.py", line 159, in __init__
    self.dist = self._prepare()
                ^^^^^^^^^^^^^^^
  File "D:\TOOLS\Lib\site-packages\pip\_internal\resolution\resolvelib\candidates.py", line 236, in _prepare
    dist = self._prepare_distribution()
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "D:\TOOLS\Lib\site-packages\pip\_internal\resolution\resolvelib\candidates.py", line 339, in _prepare_distribution
    return self._factory.preparer.prepare_editable_requirement(self._ireq)
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "D:\TOOLS\Lib\site-packages\pip\_internal\operations\prepare.py", line 698, in prepare_editable_requirement
    dist = _get_prepared_distribution(
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "D:\TOOLS\Lib\site-packages\pip\_internal\operations\prepare.py", line 72, in _get_prepared_distribution
    abstract_dist.prepare_distribution_metadata(
  File "D:\TOOLS\Lib\site-packages\pip\_internal\distributions\sdist.py", line 54, in prepare_distribution_metadata
    self.req.isolated_editable_sanity_check()
  File "D:\TOOLS\Lib\site-packages\pip\_internal\req\req_install.py", line 541, in isolated_editable_sanity_check
    and not self.supports_pyproject_editable
            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "D:\TOOLS\Lib\functools.py", line 998, in __get__
    val = self.func(instance)
          ^^^^^^^^^^^^^^^^^^^
  File "D:\TOOLS\Lib\site-packages\pip\_internal\req\req_install.py", line 258, in supports_pyproject_editable
    return "build_editable" in self.pep517_backend._supported_features()
                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "D:\TOOLS\Lib\site-packages\pip\_vendor\pyproject_hooks\_impl.py", line 180, in _supported_features
    return self._call_hook("_supported_features", {})
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "D:\TOOLS\Lib\site-packages\pip\_vendor\pyproject_hooks\_impl.py", line 402, in _call_hook
    raise BackendUnavailable(
pip._vendor.pyproject_hooks._impl.BackendUnavailable: Cannot import 'setuptools.backends._legacy'
```

### python-test.log
```npython.exe : D:\TOOLS\python.exe: No module named pytest
At D:\SPDF DEVELOPMENT\SPDF\scripts\build-status.ps1:34 char:9
+         & $args_list[0] $args_list[1..($args_list.Length-1)] *> $logP ...
+         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (D:\TOOLS\python...le named pytest:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
```

### rust-build.log
```n   Compiling serde_json v1.0.149
   Compiling lzma-rs v0.3.0
   Compiling crossbeam-epoch v0.9.18
   Compiling pbkdf2 v0.12.2
   Compiling sha1 v0.10.6
   Compiling aes v0.8.4
   Compiling zopfli v0.8.3
   Compiling deflate64 v0.1.12
   Compiling rayon-core v1.13.0
   Compiling constant_time_eq v0.3.1
   Compiling uuid v1.22.0
   Compiling crossbeam-deque v0.8.6
   Compiling pyo3-ffi v0.22.6
   Compiling pyo3-macros-backend v0.22.6
   Compiling wasm-bindgen v0.2.114
   Compiling sha2 v0.10.9
   Compiling memoffset v0.9.1
   Compiling minimal-lexical v0.2.1
   Compiling either v1.15.0
   Compiling heck v0.5.0
   Compiling pyo3 v0.22.6
   Compiling md-5 v0.10.6
   Compiling nom v7.1.3
   Compiling encoding_rs v0.8.35
   Compiling rangemap v1.7.1
   Compiling weezl v0.1.12
   Compiling rayon v1.11.0
   Compiling wasm-bindgen-macro-support v0.2.114
   Compiling bzip2 v0.5.2
   Compiling indoc v2.0.7
   Compiling unindent v0.2.4
   Compiling serde_derive v1.0.228
   Compiling thiserror-impl v2.0.18
   Compiling zeroize_derive v1.4.3
   Compiling displaydoc v0.2.5
   Compiling zeroize v1.8.2
   Compiling zstd v0.13.3
   Compiling wasm-bindgen-macro v0.2.114
   Compiling pyo3-macros v0.22.6
   Compiling xz2 v0.1.7
   Compiling js-sys v0.3.91
   Compiling chrono v0.4.44
   Compiling spdf-core v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core)
   Compiling lopdf v0.34.0
   Compiling spdf-validator v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-validator)
   Compiling spdf-renderer v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer)
   Compiling spdf-python v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python)
   Compiling serde-wasm-bindgen v0.6.5
   Compiling spdf-wasm v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-wasm)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 45.35s
```

### rust-clippy.log
```n   Compiling pyo3-macros-backend v0.22.6
    Checking crossbeam-deque v0.8.6
    Checking minimal-lexical v0.2.1
    Checking either v1.15.0
    Checking hmac v0.12.1
    Checking sha1 v0.10.6
    Checking aes v0.8.4
    Checking pbkdf2 v0.12.2
    Checking sha2 v0.10.9
    Checking rayon-core v1.13.0
    Checking nom v7.1.3
    Checking md-5 v0.10.6
   Compiling pyo3 v0.22.6
    Checking encoding_rs v0.8.35
    Checking rangemap v1.7.1
    Checking weezl v0.1.12
    Checking memoffset v0.9.1
    Checking rayon v1.11.0
    Checking uuid v1.22.0
    Checking serde_json v1.0.149
    Checking unindent v0.2.4
   Compiling zstd-sys v2.0.16+zstd.1.5.7
   Compiling bzip2-sys v0.1.13+1.0.8
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
    Checking thiserror v2.0.18
    Checking xz2 v0.1.7
   Compiling wasm-bindgen-macro v0.2.114
    Checking zip v2.4.2
   Compiling pyo3-macros v0.22.6
    Checking wasm-bindgen v0.2.114
    Checking serde v1.0.228
    Checking chrono v0.4.44
    Checking js-sys v0.3.91
    Checking spdf-core v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-core)
    Checking lopdf v0.34.0
    Checking spdf-validator v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-validator)
    Checking spdf-renderer v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-renderer)
    Checking spdf-python v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python)
    Checking serde-wasm-bindgen v0.6.5
    Checking spdf-wasm v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-wasm)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 17.99s
```

### rust-test.log
```ntest e008_line_item_table_row_cell_mismatch ... ok
test e007_table_no_headers ... ok
test e008_table_row_cell_mismatch ... ok
test e001_empty_title ... ok
test e009_empty_invoice_number ... ok
test e010_empty_payment_total ... ok
test e011_redaction_empty_redacted_eid ... ok
test e012_select_without_options ... ok
test e013_empty_variable_name ... ok
test f001_no_pages ... ok
test f002_empty_page ... ok
test f004_empty_layer_checksum ... ok
test f003_wrong_manifest_format ... ok
test f005_empty_manifest_hash ... ok
test report_counts ... ok
test valid_document_passes ... ok
test valid_manifest_passes ... ok

test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

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

