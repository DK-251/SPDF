# CHECK RESULTS

## Run Info
| Field | Value |
|-------|-------|
| Version | 0.1.0-snapshot.14 |
| Commit | `1071a72` |
| Branch | main |
| Date | 2026-03-26T10:32:07Z |
| Machine | TUF_WARRIOR_DK |
| Overall | **FAILING** (6 pass, 3 fail, 0 skip / 9 steps) |

## Steps

| # | Section | Step | Result |
|---|---------|------|--------|
| 1 | Rust | `cargo fmt --check` | PASS |
| 2 | Rust | `cargo clippy` | PASS |
| 3 | Rust | `cargo test` | PASS |
| 4 | Python | `pip install api[dev]` | PASS |
| 5 | Python | `maturin develop` | PASS |
| 6 | Python | `pytest` | PASS |
| 7 | Studio | `npm ci` | **FAIL** |
| 8 | Studio | `vitest` | **FAIL** |
| 9 | Studio | `vite build` | **FAIL** |

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

**Summary:** `====================== 157 passed, 18 warnings in 9.38s =======================`

---

## Studio Test Breakdown

*Vitest: 0 passed, 0 failed*

---

## Grand Total

| | Passed | Failed | Skipped | Total |
|--|--------|--------|---------|-------|
| Rust | 140 | 0 | 0 | 140 |
| Python | 157 | 0 | 0 | 157 |
| Studio | 0 | 0 | 0 | 0 |
| **Total** | **297** | **0** | **0** | **297** |

---

## Failure Details

### Studio: npm ci
Exit code: 1

```text
At line:1 char:41
+ cd studio; npm ci --prefer-offline 2>&1 2>&1
+                                         ~~~~
The error stream for this command is already redirected.
```

### Studio: vitest
Exit code: 1

```text
At line:1 char:32
+ cd studio; npx vitest run 2>&1 2>&1
+                                ~~~~
The error stream for this command is already redirected.
```

### Studio: vite build
Exit code: 1

```text
At line:1 char:32
+ cd studio; npx vite build 2>&1 2>&1
+                                ~~~~
The error stream for this command is already redirected.
```

error copied from console:

PS D:\SPDF DEVELOPMENT\SPDF> just check
powershell -ExecutionPolicy Bypass -File scripts/check.ps1

=== Rust ===
  [Rust] cargo fmt --check ... PASS
  [Rust] cargo clippy ... PASS
  [Rust] cargo test ... PASS

=== Python ===
  [Python] pip install api[dev] ... PASS
  [Python] maturin develop ... PASS
  [Python] pytest ... PASS

=== Studio ===
npm error code EUSAGE
npm error
npm error The `npm ci` command can only install with an existing package-lock.json or
npm error npm-shrinkwrap.json with lockfileVersion >= 1. Run an install with npm@5 or
npm error later to generate a package-lock.json file, then try again.
npm error
npm error Clean install a project
npm error
npm error Usage:
npm error npm ci
npm error
npm error Options:
npm error [--install-strategy <hoisted|nested|shallow|linked>] [--legacy-bundling]
npm error [--global-style] [--omit <dev|optional|peer> [--omit <dev|optional|peer> ...]]
npm error [--include <prod|dev|optional|peer> [--include <prod|dev|optional|peer> ...]]
npm error [--strict-peer-deps] [--foreground-scripts] [--ignore-scripts] [--no-audit]
npm error [--no-bin-links] [--no-fund] [--dry-run]
npm error [-w|--workspace <workspace-name> [-w|--workspace <workspace-name> ...]]
npm error [-ws|--workspaces] [--include-workspace-root] [--install-links]
npm error
npm error aliases: clean-install, ic, install-clean, isntall-clean
npm error
npm error Run "npm help ci" for more info

npm error A complete log of this run can be found in: C:\Users\mrdee\AppData\Local\npm-cache\_logs\2026-03-26T11_00_42_512Z-debug-0.log
 FAIL (exit 1)
