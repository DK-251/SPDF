# Build Status

## Last Run
- **Version:** 0.1.0-snapshot.6
- **Date:** 2026-03-25T18:07:53Z
- **Commit:** f5d1dfa
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
```n  Preparing editable metadata (pyproject.toml): finished with status 'done'
Requirement already satisfied: fastapi>=0.115.0 in d:\tools\lib\site-packages (from spdf-api==0.1.0) (0.135.2)
Requirement already satisfied: uvicorn>=0.32.0 in d:\tools\lib\site-packages (from uvicorn[standard]>=0.32.0->spdf-api==0.1.0) (0.42.0)
Requirement already satisfied: python-multipart>=0.0.18 in d:\tools\lib\site-packages (from spdf-api==0.1.0) (0.0.22)
Requirement already satisfied: pydantic>=2.10.0 in d:\tools\lib\site-packages (from spdf-api==0.1.0) (2.12.5)
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
Building wheels for collected packages: spdf-api
  Building editable for spdf-api (pyproject.toml): started
  Building editable for spdf-api (pyproject.toml): finished with status 'done'
  Created wheel for spdf-api: filename=spdf_api-0.1.0-0.editable-py3-none-any.whl size=2814 sha256=4b8c4b19df1e45f4f5ec6293aa298ecce478c9a2d31204bb8b9cb46d266839da
  Stored in directory: C:\Users\mrdee\AppData\Local\Temp\pip-ephem-wheel-cache-oxnyyr56\wheels\81\c1\f1\60acab9f8958ddc6729230800ba7ca79403af75240dad99d81
Successfully built spdf-api
Installing collected packages: spdf-api
  Attempting uninstall: spdf-api
    Found existing installation: spdf-api 0.1.0
    Uninstalling spdf-api-0.1.0:
      Successfully uninstalled spdf-api-0.1.0
Successfully installed spdf-api-0.1.0
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
```ncargo.exe :    Compiling pyo3-build-config v0.22.6
At D:\SPDF DEVELOPMENT\SPDF\scripts\build-status.ps1:34 char:9
+         & $args_list[0] $args_list[1..($args_list.Length-1)] *> $logP ...
+         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling pyo3-build-config v0.22.6:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   Compiling pyo3-macros-backend v0.22.6
   Compiling pyo3-ffi v0.22.6
   Compiling pyo3 v0.22.6
   Compiling pyo3-macros v0.22.6
   Compiling spdf-python v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 9.56s
```

### rust-clippy.log
```ncargo.exe :    Compiling pyo3-build-config v0.22.6
At D:\SPDF DEVELOPMENT\SPDF\scripts\build-status.ps1:34 char:9
+         & $args_list[0] $args_list[1..($args_list.Length-1)] *> $logP ...
+         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (   Compiling pyo3-build-config v0.22.6:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   Compiling pyo3-macros-backend v0.22.6
   Compiling pyo3-ffi v0.22.6
   Compiling pyo3 v0.22.6
   Compiling pyo3-macros v0.22.6
    Checking spdf-python v0.1.0 (D:\SPDF DEVELOPMENT\SPDF\crates\spdf-python)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.77s
```

### rust-test.log
```ntest e006_heading_level_zero ... ok
test e008_table_row_cell_mismatch ... ok
test e007_table_no_headers ... ok
test e010_empty_payment_total ... ok
test e011_redaction_empty_redacted_eid ... ok
test e009_empty_invoice_number ... ok
test e008_line_item_table_row_cell_mismatch ... ok
test e012_select_without_options ... ok
test f001_no_pages ... ok
test e013_empty_variable_name ... ok
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

