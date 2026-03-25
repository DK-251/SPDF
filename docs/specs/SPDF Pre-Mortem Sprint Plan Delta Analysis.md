# SPDF Pre-Mortem → Sprint Plan Delta Analysis
## What changes, what stays, and exactly where each change lands

---

## Method

For each of the 20 busters in the pre-mortem, I asked three questions:
1. Is it already in the sprint plan?
2. If not, does it require a new day, or does it fit inside an existing day?
3. Does it change the sequence of anything?

Result: **4 categories of change.**

| Category | Count | Action |
|---|---|---|
| ✅ Already in plan — no change needed | 7 | Document as confirmed |
| ➕ Missing — fits inside existing day (add to task prompt) | 8 | Amend existing day's Claude Code prompt |
| 📅 Missing — needs a dedicated half-day slot | 4 | Insert into existing week without adding a week |
| 🔴 Structural change required — sequence must change | 1 | Move item earlier in plan |

**Net result: Zero new weeks added. Zero days added. One sequence change. Eight prompt amendments. Four half-day insertions.**

---

## Category 1 — Already in Plan ✅

These busters are already covered by existing sprint plan tasks. No change needed. Confirmed covered.

| Buster | Where it exists in the plan |
|---|---|
| Path traversal protection | Week 2 Day 6 — container reader spec already states "validate ZIP entry paths" |
| SVG sanitisation | Week 2 Day 9 — asset management task already includes SVG sanitisation requirement |
| ZIP container comment | Week 2 Day 6 — container writer task already generates the ZIP |
| Confidence score on conversion | Week 3 Day 14 — conversion pipeline task includes `confidence_report` output |
| Stripe live mode switch | Week 11 Day 55 — already explicitly in plan |
| Audit log hash chain | Week 2 Day 10 — integration test already verifies audit chain integrity |
| Rate limit 429 with upgrade_url | Week 3 Day 15 — rate limiting task already specifies the upgrade_url in error response |

**These seven are confirmed. No changes.**

---

## Category 2 — Missing, Fits Inside Existing Day ➕

These busters are not in the current Claude Code prompts but do not require extra time — they add 5–15 lines to an existing task. Add them as additional requirements in the specified day's prompt.

---

### Amendment A — Day 6 (Container Reader): Add ZIP Bomb Protection

**Current Day 6 task:** Build container reader with path traversal prevention.

**Add this requirement to the Claude Code prompt:**

```
Also implement ZIP bomb protection in read_zip_entry_safe():
- Before extracting any entry, check: if compressed_size > 0 and
  (uncompressed_size / compressed_size) > 1000.0, return Err(SpdfError::ZipBombDetected)
- Enforce a running decompressed byte counter across the entire archive.
  If cumulative decompressed bytes exceed 2GB, abort with E_CONTAINER_TOO_LARGE.
- Use entry.take(uncompressed_size + 1024) when reading — never read unbounded.
Add a unit test: craft a mock ZIP entry with 1KB compressed / 10MB declared uncompressed.
Assert that read_zip_entry_safe() returns ZipBombDetected error.
```

**Time impact: 0 — fits within Day 6 budget.**

---

### Amendment B — Day 6 (Container Writer): Add ZIP Comment

**Current Day 6 task:** Build SpdfWriter.

**Add this requirement:**

```
When SpdfWriter::build() finalises the ZIP archive, set the ZIP archive comment to:
"This file is an SPDF document. Open it with any PDF viewer or visit spdf.dev for more information."
ZIP archive comments are part of the End of Central Directory record and are visible
when any ZIP utility opens the file. This prevents user confusion on systems without
a registered .spdf handler.
```

**Time impact: 0 — 3 lines of code.**

---

### Amendment C — Day 9 (Asset Management): Replace manual SVG regex with ammonia crate

**Current Day 9 task:** SVG sanitisation.

**Add this requirement:**

