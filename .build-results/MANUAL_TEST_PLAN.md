# SPDF Studio — Manual Test Plan

> **Version:** 0.1.0-snapshot.14
> **Date:** 2026-03-26
> **Tester:** _______________
> **Browser:** _______________
> **Prerequisites:** `just dev-quick` running (API on :8000, Studio on :5173)
>
> **How to use:** For each test, update Status to PASS/FAIL/SKIP and add notes.
> Push results back so we can triage failures.

---

## 1. Authentication & Entry (UI + Functional)

| # | Test | Status | Notes |
|---|------|--------|-------|
| 1.1 | Open `http://localhost:5173` — AuthGate modal appears with "Welcome to SPDF Studio", password input, "Connect" button | | |
| 1.2 | Click "Connect" with empty input — error "API key is required" | | |
| 1.3 | Enter "badkey" (no sk_ prefix) and click "Connect" — error "API key must start with sk_" | | |
| 1.4 | Start typing after error — error message clears | | |
| 1.5 | Enter valid key (e.g. `sk_live_test123`) and click "Connect" — modal closes, app shell loads | | |
| 1.6 | Refresh page — AuthGate skipped, app loads directly (key persisted in localStorage) | | |

---

## 2. Layout & Navigation (UI)

| # | Test | Status | Notes |
|---|------|--------|-------|
| 2.1 | Sidebar shows 4 links: Documents, Generate, Templates, Settings — each with icon | | |
| 2.2 | Click each sidebar link — page changes, active link has highlight styling | | |
| 2.3 | Header shows correct title per page: Documents `/`, Generate Document `/generate`, Templates `/templates`, Settings `/settings` | | |
| 2.4 | Header badge shows green key icon + first 7 chars of API key + "..." | | |
| 2.5 | Click sidebar collapse button — sidebar shrinks to icons only, labels hidden | | |
| 2.6 | Hover collapsed sidebar icons — tooltip shows link name | | |
| 2.7 | Refresh — sidebar collapse state persists | | |
| 2.8 | Click expand button — sidebar returns to full width with labels | | |

---

## 3. Dashboard / Documents Page (UI + Functional)

| # | Test | Status | Notes |
|---|------|--------|-------|
| 3.1 | Dashboard shows "Documents" header, Generate button (top right), upload zone | | |
| 3.2 | Empty state message visible: "No documents yet..." | | |
| 3.3 | Upload zone shows dashed border with "Drop an SPDF file here" or "click to browse" | | |
| 3.4 | Click upload zone — file picker opens, accepts `.spdf` files | | |
| 3.5 | Drag an SPDF file over zone — border changes color, drop icon animates | | |
| 3.6 | Drop/select SPDF file — document appears as card in grid | | |
| 3.7 | Document card shows: file icon, name, state badge (Draft), size, timestamp | | |
| 3.8 | Hover card — background changes, Download and Delete buttons appear | | |
| 3.9 | Click card — navigates to `/documents/{id}` (Document Viewer) | | |
| 3.10 | Click Delete button on card — document removed from list | | |
| 3.11 | Click "Generate" button — navigates to `/generate` page | | |

---

## 4. Generate Page (Functional)

| # | Test | Status | Notes |
|---|------|--------|-------|
| 4.1 | Page loads with sample JSON pre-filled in textarea and name "Untitled Document" | | |
| 4.2 | "Load Sample" button resets textarea to sample JSON | | |
| 4.3 | Break JSON syntax (remove a brace), click "Generate SPDF" — shows JSON parse error | | |
| 4.4 | Fix JSON, error clears | | |
| 4.5 | Enter valid JSON + document name, click "Generate SPDF" — loading spinner shows | | |
| 4.6 | On success: navigates to `/documents/{id}` with new document | | |
| 4.7 | On error (e.g. invalid semantic structure): error message shown, can retry | | |

---

## 5. Document Viewer (UI + Functional)

| # | Test | Status | Notes |
|---|------|--------|-------|
| 5.1 | Back button (top left) navigates to Dashboard | | |
| 5.2 | Document name shown in header | | |
| 5.3 | Split pane layout: PDF preview left (~60%), element tree + properties right (~40%) | | |
| 5.4 | PDF preview loads via iframe (calls `/api/v1/documents/render`) | | |
| 5.5 | Element tree shows pages with expand/collapse chevrons | | |
| 5.6 | Expand page — shows child elements with type icon + EID | | |
| 5.7 | Click element — selected highlight appears, properties show in panel | | |
| 5.8 | Property panel shows key-value pairs (booleans as Yes/No, objects as JSON) | | |
| 5.9 | Navigate to `/documents/nonexistent-id` — "Document not found" message | | |

---

## 6. Document Actions — Sign, Verify, Download (Functional + E2E)

| # | Test | Status | Notes |
|---|------|--------|-------|
| 6.1 | State badge shows correct color (Draft=gray, Review=amber, Signed=green) | | |
| 6.2 | Click "Verify" — calls API, shows signature count + valid status | | |
| 6.3 | Click "Download SPDF" — browser downloads `.spdf` file | | |
| 6.4 | Click "Download PDF" — spinner, then browser downloads `.pdf` file | | |
| 6.5 | Sign button visible only when state = "Review" | | |
| 6.6 | Click "Sign" — dialog opens with Name and Email fields | | |
| 6.7 | Fill name + email, click sign — document state changes to "Signed" | | |
| 6.8 | Verify after signing — shows updated signature count | | |

---

## 7. Templates Page (Functional + E2E)

