# CHECK RESULTS

## Run Info
- **Version:** 0.1.0-snapshot.6
- **Commit:** 9d9b44a
- **Branch:** main
- **Date:** 2026-03-25T18:01:50Z
- **Machine:** TUF_WARRIOR_DK
- **Overall:** FAILING (4 pass, 2 fail, 0 skip / 6 total)

## Steps

### Rust
- [x] `cargo fmt --check`: **PASS**
- [x] `cargo clippy`: **PASS**
- [x] `cargo test`: **PASS**

### Python
- [x] `pip install api[dev]`: **PASS**
- [ ] `maturin develop`: **FAIL**
- [ ] `pytest`: **FAIL**

---

## Failure Details

### Python: maturin develop
Exit code: 1

```text
maturin : ≡ƒÆÑ maturin failed
At line:1 char:1
+ maturin develop -m crates/spdf-python/Cargo.toml 2>&1
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (≡ƒÆÑ maturin failed:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
  Caused by: Couldn't find a virtualenv or conda environment, but you need one to use this command. For maturin to find your virtualenv you need to either set VIRTUAL_ENV (through activate), set CONDA_PREFIX (through 
conda activate) or have a virtualenv called .venv in the current or any parent folder. See https://virtualenv.pypa.io/en/latest/index.html on how to use virtualenv or use `maturin build` and `pip install 
<path/to/wheel>` instead.
```

### Python: pytest
Exit code: 4

```text
python : ImportError while loading conftest 'D:\SPDF DEVELOPMENT\SPDF\api\tests\conftest.py'.
At line:1 char:1
+ python -m pytest api/tests/ -v --tb=short 2>&1
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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

---

## Test Summary

### Rust
- `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`
- `test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s`
- `test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s`
- `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`
- `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`
- `test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s`
- `test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s`
- `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`
- `test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s`
- `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`
- `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`
- `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`
- `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`
- `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`

### Python
- `E   ModuleNotFoundError: No module named 'spdf_native'`