```
Use the ammonia crate (version 3.3) for SVG sanitisation rather than manual regex.
Add to Cargo.toml: ammonia = "3.3"

The ammonia-based sanitiser must:
1. Allow only the static SVG tags listed in the spec (path, rect, circle, etc.)
2. Remove script, foreignObject, animate, set, animateTransform
3. After ammonia sanitisation, run a secondary check: if the result still contains
   the string "href=\"http" or "href='http", return E_ASSET_UNSAFE
4. Return E_ASSET_UNSAFE if the sanitised SVG differs from the input in any way
   that changes visual output (compare rendered bounding boxes — if ammonia changed
   structural tags, flag for manual review rather than silently accepting)

Add a unit test: craft an SVG with an embedded <script> tag and an onclick handler.
Assert that sanitise_svg() returns a result with zero script or event attribute content.
Assert that the sanitised SVG still contains the original path data.
```

**Time impact: 0 — replaces the existing manual regex approach with a safer library.**

---

### Amendment D — Day 14 (Conversion Pipeline): Add Financial Integrity Validator

**Current Day 14 task:** Build Celery worker and PDF→SPDF pipeline.

**Add this requirement to the conversion pipeline task:**

```
After Step 4 (Claude semantic extraction), before Step 5 (DOM assembly),
add Step 4c: Financial Integrity Validation.

Create services/worker/pipeline/financial_validator.py:

class FinancialIntegrityValidator:
    def validate(self, extracted: dict) -> ValidationResult:
        errors = []
        
        # Rule 1: Each line item — qty * unit_price must equal total (±0.01 tolerance)
        for i, item in enumerate(extracted.get("line_items", [])):
            try:
                computed = Decimal(item["qty"]) * Decimal(item["unit_price"])
                stated = Decimal(item["total"])
                if abs(computed - stated) > Decimal("0.01"):
                    errors.append({"field": f"line_items[{i}].total",
                                   "stated": str(stated), "computed": str(computed),
                                   "action": "OVERRIDE_WITH_COMPUTED"})
                    item["total"] = str(computed)  # auto-correct silently
            except (InvalidOperation, KeyError):
                pass  # field missing or non-numeric — caught by schema validation
        
        # Rule 2: Sum of line item totals must equal subtotal (±0.01)
        if extracted.get("line_items") and extracted.get("subtotal"):
            computed_sub = sum(Decimal(i["total"]) for i in extracted["line_items"])
            stated_sub = Decimal(extracted["subtotal"])
            if abs(computed_sub - stated_sub) > Decimal("0.01"):
                errors.append({"field": "subtotal",
                               "stated": str(stated_sub), "computed": str(computed_sub),
                               "action": "OVERRIDE_WITH_COMPUTED"})
                extracted["subtotal"] = str(computed_sub)
        
        # Rule 3: subtotal + tax_amount must equal total (±0.01)
        # DO NOT auto-correct total — flag for human review only
        if all(k in extracted for k in ["subtotal", "tax_amount", "total"]):
            computed_total = Decimal(extracted["subtotal"]) + Decimal(extracted["tax_amount"])
            stated_total = Decimal(extracted["total"])
            if abs(computed_total - stated_total) > Decimal("0.01"):
                errors.append({"field": "total",
                               "stated": str(stated_total), "computed": str(computed_total),
                               "action": "FLAG_FOR_HUMAN_REVIEW"})
        
        return {"is_financially_valid": len([e for e in errors
                if e["action"] == "FLAG_FOR_HUMAN_REVIEW"]) == 0,
                "corrections": errors}

Call FinancialIntegrityValidator().validate(extracted_data) and:
- Log all auto-corrections to structlog
- Add corrections list to the job's result_data (for audit purposes)
- If any field is FLAG_FOR_HUMAN_REVIEW, set document confidence_score cap at 0.70
  regardless of Claude's reported confidence
```

**Time impact: 0 — fits within the Day 14 two-day window.**

---

### Amendment E — Day 14 (Conversion Pipeline): Add Prompt Injection Sanitiser

**Add this to the same Day 14 task:**