| # | Test | Status | Notes |
|---|------|--------|-------|
| 7.1 | Page loads, shows loading spinner, then template list or empty state | | |
| 7.2 | Empty state: "No templates yet. Create one to get started." | | |
| 7.3 | Click "New Template" — dialog with Name (required) + Description (optional) | | |
| 7.4 | Fill name, click "Create Template" — template appears in list | | |
| 7.5 | Template card shows: name, description, "Updated {date}" | | |
| 7.6 | Click edit (pencil) on template — dialog pre-filled, edit and save | | |
| 7.7 | Click delete (trash) on template — template removed from list | | |
| 7.8 | Create 20+ templates — "Load More" button appears (cursor pagination) | | |

---

## 8. Settings Page (Functional)

| # | Test | Status | Notes |
|---|------|--------|-------|
| 8.1 | Three sections visible: API Key, Usage, Billing | | |
| 8.2 | API Key card shows key prefix (first 7 chars + "...") with "Active" badge | | |
| 8.3 | Click "Rotate Key" — new key displayed in amber warning box | | |
| 8.4 | Click copy button next to new key — key copied to clipboard, checkmark shows briefly | | |
| 8.5 | Usage card shows tier badge + usage families with progress bars | | |
| 8.6 | Billing card shows plan name + status badge + period end date | | |
| 8.7 | Click "Disconnect" — API key cleared, AuthGate reappears on refresh | | |

---

## 9. End-to-End Workflows

### 9A. Generate -> View -> Sign -> Verify

| # | Step | Status | Notes |
|---|------|--------|-------|
| 9A.1 | Navigate to `/generate`, use sample JSON, click "Generate SPDF" | | |
| 9A.2 | Redirected to document viewer, PDF preview loads | | |
| 9A.3 | Element tree populated, click elements to inspect properties | | |
| 9A.4 | Click "Download SPDF" — file saved locally | | |
| 9A.5 | Click "Download PDF" — PDF rendered and saved | | |
| 9A.6 | Click "Verify" — shows 0 signatures, valid | | |
| 9A.7 | If state is Review: click "Sign", enter name + email, submit | | |
| 9A.8 | State badge changes to "Signed" | | |
| 9A.9 | Click "Verify" again — shows 1 signature | | |

### 9B. Upload -> View -> Download

| # | Step | Status | Notes |
|---|------|--------|-------|
| 9B.1 | Go to Dashboard, upload an SPDF file via drag-drop or file picker | | |
| 9B.2 | Document card appears in grid | | |
| 9B.3 | Click card — viewer opens, PDF renders | | |
| 9B.4 | Inspect element tree and properties | | |
| 9B.5 | Download SPDF and PDF versions | | |

### 9C. Template CRUD Cycle

| # | Step | Status | Notes |
|---|------|--------|-------|
| 9C.1 | Navigate to Templates, create "Invoice Template" with description | | |
| 9C.2 | Edit template — change description, save | | |
| 9C.3 | Verify updated description shows in list | | |
| 9C.4 | Delete template — removed from list | | |

### 9D. API Key Rotation

| # | Step | Status | Notes |
|---|------|--------|-------|
| 9D.1 | Go to Settings, note current key prefix | | |
| 9D.2 | Click "Rotate Key" — new key shown | | |
| 9D.3 | Copy new key | | |
| 9D.4 | Refresh page — still authenticated with new key | | |
| 9D.5 | Navigate to other pages — API calls succeed with new key | | |

---

## 10. Error Handling & Edge Cases

| # | Test | Status | Notes |
|---|------|--------|-------|
| 10.1 | Stop API server (Ctrl+C on backend) — Studio shows errors on API calls | | |
| 10.2 | Navigate to `/anything-invalid` — 404 page with "Page Not Found" and back button | | |
| 10.3 | Upload non-SPDF file — error handling (should reject or show error) | | |
| 10.4 | Generate with empty JSON `{}` — backend returns validation error | | |
| 10.5 | Generate with malformed semantic structure — error displayed in UI | | |
| 10.6 | Resize browser window — dashboard grid reflows (3 col -> 2 -> 1) | | |
| 10.7 | Open DevTools console — no uncaught errors during normal navigation | | |

---

## 11. Visual / Theme (UI)

| # | Test | Status | Notes |
|---|------|--------|-------|
| 11.1 | Dark theme applied: dark background, light text | | |
| 11.2 | Cards have rounded corners, subtle borders, hover transitions | | |
| 11.3 | State badges use correct colors: Draft=gray, Review=amber, Signed=green, Archived=blue, Revoked=red | | |
| 11.4 | Loading spinners appear during async operations | | |
| 11.5 | Form error messages shown in red/destructive color | | |
| 11.6 | Sidebar collapse/expand transition is smooth (~200ms) | | |

---

## Summary

| Category | Total Tests | Pass | Fail | Skip |
|----------|-------------|------|------|------|
| 1. Authentication | 6 | | | |
| 2. Layout & Nav | 8 | | | |
| 3. Dashboard | 11 | | | |
| 4. Generate | 7 | | | |
| 5. Document Viewer | 9 | | | |
| 6. Doc Actions | 8 | | | |
| 7. Templates | 8 | | | |
| 8. Settings | 7 | | | |
| 9. E2E Workflows | 18 | | | |
| 10. Error/Edge | 7 | | | |
| 11. Visual/Theme | 6 | | | |
| **Total** | **95** | | | |

**Overall Result:** _______________
**Tested By:** _______________
**Date Completed:** _______________
