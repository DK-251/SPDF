# Build Status

## Last Run
- **Version:** 0.1.0-snapshot.6
- **Date:** 2026-03-25T18:00:50Z
- **Commit:** 9d9b44a
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
```n  Downloading watchfiles-1.1.1-cp312-cp312-win_amd64.whl.metadata (5.0 kB)
Collecting websockets>=10.4 (from uvicorn[standard]>=0.32.0->spdf-api==0.1.0)
  Downloading websockets-16.0-cp312-cp312-win_amd64.whl.metadata (7.0 kB)
Downloading fastapi-0.135.2-py3-none-any.whl (117 kB)
Downloading httpx-0.28.1-py3-none-any.whl (73 kB)
Downloading httpcore-1.0.9-py3-none-any.whl (78 kB)
Downloading pydantic-2.12.5-py3-none-any.whl (463 kB)
Downloading pydantic_core-2.41.5-cp312-cp312-win_amd64.whl (2.0 MB)
   ---------------------------------------- 2.0/2.0 MB 978.3 kB/s eta 0:00:00
Downloading pytest-9.0.2-py3-none-any.whl (374 kB)
Downloading python_multipart-0.0.22-py3-none-any.whl (24 kB)
Downloading uvicorn-0.42.0-py3-none-any.whl (68 kB)
Downloading annotated_doc-0.0.4-py3-none-any.whl (5.3 kB)
Downloading annotated_types-0.7.0-py3-none-any.whl (13 kB)
Downloading click-8.3.1-py3-none-any.whl (108 kB)
Downloading colorama-0.4.6-py2.py3-none-any.whl (25 kB)
Downloading h11-0.16.0-py3-none-any.whl (37 kB)
Downloading httptools-0.7.1-cp312-cp312-win_amd64.whl (86 kB)
Downloading iniconfig-2.3.0-py3-none-any.whl (7.5 kB)
Downloading packaging-26.0-py3-none-any.whl (74 kB)
Downloading pluggy-1.6.0-py3-none-any.whl (20 kB)
Downloading pygments-2.19.2-py3-none-any.whl (1.2 MB)
   ---------------------------------------- 1.2/1.2 MB 1.3 MB/s eta 0:00:00
Downloading python_dotenv-1.2.2-py3-none-any.whl (22 kB)
Downloading pyyaml-6.0.3-cp312-cp312-win_amd64.whl (154 kB)
Downloading starlette-1.0.0-py3-none-any.whl (72 kB)
Downloading anyio-4.13.0-py3-none-any.whl (114 kB)
Downloading idna-3.11-py3-none-any.whl (71 kB)
Downloading typing_extensions-4.15.0-py3-none-any.whl (44 kB)
Downloading typing_inspection-0.4.2-py3-none-any.whl (14 kB)
Downloading watchfiles-1.1.1-cp312-cp312-win_amd64.whl (288 kB)
Downloading websockets-16.0-cp312-cp312-win_amd64.whl (178 kB)
Downloading certifi-2026.2.25-py3-none-any.whl (153 kB)
Building wheels for collected packages: spdf-api
  Building editable for spdf-api (pyproject.toml): started
  Building editable for spdf-api (pyproject.toml): finished with status 'done'
  Created wheel for spdf-api: filename=spdf_api-0.1.0-0.editable-py3-none-any.whl size=2814 sha256=7624c3ca1176b0a6dc82bf0078782073c9b0810a9dc7b55257c093d0cf05bb67
  Stored in directory: C:\Users\mrdee\AppData\Local\Temp\pip-ephem-wheel-cache-i2mzrp5x\wheels\81\c1\f1\60acab9f8958ddc6729230800ba7ca79403af75240dad99d81
Successfully built spdf-api
Installing collected packages: websockets, typing-extensions, pyyaml, python-multipart, python-dotenv, pygments, pluggy, packaging, iniconfig, idna, httptools, h11, colorama, certifi, annotated-types, annotated-doc, typing-inspection, pytest, pydantic-core, httpcore, click, anyio, watchfiles, uvicorn, starlette, pydantic, httpx, fastapi, spdf-api
Successfully installed annotated-doc-0.0.4 annotated-types-0.7.0 anyio-4.13.0 certifi-2026.2.25 click-8.3.1 colorama-0.4.6 fastapi-0.135.2 h11-0.16.0 httpcore-1.0.9 httptools-0.7.1 httpx-0.28.1 idna-3.11 iniconfig-2.3.0 packaging-26.0 pluggy-1.6.0 pydantic-2.12.5 pydantic-core-2.41.5 pygments-2.19.2 pytest-9.0.2 python-dotenv-1.2.2 python-multipart-0.0.22 pyyaml-6.0.3 spdf-api-0.1.0 starlette-1.0.0 typing-extensions-4.15.0 typing-inspection-0.4.2 uvicorn-0.42.0 watchfiles-1.1.1 websockets-16.0
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
api\app\main.py:10: in <module>
    from app.routers import documents
api\app\routers\documents.py:17: in <module>
    from app.services.spdf_engine import SpdfEngine
api\app\services\spdf_engine.py:12: in <module>
    import spdf_native
E   ModuleNotFoundError: No module named 'spdf_native'
```

### rust-build.log
```ncargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
At D:\SPDF DEVELOPMENT\SPDF\scripts\build-status.ps1:34 char:9
+         & $args_list[0] $args_list[1..($args_list.Length-1)] *> $logP ...
+         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.36s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
```

### rust-clippy.log
```ncargo.exe :     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
At D:\SPDF DEVELOPMENT\SPDF\scripts\build-status.ps1:34 char:9
+         & $args_list[0] $args_list[1..($args_list.Length-1)] *> $logP ...
+         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (    Finished `d...get(s) in 0.35s:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
```

### rust-test.log
```ntest e007_table_no_headers ... ok
test e003_duplicate_element_id ... ok
test e001_whitespace_title ... ok
test e009_empty_invoice_number ... ok
test e006_heading_level_zero ... ok
test e010_empty_payment_total ... ok
test e011_redaction_empty_redacted_eid ... ok
test e012_select_without_options ... ok
test f001_no_pages ... ok
test e013_empty_variable_name ... ok
test f002_empty_page ... ok
test f003_wrong_manifest_format ... ok
test f004_empty_layer_checksum ... ok
test f005_empty_manifest_hash ... ok
test report_counts ... ok
test valid_manifest_passes ... ok
test valid_document_passes ... ok

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