```
Before passing text_blocks to Claude in Step 4, run sanitise_for_llm() on each block.

Create services/worker/intelligence/injection_guard.py:

INJECTION_PATTERNS = [
    r"ignore\s+(all\s+)?previous\s+instructions",
    r"you\s+are\s+now\s+(a|an)",
    r"system\s*:\s*",
    r"<\s*/?\s*system\s*>",
    r"new\s+instruction",
    r"disregard\s+(the\s+)?(above|previous)",
    r"forget\s+(everything|all)",
]

def sanitise_for_llm(text: str) -> tuple[str, bool]:
    """Returns (sanitised_text, was_flagged)"""
    for pattern in INJECTION_PATTERNS:
        if re.search(pattern, text, re.IGNORECASE):
            flagged_text = f"[CONTENT_REVIEW_REQUIRED: text block flagged at extraction]"
            return flagged_text, True
    return text, False

Apply to every text block before building the Claude prompt.
If more than 3 blocks in a single document are flagged, set conversion_method="HEURISTIC"
and skip the Claude call entirely for that document (the document is likely adversarial).
Log all flagged blocks to the audit trail.
```

**Time impact: 0 — small addition to Day 14.**

---

### Amendment F — Day 37 (CI Pipeline): Add cargo-fuzz job

**Current Day 37 task:** Build GitHub Actions CI pipeline.

**Add this requirement:**

```
Add a fourth CI job: fuzz (runs on schedule: weekly, not on every PR — too slow for PR gate)

fuzz:
  runs-on: ubuntu-latest
  if: github.event_name == 'schedule'
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@nightly  # fuzz requires nightly
    - run: cargo install cargo-fuzz
    - name: Fuzz container parser (60 minutes)
      run: |
        cd crates/spdf-core
        cargo fuzz run fuzz_container_parser -- -max_total_time=3600
    - name: Fuzz semantic JSON parser (60 minutes)
      run: |
        cargo fuzz run fuzz_semantic_parser -- -max_total_time=3600
    - name: Upload crash artifacts
      if: failure()
      uses: actions/upload-artifact@v4
      with:
        name: fuzz-crashes
        path: crates/spdf-core/fuzz/artifacts/

Also create fuzz/fuzz_targets/ directory with two targets:
  fuzz_container_parser.rs — calls read_container() with arbitrary bytes, must never panic
  fuzz_semantic_parser.rs — calls parse_semantic() with arbitrary strings, must never panic

Add to crates/spdf-core/Cargo.toml:
[profile.fuzz]
inherits = "dev"
opt-level = 3
```

**Time impact: 0 — adds a scheduled CI job, not a blocking PR check.**

---

### Amendment G — Day 43 (GTM Content): Add Prompt Injection to "AI Accuracy" FAQ

**Current Day 43 task:** Write all launch content.

**Add this to the `LAUNCH/hacker_news_post.md` and `LAUNCH/devto_article.md` content briefs:**

```
In the technical article and HN post, include a section titled
"What about prompt injection attacks?"

Content: "A malicious PDF could embed invisible text designed to hijack the
Claude extraction prompt. SPDF defends against this with three layers:
(1) a pre-extraction sanitisation pass that detects and neutralises injection patterns,
(2) schema-constrained Claude output that rejects any response not matching the
SPDF element schema, and (3) a post-extraction financial arithmetic validator that
overrides any LLM-stated financial value that does not match deterministic calculation.
An attacker who manipulates Claude's output cannot change a total that does not
match qty × unit_price."

This section will resonate strongly with security-conscious developers on HN.
It demonstrates that the AI accuracy problem was thought through, not ignored.
```

**Time impact: 0 — adds one section to existing writing tasks.**

---

### Amendment H — Day 44 (Security Hardening): Add CSP headers to Vercel config

**Current Day 44 task:** Security hardening.

**Add this requirement:**

