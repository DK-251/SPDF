# CHECK RESULTS

## Run Info
| Field | Value |
|-------|-------|
| Version | 0.1.0-snapshot.14 |
| Commit | `665e60f` |
| Branch | main |
| Date | 2026-03-26T11:12:59Z |
| Machine | TUF_WARRIOR_DK |
| Overall | **FAILING** (8 pass, 1 fail, 0 skip / 9 steps) |

## Steps

| # | Section | Step | Result |
|---|---------|------|--------|
| 1 | Rust | `cargo fmt --check` | PASS |
| 2 | Rust | `cargo clippy` | PASS |
| 3 | Rust | `cargo test` | PASS |
| 4 | Python | `pip install api[dev]` | PASS |
| 5 | Python | `maturin develop` | PASS |
| 6 | Python | `pytest` | PASS |
| 7 | Studio | `npm install` | PASS |
| 8 | Studio | `vitest` | **FAIL** |
| 9 | Studio | `vite build` | PASS |

---

## Rust Test Breakdown

| Module | Passed | Failed |
|--------|--------|--------|
| `tests\container_tests.rs` | 14 | 0 |
| `tests\diff_tests.rs` | 11 | 0 |
| `tests\dom_tests.rs` | 31 | 0 |
| `tests\redaction_tests.rs` | 11 | 0 |
| `tests\signing_tests.rs` | 15 | 0 |
| `tests\binding_logic_tests.rs` | 20 | 0 |
| `tests\integration_test.rs` | 3 | 0 |
| `tests\validator_tests.rs` | 25 | 0 |
| `tests\wasm_logic_tests.rs` | 10 | 0 |
| **Total** | **140** | **0** |

---

## Python Test Breakdown

| Module | Passed | Failed | Skipped |
|--------|--------|--------|---------|
| `test_account.py` | 16 | 0 | 0 |
| `test_billing.py` | 11 | 0 | 0 |
| `test_diff.py` | 9 | 0 | 0 |
| `test_documents.py` | 32 | 0 | 0 |
| `test_e2e.py` | 8 | 0 | 0 |
| `test_hardening.py` | 13 | 0 | 0 |
| `test_jwt_auth.py` | 11 | 0 | 0 |
| `test_rate_limit.py` | 13 | 0 | 0 |
| `test_redaction.py` | 8 | 0 | 0 |
| `test_signing.py` | 11 | 0 | 0 |
| `test_templates.py` | 15 | 0 | 0 |
| `test_webhooks.py` | 10 | 0 | 0 |
| **Total** | **157** | **0** | **0** |

### Python Warnings (24)

```text
tests/test_e2e.py::test_jwt_auth_template_crud
tests/test_jwt_auth.py::test_jwt_auth_returns_200
tests/test_jwt_auth.py::test_jwt_auth_resolves_correct_user
tests/test_jwt_auth.py::test_jwt_auth_rate_limit_headers
tests/test_jwt_auth.py::test_both_auth_methods_on_same_endpoint
tests/test_jwt_auth.py::test_expired_jwt_returns_401
tests/test_jwt_auth.py::test_unknown_email_returns_401
tests/test_jwt_auth.py::test_wrong_issuer_returns_401
D:\SPDF DEVELOPMENT\SPDF\.venv\Lib\site-packages\jwt\api_jwt.py:147: InsecureKeyLengthWarning: The HMAC key is 20 bytes long, which is below the minimum recommended length of 32 bytes for SHA256. See RFC 7518 Section 3.2.
return self._jws.encode(
tests/test_e2e.py::test_jwt_auth_template_crud
tests/test_jwt_auth.py::test_jwt_auth_returns_200
tests/test_jwt_auth.py::test_jwt_auth_resolves_correct_user
tests/test_jwt_auth.py::test_jwt_auth_rate_limit_headers
tests/test_jwt_auth.py::test_both_auth_methods_on_same_endpoint
tests/test_jwt_auth.py::test_expired_jwt_returns_401
tests/test_jwt_auth.py::test_bad_signature_returns_401
tests/test_jwt_auth.py::test_unknown_email_returns_401
tests/test_jwt_auth.py::test_wrong_issuer_returns_401
D:\SPDF DEVELOPMENT\SPDF\.venv\Lib\site-packages\jwt\api_jwt.py:365: InsecureKeyLengthWarning: The HMAC key is 20 bytes long, which is below the minimum recommended length of 32 bytes for SHA256. See RFC 7518 Section 3.2.
decoded = self.decode_complete(
tests/test_jwt_auth.py::test_bad_signature_returns_401
D:\SPDF DEVELOPMENT\SPDF\.venv\Lib\site-packages\jwt\api_jwt.py:147: InsecureKeyLengthWarning: The HMAC key is 12 bytes long, which is below the minimum recommended length of 32 bytes for SHA256. See RFC 7518 Section 3.2.
return self._jws.encode(
```

**Summary:** `====================== 157 passed, 18 warnings in 5.76s =======================`

---

## Studio Test Breakdown

*Vitest: 0 passed, 1 failed*

**Summary:** `[2m   Duration [22m 3.94s[2m (transform 1.33s, setup 3.00s, collect 8.19s, tests 2.75s, environment 13.40s, prepare 5.88s)[22m`

---

## Grand Total

