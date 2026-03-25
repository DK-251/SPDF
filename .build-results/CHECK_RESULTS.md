# CHECK RESULTS

## Run Info
- **Version:** 0.1.0-snapshot.7
- **Commit:** 8e89b4e
- **Branch:** main
- **Date:** 2026-03-25T18:43:03Z
- **Machine:** TUF_WARRIOR_DK
- **Overall:** FAILING (5 pass, 1 fail, 0 skip / 6 total)

## Steps

### Rust
- [x] `cargo fmt --check`: **PASS**
- [x] `cargo clippy`: **PASS**
- [x] `cargo test`: **PASS**

### Python
- [x] `pip install api[dev]`: **PASS**
- [x] `maturin develop`: **PASS**
- [ ] `pytest`: **FAIL**

---

## Failure Details

### Python: pytest
Exit code: 4

```text
python : (trapped) error reading bcrypt version
At line:1 char:1
+ python -m pytest api/tests/ -v --tb=short 2>&1
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: ((trapped) error reading bcrypt version:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
Traceback (most recent call last):
  File "D:\SPDF DEVELOPMENT\SPDF\.venv\Lib\site-packages\passlib\handlers\bcrypt.py", line 620, in _load_backend_mixin
    version = _bcrypt.__about__.__version__
              ^^^^^^^^^^^^^^^^^
AttributeError: module 'bcrypt' has no attribute '__about__'
ImportError while loading conftest 'D:\SPDF DEVELOPMENT\SPDF\api\tests\conftest.py'.
api\tests\conftest.py:14: in <module>
    from app.main import app
api\app\main.py:33: in <module>
    seed_test_user()
api\app\services\api_keys.py:51: in seed_test_user
    key_hash = bcrypt.using(rounds=4).hash(TEST_API_KEY)
               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
.venv\Lib\site-packages\passlib\utils\handlers.py:779: in hash
    self.checksum = self._calc_checksum(secret)
                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
.venv\Lib\site-packages\passlib\handlers\bcrypt.py:591: in _calc_checksum
    self._stub_requires_backend()
.venv\Lib\site-packages\passlib\utils\handlers.py:2254: in _stub_requires_backend
    cls.set_backend()
.venv\Lib\site-packages\passlib\utils\handlers.py:2156: in set_backend
    return owner.set_backend(name, dryrun=dryrun)
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
.venv\Lib\site-packages\passlib\utils\handlers.py:2163: in set_backend
    return cls.set_backend(name, dryrun=dryrun)
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
.venv\Lib\site-packages\passlib\utils\handlers.py:2188: in set_backend
    cls._set_backend(name, dryrun)
.venv\Lib\site-packages\passlib\utils\handlers.py:2311: in _set_backend
    super(SubclassBackendMixin, cls)._set_backend(name, dryrun)
.venv\Lib\site-packages\passlib\utils\handlers.py:2224: in _set_backend
    ok = loader(**kwds)
         ^^^^^^^^^^^^^^
.venv\Lib\site-packages\passlib\handlers\bcrypt.py:626: in _load_backend_mixin
    return mixin_cls._finalize_backend_mixin(name, dryrun)
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
.venv\Lib\site-packages\passlib\handlers\bcrypt.py:421: in _finalize_backend_mixin
    if detect_wrap_bug(IDENT_2A):
       ^^^^^^^^^^^^^^^^^^^^^^^^^
.venv\Lib\site-packages\passlib\handlers\bcrypt.py:380: in detect_wrap_bug
    if verify(secret, bug_hash):
       ^^^^^^^^^^^^^^^^^^^^^^^^
.venv\Lib\site-packages\passlib\utils\handlers.py:792: in verify
    return consteq(self._calc_checksum(secret), chk)
                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^
.venv\Lib\site-packages\passlib\handlers\bcrypt.py:655: in _calc_checksum
    hash = _bcrypt.hashpw(secret, config)
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   ValueError: password cannot be longer than 72 bytes, truncate manually if necessary (e.g. my_password[:72])
```

---

## Test Summary

### Rust
- `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`
- `test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s`
- `test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s`
- `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`
- `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`
- `test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s`
- `test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s`
- `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`
- `test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s`
- `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`
- `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`
- `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`
- `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`
- `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`

### Python
- `E   ValueError: password cannot be longer than 72 bytes, truncate manually if necessary (e.g. my_password[:72])`