```
Create apps/studio/vercel.json (or update if it exists):

{
  "headers": [
    {
      "source": "/(.*)",
      "headers": [
        {
          "key": "Content-Security-Policy",
          "value": "default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; img-src 'self' blob: data:; connect-src 'self' https://api.spdf.dev https://clerk.spdf.dev; object-src 'none'; base-uri 'self'; frame-ancestors 'none'"
        },
        { "key": "X-Content-Type-Options", "value": "nosniff" },
        { "key": "X-Frame-Options", "value": "DENY" },
        { "key": "Referrer-Policy", "value": "strict-origin-when-cross-origin" }
      ]
    }
  ]
}

NOTE: 'wasm-unsafe-eval' is required for the WASM module to execute in the browser.
Without it, the SPDF validation WASM will be blocked by CSP.
Test after deployment: open browser dev tools → Console → verify zero CSP violations
when loading the Studio and viewing a document.
```

**Time impact: 0 — configuration file, not code.**

---

## Category 3 — Missing, Needs a Dedicated Slot 📅

These four busters are substantive enough to need dedicated time but small enough to fit within existing weeks without displacing other work. Each takes a half-day (2 hours).

---

### New Slot 1 — Week 7 Day 33 (afternoon): 500-File Golden Corpus + Pixel Diff

**Insert as the afternoon session of Day 33 (currently: test suite writing).**

**Why Day 33:** The test suite is being built that week. The golden corpus is a test artifact. It belongs in the same week.

**Task:**

```
Build the render regression testing infrastructure.

1. Create scripts/build_golden_corpus.py:
   Generates 50 synthetic SPDF test documents covering edge cases:
   - 10 documents with Indian rupee (₹) symbol in financial fields
   - 10 documents with German locale (comma as thousands separator, period in display)
   - 10 multi-page documents (7+ pages, 80+ line items)
   - 10 documents with very long vendor/client names (>100 chars)
   - 10 documents with narrow table columns (5 columns on A4)

   For each, generate the SPDF and render to PNG at 150 DPI.
   Save as tests/golden/render/{document_name}.png

2. Create tests/test_render_regression.py:
   def test_render_fidelity_all_golden_files():
     for golden_png in glob("tests/golden/render/*.png"):
         spdf_file = golden_png.replace(".png", ".spdf")
         rendered = render_spdf_to_png(spdf_file, dpi=150)
         diff_pct = pixel_diff_percentage(rendered, load_png(golden_png))
         assert diff_pct < 0.001, f"Render regression {diff_pct:.4%} in {spdf_file}"

3. Add to CI yml (python-backend job):
   - name: Render regression tests
     run: pytest tests/test_render_regression.py -v

Note: 50 synthetic documents is sufficient for launch. The 500-file corpus
described in the pre-mortem is the production target — expand over time by
adding real customer documents when the conversion feature is re-enabled.
```

**Time impact: 2 hours on Day 33 afternoon. No other tasks displaced.**

---

### New Slot 2 — Week 9 Day 41 (afternoon): Receiver-First Landing Page

**Insert as the afternoon session of Day 41 (currently: documentation site — morning).**

**Why Day 41:** GTM week. The docs go up in the morning; this goes up in the afternoon.

**Task:**

```
Create apps/landing/receive.html — a separate landing page for AP managers and
finance teams, not developers.

This page must feel completely different from spdf.dev (the developer landing page).
No code examples. No SDK references. Language is business, not technical.

Sections:
1. HERO:
   Headline: "Process invoices in 1 day, not 14"
   Subheadline: "Ask your suppliers to send SPDF invoices. Your accounting system
   reads them automatically — no manual data entry, no errors."
   Single CTA: [Download the supplier email template →]
   (Downloads a pre-written email to send to suppliers)

2. THE PROBLEM (3 stats, large and bold):
   "57% of invoices still require manual data entry"
   "39% of invoices contain errors requiring follow-up"
   "Average invoice processing time: 14.6 days"

3. HOW IT WORKS (3 steps, no technical detail):
   Step 1: Ask your suppliers to use SPDF (share the template)
   Step 2: Receive SPDF invoices — they look identical to PDF
   Step 3: Your system extracts all data automatically — zero manual entry

4. SUPPLIER EMAIL TEMPLATE (inline, copyable):
   The full "Send Me SPDF" email from the pre-mortem Buster 1.1 — formatted
   as a copyable text block with a "Copy to clipboard" button.

5. FAQ for AP managers:
   "Does this change how invoices look?" No — SPDF invoices look identical to PDF.
   "Do our suppliers need to pay anything?" No — the SDK is free.
   "Does this work with SAP/Oracle/QuickBooks?" Data extraction API returns standard JSON.

Add link to receive.html in the main landing page footer: "For AP Teams →"
```