| | Passed | Failed | Skipped | Total |
|--|--------|--------|---------|-------|
| Rust | 140 | 0 | 0 | 140 |
| Python | 157 | 0 | 0 | 157 |
| Studio | 0 | 1 | 0 | 1 |
| **Total** | **297** | **1** | **0** | **298** |

---

## Failure Details

### Studio: vitest
Exit code: 1

*(truncated to last 100 lines)*

```text
              [33mstroke-linejoin[31m=[32m"round"[31m
              [33mstroke-width[31m=[32m"2"[31m
              [33mviewBox[31m=[32m"0 0 24 24"[31m
              [33mwidth[31m=[32m"24"[31m
              [33mxmlns[31m=[32m"http://www.w3.org/2000/svg"[31m
            [36m>[31m
              [36m<path[31m
                [33md[31m=[32m"M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"[31m
              [36m/>[31m
              [36m<path[31m
                [33md[31m=[32m"M14 2v4a2 2 0 0 0 2 2h4"[31m
              [36m/>[31m
              [36m<path[31m
                [33md[31m=[32m"M10 9H8"[31m
              [36m/>[31m
              [36m<path[31m
                [33md[31m=[32m"M16 13H8"[31m
              [36m/>[31m
              [36m<path[31m
                [33md[31m=[32m"M16 17H8"[31m
              [36m/>[31m
            [36m</svg>[31m
            [36m<span>[31m
              [0mDocuments[0m
            [36m</span>[31m
          [36m</a>[31m
          [36m<a[31m
            [33mclass[31m=[32m"flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors text-muted-foreground hover:bg-accent/50 hover:text-foreground"[31m
            [33mhref[31m=[32m"/generate"[31m
          [36m>[31m
            [36m<svg[31m
              [33mclass[31m=[32m"lucide lucide-plus h-4 w-4 shrink-0"[31m
              [33mfill[31m=[32m"none"[31m
              [33mheight[31m=[32m"24"[31m
              [33mstroke[31m=[32m"currentColor"[31m
              [33mstroke-linecap[31m=[32m"round"[31m
              [33mstroke-linejoin[31m=[32m"round"[31m
              [33mstroke-width[31m=[32m"2"[31m
              [33mviewBox[31m=[32m"0 0 24 24"[31m
              [33mwidth[31m=[32m"24"[31m
              [33mxmlns[31m=[32m"http://www.w3.org/2000/svg"[31m
            [36m>[31m
              [36m<path[31m
                [33md[31m=[32m"M5 12h14"[31m
              [36m/>[31m
              [36m<path[31m
                [33md[31m=[32m"M12 5v14"[31m
              [36m/>[31m
            [36m</svg>[31m
            [36m<span>[31m
              [0mGenerate[0m
            [36m</span>[31m
          [36m</a>[31m
          [36m<a[31m
            [33mclass[31m=[32m"flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors text-muted-foreground hover:bg-accent/50 hover:text-foreground"[31m
            [33mhref[31m=[32m"/templates"[31m
          [36m>[31m
            [36m<svg[31m
              [33mclass[31m=[32m"lucide lucide-layout-template h-4 w-4 shrink-0"[31m
              [33mfill[31m=[32m"none"[31m
              [33mheight[31m=[32m"24"[31m
              [33mstroke[31m=[32m"currentColor"[31m
              [33mstroke-linecap[31m=[32m"round"[31m
              [33mstroke-linejoin[31m=[32m"round"[31m
              [33mstroke-width[31m=[32m"2"[31m
              [33mviewBox[31m=[32m"0 0 24 24"[31m
              [33mwidth[31m=[32m"24"[31m
              [33mxmlns[31m=[32m"http://www.w3.org/2000/svg"[31m
            [36m>[31m
              [36m<rect[31m
                [33mheight[31m=[32m"7"[31m
                [33mrx[31m=[32m"1"[31m
                [33mwidth[31m=[32m"18"[31m
                [33mx[31m=[32m"3"[31m
                [33my[31m=[32m"3"[31m
              [36m/>[31m
              [36m<rect[31m
                [33mheight[31m=[32m"7"[31m
                [33mrx[31m=[32m"1"[31m
                [33mwidth[31m=[32m"9"[31m
                [33mx[31m=[32m"3"[31m
                [33my[31m=[32m"14"[31m
              [36m/>[31m
              [36m<rect[31m
                [33mheight[31m=[32m"7"[31m
                [33mrx[31m=[32m"1"[31m
                [33mwidth[31m=[32m"5"[31m
                [33mx[31m=[32m"16"[31m
                [33my[31m=[32m"14"[31m
              [36m/>[31m
            [36m</svg>[31m
            [36m<span>[31m
              [0mTemplates[0m
            [36m</s...[39m
 [32mΓ£ô[39m src/__tests__/auth-store.test.ts [2m ([22m[2m4 tests[22m[2m)[22m[90m 3[2mms[22m[39m

[2m Test Files [22m [1m[31m1 failed[39m[22m[2m | [22m[1m[32m11 passed[39m[22m[90m (12)[39m
[2m      Tests [22m [1m[31m1 failed[39m[22m[2m | [22m[1m[32m69 passed[39m[22m[90m (70)[39m
[2m   Start at [22m 16:44:00
[2m   Duration [22m 3.94s[2m (transform 1.33s, setup 3.00s, collect 8.19s, tests 2.75s, environment 13.40s, prepare 5.88s)[22m
```