**Time impact: 2 hours on Day 41 afternoon. No other tasks displaced.**

---

### New Slot 3 — Week 9 Day 42 (afternoon): Migration Calculator Tool

**Insert as the afternoon session of Day 42 (currently: spec README — morning).**

**Task:**

```
Create apps/landing/migrate.html — the developer migration calculator.

A single-page tool where a developer can estimate their savings from switching
to SPDF from reportlab/iText/wkhtmltopdf.

Inputs (sliders/inputs, no form submission — all client-side JavaScript):
- "Lines of code in your current PDF generation code" (slider: 50–500)
- "Hours per month fixing PDF rendering bugs" (slider: 0–20)
- "How many invoices do you generate per month?" (dropdown: <100, 100-1K, 1K-10K, 10K+)
- "Current tool" (dropdown: reportlab, iText, wkhtmltopdf, fpdf, other)

Outputs (update in real-time as sliders move):
- Estimated lines of code with SPDF (formula: input_lines × 0.12)
- Lines saved: input_lines - spdf_lines
- Hours saved per month: (hours_fixing_bugs × 0.85) + (invoices_per_month × 0.001)
- "Your invoice data is currently locked in black-box PDFs.
   With SPDF, every field is extractable in one API call."

Below the calculator:
- Side-by-side code comparison: their current tool's invoice code (static examples
  for each tool in the dropdown) vs the 20-line SPDF equivalent
- CTA: [Install the SDK — pip install spdf-sdk]

Add to main navigation: "Migration Calculator"
```

**Time impact: 2 hours on Day 42 afternoon. No other tasks displaced.**

---

### New Slot 4 — Week 9 Day 44 (afternoon): Audit Language Audit + Enterprise Page

**Insert as the afternoon session of Day 44 (currently: security hardening — morning).**

**Task:**

```
Two tasks — 1 hour each.

TASK A: Audit language correction (1 hour)
Search the entire repository for these exact phrases and replace them:

Find: "unforgeable"
Replace with: "tamper-evident"

Find: "cannot be altered"
Replace with: "any alteration is cryptographically detectable"

Find: "proves the document is authentic"
Replace with: "provides cryptographic evidence of document integrity"

Find: "legally binding audit trail"
Replace with: "cryptographically verifiable audit trail"

Files to check: all .md files, all .mdx files, apps/landing/*.html,
the SPDF Format Specification SPEC.md.

After replacement, add this note to docs/concepts/security.mdx:
"Note on legal admissibility: SPDF's audit trail provides cryptographic tamper
detection. Whether it constitutes legally admissible evidence in a specific
jurisdiction depends on applicable law and judicial interpretation. Consult
legal counsel for document retention and evidentiary requirements in your
jurisdiction."

TASK B: Enterprise readiness page (1 hour)
Create apps/landing/enterprise.html with these sections:
1. "Enterprise-ready document infrastructure"
2. Security section: SOC 2 (honest: "planned for 2026"), data residency, encryption at rest
3. Compliance section: GDPR (PII tagging built into format), HIPAA (PHI element encryption)
4. Integration section: REST API, Python SDK, TypeScript SDK, JSON output for any ERP
5. Support section: dedicated Slack channel, SLA (99.9% uptime for Enterprise tier)
6. Pilot program: 30-day free pilot, up to 10,000 conversions, migration assistance
7. Contact form (simple: name, company, email, use case — submits to a Resend email)

Add link in main navigation: "Enterprise"
```

**Time impact: 2 hours on Day 44 afternoon. No other tasks displaced.**

---

## Category 4 — Structural Change Required 🔴

### One Sequence Change: Tier Design Must Move to Week 4 Day 16

**Current plan:** Billing endpoints built on Day 16. Tier definitions inherited from the existing plan (volume-based).

**Required change:** The capability-based tier redesign (Buster 5.1) must be decided and implemented on Day 16 — the day billing is built. If volume-based tiering is built first and used in production, changing to capability-based tiering requires a database migration, Stripe product recreation, and re-writing middleware. It is a 3-day retrofit versus a 1-day first-build.

**What changes on Day 16:**

Replace the current Day 16 billing task's tier definition with this:

```
REVISED TIER CAPABILITY MATRIX — implement this, not volume-only limits:

FREE tier capabilities:
  - Native SPDF generation: UNLIMITED
  - Basic extraction (invoice fields only): 100/day
  - PDF → SPDF conversion: DISABLED (returns 402 with upgrade prompt)
  - Semantic diff: DISABLED
  - Document signing: DISABLED
  - AI /ask queries: DISABLED
  - Custom templates: max 0 (use public templates only)
  - API render to PDF: DISABLED (SDK export works locally, API render is paid)
  - Audit log export: DISABLED
  - Org seats: 1
  - Rate limit: 500 req/day on enabled endpoints

PRO tier ($29/month) capabilities:
  - Everything FREE, plus:
  - PDF → SPDF conversion: 1,000/month
  - Semantic diff: ENABLED
  - Document signing: ENABLED (simplified SHA-256 — X.509 is Enterprise)
  - AI /ask queries: 500/month
  - Custom templates: 10
  - API render to PDF: ENABLED
  - Audit log export: ENABLED
  - Rate limit: 50,000 req/day

TEAM tier ($99/month):
  - Everything PRO, plus:
  - PDF → SPDF conversion: 10,000/month
  - AI /ask queries: UNLIMITED
  - Custom templates: UNLIMITED
  - Org seats: 10
  - Rate limit: 500,000 req/day
  - Priority support

ENTERPRISE (custom):
  - Everything TEAM, plus:
  - X.509 certificate signing (full PKI)
  - On-premise deployment option
  - Custom rate limits
  - Dedicated support SLA (99.9% uptime)
  - SAP/Oracle connector assistance

Implementation:
Create services/api/middleware/capabilities.py:

TIER_CAPABILITIES = {
    "FREE":       {"signing": False, "diff": False, "conversion": False,
                   "ask": False, "render_api": False, "audit_export": False,
                   "custom_templates": 0, "conversion_monthly": 0,
                   "ask_monthly": 0},
    "PRO":        {"signing": True,  "diff": True,  "conversion": True,
                   "ask": True, "render_api": True, "audit_export": True,
                   "custom_templates": 10, "conversion_monthly": 1000,
                   "ask_monthly": 500},
    "TEAM":       {"signing": True,  "diff": True,  "conversion": True,
                   "ask": True, "render_api": True, "audit_export": True,
                   "custom_templates": -1, "conversion_monthly": 10000,
                   "ask_monthly": -1},  # -1 = unlimited
    "ENTERPRISE": {"signing": True,  "diff": True,  "conversion": True,
                   "ask": True, "render_api": True, "audit_export": True,
                   "custom_templates": -1, "conversion_monthly": -1,
                   "ask_monthly": -1},
}

async def require_capability(capability: str, user: User):
    caps = TIER_CAPABILITIES[user.tier]
    if not caps.get(capability, False):
        raise HTTPException(
            status_code=402,
            detail={
                "code": "TIER_REQUIRED",
                "message": f"{capability.replace('_', ' ').title()} is not available on the {user.tier} tier.",
                "upgrade_url": "https://spdf.dev/pricing",
                "current_tier": user.tier,
                "required_tier": "PRO"
            }
        )

Apply require_capability() to:
  POST /v1/documents/{id}/sign → require_capability("signing", user)
  GET /v1/documents/{id}/diff/{id2} → require_capability("diff", user)
  POST /v1/documents/upload → require_capability("conversion", user)
  POST /v1/documents/{id}/ask → require_capability("ask", user)
  POST /v1/documents/{id}/render → require_capability("render_api", user)

In Stripe, create products matching PRO and TEAM with these EXACT names
(used in webhook handler to map Stripe price → tier):
  Product: "SPDF Pro", Price: $29/month, metadata: {tier: "PRO"}
  Product: "SPDF Team", Price: $99/month, metadata: {tier: "TEAM"}
```

**Also update Day 24 (Studio upgrade prompts) to reference capability gates, not rate limits:**

```
When a FREE user encounters a capability gate (402 response), show a contextual modal:

For signing:  "Document signing requires Pro. Sign documents with cryptographic integrity
              that proves they haven't been tampered with. [Upgrade to Pro — $29/month]"

For diff:     "Semantic diff requires Pro. Compare any two SPDF documents at the element
              level — see exactly which fields changed and by how much. [Upgrade to Pro]"

For convert:  "PDF conversion requires Pro. Convert your existing PDF invoice archive
              to structured SPDF files automatically. [Upgrade to Pro — $29/month]"

These modals appear inline in the flow — not on a settings page the user never visits.
```

---

## The Complete Change Summary

Here is exactly what changes in the sprint plan, listed by day:

| Day | Change Type | What Changes |
|---|---|---|
| **Day 6** | Amend prompt | Add ZIP bomb protection + ZIP archive comment to container layer tasks |
| **Day 9** | Amend prompt | Replace manual SVG regex with ammonia crate in asset management task |
| **Day 14** | Amend prompt | Add financial integrity validator + prompt injection sanitiser to conversion pipeline task |
| **Day 16** | 🔴 Structural | Replace volume-based tier definitions with capability-based tier matrix + capabilities middleware |
| **Day 24** | Amend prompt | Update upgrade prompts in Studio to reference capability gates (402), not rate limits (429) |
| **Day 33** | New half-day slot | Add golden corpus + pixel-diff regression test infrastructure (afternoon) |
| **Day 37** | Amend prompt | Add cargo-fuzz scheduled CI job to the CI pipeline task |
| **Day 41** | New half-day slot | Add receiver-first AP manager landing page (afternoon) |
| **Day 42** | New half-day slot | Add migration calculator tool (afternoon) |
| **Day 43** | Amend prompt | Add prompt injection defence section to HN post and Dev.to article content |
| **Day 44** | New half-day slot | Audit language correction + enterprise page (afternoon) |
| **Day 44** | Amend prompt | Add CSP headers to Vercel config in security hardening task |

---

## What Does NOT Change

Everything else in the 12-week plan is unchanged. Specifically:

- **All 12 weeks and all 60 days remain** — nothing is added or removed from the schedule
- **Week sequence is unchanged** — environment → core engine → API → studio → SDKs → testing → GTM → launch
- **The deferred feature register is unchanged** — PDF conversion, X.509 signing, NL query, webhooks all stay deferred under their original revenue triggers
- **The two-machine workflow is unchanged**
- **The cost profile is unchanged** — $0 through Week 10, $15–25 from Week 11
- **The GTM launch sequence is unchanged**

---

## Effort Reconciliation

| Change type | Count | Hours added |
|---|---|---|
| Prompt amendments (add to existing Claude Code task) | 8 | ~0 (Claude writes the code) |
| Half-day insertions (new tasks in existing afternoons) | 4 | 8 hours total (4 × 2h) |
| Structural tier redesign (Day 16 rewrite) | 1 | ~2 hours (Claude rewrites the billing task) |
| **Total additional effort** | **13 changes** | **~10 hours** |

10 hours across a 264-hour plan is a **3.8% overhead** to close every critical pre-mortem risk identified.

---

*Delta analysis complete. The sprint plan requires targeted amendments, not a rewrite.*
