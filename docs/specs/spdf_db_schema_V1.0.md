# SPDF Database Schema Design
## SPDF-DB-2025-001 · Version 1.0 · Status: APPROVED — Ready for Implementation

**Predecessor:** SPDF-API-2025-001 (API Contract Specification)  
**Successor:** SPDF-SPRINT-001 (Development Sprint Plan)  
**Database:** PostgreSQL 15 hosted on Supabase  
**Classification:** Internal Engineering — Confidential

---

## Decisions Recorded

| Question | Decision | Rationale |
|---|---|---|
| Document versioning | No versioning in MVP — overwrite in place | Simpler, less storage. SPDF audit.json inside the file handles content history |
| Multi-tenancy | Row-Level Security (RLS) via Supabase policies | Database enforces ownership — defence in depth |
| Asset deduplication | Per organisation — shared asset pool across org members | Same logo stored once per org. No cross-org privacy surface |

---

## Section 01 — Design Principles

### 1.1 Schema Conventions

| Convention | Rule |
|---|---|
| Primary keys | `UUID` via `uuid_generate_v4()`. Never sequential integers |
| Foreign keys | Always explicit with `ON DELETE` behaviour declared |
| Timestamps | All `TIMESTAMPTZ`. Always stored as UTC |
| Soft deletes | Tables with user data have `deleted_at TIMESTAMPTZ`. No hard deletes |
| Naming | Tables: `snake_case` plural. Columns: `snake_case` |
| Booleans | Always `BOOLEAN NOT NULL` with explicit default. Never nullable |
| Money/decimals | `NUMERIC(20, 6)`. Never `FLOAT` or `DOUBLE PRECISION` |
| JSON columns | `JSONB` always. Never `JSON` (text) |
| Enums | `CHECK` constraints on `TEXT` columns — not PostgreSQL `ENUM` types |
| Text lengths | `TEXT` for variable strings. `VARCHAR(n)` only where limit is a business rule |

### 1.2 RLS Strategy

| Connection context | Role used | RLS applies |
|---|---|---|
| API requests (Clerk JWT) | `authenticated` | Yes — ownership enforced automatically |
| Worker service | `service_role` | No — trusted internal service |
| Migrations (Alembic) | `postgres` superuser | No |

**Standard RLS pattern used on all tables:**

```sql
USING (
  owner_user_id = auth.uid()
  OR owner_org_id IN (
    SELECT org_id FROM org_members WHERE user_id = auth.uid()
  )
)
```

### 1.3 Table Inventory

| Table | Purpose | RLS |
|---|---|---|
| `users` | User accounts mirrored from Clerk | Yes |
| `organizations` | Organisation accounts mirrored from Clerk | Yes |
| `org_members` | User ↔ Organisation membership and roles | Yes |
| `documents` | SPDF document records | Yes |
| `conversion_jobs` | Async job queue records | Yes |
| `templates` | Document template definitions | Yes |
| `assets` | Content-addressed binary asset registry | Yes (org-scoped) |
| `document_assets` | Join table: documents ↔ assets | Yes |
| `subscriptions` | Stripe subscription records | Yes |
| `usage_events` | Per-operation billing events | Yes |
| `api_keys` | Hashed API key records | Yes |
| `audit_log` | System-level audit trail | Yes |

---

## Section 02 — Extensions and Setup

```sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";
CREATE EXTENSION IF NOT EXISTS "btree_gin";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
```

**Enum value reference (implemented as CHECK constraints):**

```
User tier:       FREE | PRO | TEAM | ENTERPRISE
Org tier:        TEAM | ENTERPRISE
Org member role: OWNER | ADMIN | MEMBER | VIEWER
Document state:  DRAFT | REVIEW | SIGNED | CERTIFIED
Source format:   NATIVE | PDF_CONVERTED
Job type:        PDF_TO_SPDF | GENERATE | SIGN | BATCH
Job status:      QUEUED | PROCESSING | COMPLETED | FAILED | CANCELLED
Event type:      CONVERSION | GENERATION | EXTRACTION | API_CALL | SIGNING | AI_QUERY
Sub status:      ACTIVE | TRIALING | PAST_DUE | CANCELLED
```

---

## Section 03 — Users and Organisations

### 3.1 users

```sql
CREATE TABLE users (
  id                    UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
  clerk_user_id         TEXT         UNIQUE NOT NULL,
  email                 TEXT         UNIQUE NOT NULL,
  display_name          TEXT,
  avatar_url            TEXT,
  tier                  TEXT         NOT NULL DEFAULT 'FREE'
                                     CHECK (tier IN ('FREE','PRO','TEAM','ENTERPRISE')),
  api_key_hash          TEXT,
  api_key_prefix        TEXT,
  api_key_created_at    TIMESTAMPTZ,
  daily_quota_conversions  INTEGER   NOT NULL DEFAULT 10,
  daily_quota_generations  INTEGER   NOT NULL DEFAULT 50,
  daily_quota_extractions  INTEGER   NOT NULL DEFAULT 100,
  stripe_customer_id    TEXT         UNIQUE,
  created_at            TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
  updated_at            TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
  deleted_at            TIMESTAMPTZ
);

CREATE INDEX idx_users_clerk_id ON users (clerk_user_id);
CREATE INDEX idx_users_email    ON users (email) WHERE deleted_at IS NULL;

ALTER TABLE users ENABLE ROW LEVEL SECURITY;

CREATE POLICY users_select_own ON users FOR SELECT USING (id = auth.uid());
CREATE POLICY users_update_own ON users FOR UPDATE USING (id = auth.uid());

CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN NEW.updated_at = NOW(); RETURN NEW; END;
$$;

CREATE TRIGGER trg_users_updated_at
  BEFORE UPDATE ON users
  FOR EACH ROW EXECUTE FUNCTION update_updated_at();
```

**Column notes:**
- `clerk_user_id` — Clerk's opaque user ID. Used for webhook matching
- `api_key_hash` — bcrypt hash cost factor 12. Full key shown once on creation
- `api_key_prefix` — first 8 chars (e.g. `sk_live_aBcDeFgH`). Display only
- `daily_quota_*` — denormalised from tier defaults. Allows per-user enterprise overrides

---

### 3.2 organizations

```sql
CREATE TABLE organizations (
  id                    UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
  clerk_org_id          TEXT         UNIQUE NOT NULL,
  name                  TEXT         NOT NULL,
  slug                  TEXT         UNIQUE NOT NULL,
  avatar_url            TEXT,
  tier                  TEXT         NOT NULL DEFAULT 'TEAM'
                                     CHECK (tier IN ('TEAM','ENTERPRISE')),
  stripe_customer_id    TEXT         UNIQUE,
  daily_quota_conversions  INTEGER,
  daily_quota_generations  INTEGER,
  daily_quota_extractions  INTEGER,
  created_at            TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
  updated_at            TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
  deleted_at            TIMESTAMPTZ
);

CREATE INDEX idx_orgs_clerk_id ON organizations (clerk_org_id);
CREATE INDEX idx_orgs_slug     ON organizations (slug) WHERE deleted_at IS NULL;

ALTER TABLE organizations ENABLE ROW LEVEL SECURITY;

CREATE POLICY orgs_select_member ON organizations
  FOR SELECT USING (
    id IN (SELECT org_id FROM org_members WHERE user_id = auth.uid())
  );

CREATE POLICY orgs_update_admin ON organizations
  FOR UPDATE USING (
    id IN (
      SELECT org_id FROM org_members
      WHERE user_id = auth.uid() AND role IN ('OWNER','ADMIN')
    )
  );

CREATE TRIGGER trg_orgs_updated_at
  BEFORE UPDATE ON organizations
  FOR EACH ROW EXECUTE FUNCTION update_updated_at();
```

---

### 3.3 org_members

```sql
CREATE TABLE org_members (
  org_id      UUID  NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
  user_id     UUID  NOT NULL REFERENCES users (id) ON DELETE CASCADE,
  role        TEXT  NOT NULL CHECK (role IN ('OWNER','ADMIN','MEMBER','VIEWER')),
  invited_by  UUID  REFERENCES users (id) ON DELETE SET NULL,
  joined_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (org_id, user_id)
);

CREATE INDEX idx_org_members_user ON org_members (user_id);
CREATE INDEX idx_org_members_org  ON org_members (org_id);

ALTER TABLE org_members ENABLE ROW LEVEL SECURITY;

CREATE POLICY org_members_select ON org_members
  FOR SELECT USING (
    org_id IN (SELECT org_id FROM org_members WHERE user_id = auth.uid())
  );

CREATE POLICY org_members_modify ON org_members
  FOR ALL USING (
    org_id IN (
      SELECT org_id FROM org_members
      WHERE user_id = auth.uid() AND role IN ('OWNER','ADMIN')
    )
  );
```

---

## Section 04 — Documents

### 4.1 documents

```sql
CREATE TABLE documents (
  id                  UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
  spdf_doc_id         TEXT         UNIQUE NOT NULL,
  title               TEXT         NOT NULL,
  owner_user_id       UUID         REFERENCES users (id) ON DELETE SET NULL,
  owner_org_id        UUID         REFERENCES organizations (id) ON DELETE SET NULL,
  document_type       TEXT         NOT NULL DEFAULT 'General',
  locale              TEXT         NOT NULL DEFAULT 'en-US',
  spdf_version        TEXT         NOT NULL DEFAULT '1.0',
  state               TEXT         NOT NULL DEFAULT 'DRAFT'
                                   CHECK (state IN ('DRAFT','REVIEW','SIGNED','CERTIFIED')),
  r2_key              TEXT         NOT NULL,
  r2_bucket           TEXT         NOT NULL DEFAULT 'spdf-documents',
  file_size_bytes     BIGINT,
  page_count          SMALLINT,
  has_render_layer    BOOLEAN      NOT NULL DEFAULT TRUE,
  source_format       TEXT         NOT NULL DEFAULT 'NATIVE'
                                   CHECK (source_format IN ('NATIVE','PDF_CONVERTED')),
  template_id         UUID         REFERENCES templates (id) ON DELETE SET NULL,
  conversion_job_id   UUID,
  confidence_score    NUMERIC(4,3) CHECK (confidence_score >= 0 AND confidence_score <= 1),
  model_used          TEXT,
  meta_author         TEXT,
  meta_currency       TEXT,
  meta_tags           TEXT[],
  metadata            JSONB        NOT NULL DEFAULT '{}',
  created_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
  updated_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
  deleted_at          TIMESTAMPTZ
);

CREATE INDEX idx_docs_owner_user  ON documents (owner_user_id, created_at DESC)
  WHERE deleted_at IS NULL;
CREATE INDEX idx_docs_owner_org   ON documents (owner_org_id, created_at DESC)
  WHERE deleted_at IS NULL;
CREATE INDEX idx_docs_state       ON documents (owner_user_id, state)
  WHERE deleted_at IS NULL;
CREATE INDEX idx_docs_title_trgm  ON documents USING GIN (title gin_trgm_ops)
  WHERE deleted_at IS NULL;
CREATE INDEX idx_docs_metadata    ON documents USING GIN (metadata);
CREATE INDEX idx_docs_tags        ON documents USING GIN (meta_tags);
CREATE UNIQUE INDEX idx_docs_spdf_id ON documents (spdf_doc_id)
  WHERE deleted_at IS NULL;

ALTER TABLE documents ADD CONSTRAINT chk_docs_one_owner
  CHECK (
    (owner_user_id IS NOT NULL AND owner_org_id IS NULL)
    OR (owner_user_id IS NULL AND owner_org_id IS NOT NULL)
  );

ALTER TABLE documents ENABLE ROW LEVEL SECURITY;

CREATE POLICY docs_select ON documents
  FOR SELECT USING (
    deleted_at IS NULL AND (
      owner_user_id = auth.uid()
      OR owner_org_id IN (SELECT org_id FROM org_members WHERE user_id = auth.uid())
    )
  );

CREATE POLICY docs_insert ON documents
  FOR INSERT WITH CHECK (
    owner_user_id = auth.uid()
    OR owner_org_id IN (
      SELECT org_id FROM org_members
      WHERE user_id = auth.uid() AND role IN ('OWNER','ADMIN','MEMBER')
    )
  );

CREATE POLICY docs_update ON documents
  FOR UPDATE USING (
    state NOT IN ('SIGNED','CERTIFIED') AND (
      owner_user_id = auth.uid()
      OR owner_org_id IN (
        SELECT org_id FROM org_members
        WHERE user_id = auth.uid() AND role IN ('OWNER','ADMIN','MEMBER')
      )
    )
  );

CREATE TRIGGER trg_docs_updated_at
  BEFORE UPDATE ON documents
  FOR EACH ROW EXECUTE FUNCTION update_updated_at();
```

**Column notes:**
- `spdf_doc_id` — spec-defined document ID from SPDF Technical Spec §2.4. Format: `spdf-{uuid4}`. Immutable
- `r2_key` — overwritten on each DRAFT edit. Previous file cleaned up by nightly R2 lifecycle job
- `chk_docs_one_owner` — exactly one of `owner_user_id` or `owner_org_id` must be non-null

---

## Section 05 — Conversion Jobs

### 5.1 conversion_jobs

```sql
CREATE TABLE conversion_jobs (
  id                UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id           UUID        REFERENCES users (id) ON DELETE SET NULL,
  org_id            UUID        REFERENCES organizations (id) ON DELETE SET NULL,
  job_type          TEXT        NOT NULL
                                CHECK (job_type IN ('PDF_TO_SPDF','GENERATE','SIGN','BATCH')),
  status            TEXT        NOT NULL DEFAULT 'QUEUED'
                                CHECK (status IN ('QUEUED','PROCESSING','COMPLETED','FAILED','CANCELLED')),
  input_r2_key      TEXT,
  input_file_name   TEXT,
  input_file_size   BIGINT,
  input_data        JSONB,
  output_doc_id     UUID        REFERENCES documents (id) ON DELETE SET NULL,
  template_id       UUID        REFERENCES templates (id) ON DELETE SET NULL,
  celery_task_id    TEXT        UNIQUE,
  attempt_count     SMALLINT    NOT NULL DEFAULT 0,
  max_attempts      SMALLINT    NOT NULL DEFAULT 3,
  current_step      TEXT,
  progress          SMALLINT    NOT NULL DEFAULT 0
                                CHECK (progress >= 0 AND progress <= 100),
  model_used        TEXT,
  conversion_method TEXT,
  result_data       JSONB,
  error_code        TEXT,
  error_message     TEXT,
  is_retryable      BOOLEAN,
  webhook_url       TEXT,
  webhook_secret    TEXT,
  webhook_delivered BOOLEAN     NOT NULL DEFAULT FALSE,
  webhook_attempts  SMALLINT    NOT NULL DEFAULT 0,
  queued_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  started_at        TIMESTAMPTZ,
  completed_at      TIMESTAMPTZ,
  expires_at        TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '7 days'
);

CREATE INDEX idx_jobs_user_status ON conversion_jobs (user_id, status, queued_at DESC);
CREATE INDEX idx_jobs_dequeue     ON conversion_jobs (queued_at ASC)
  WHERE status = 'QUEUED';
CREATE INDEX idx_jobs_celery      ON conversion_jobs (celery_task_id)
  WHERE celery_task_id IS NOT NULL;
CREATE INDEX idx_jobs_webhook     ON conversion_jobs (queued_at)
  WHERE status = 'COMPLETED' AND webhook_url IS NOT NULL AND webhook_delivered = FALSE;

ALTER TABLE documents
  ADD CONSTRAINT fk_docs_conversion_job
  FOREIGN KEY (conversion_job_id) REFERENCES conversion_jobs (id) ON DELETE SET NULL;

ALTER TABLE conversion_jobs ENABLE ROW LEVEL SECURITY;

CREATE POLICY jobs_select ON conversion_jobs
  FOR SELECT USING (
    user_id = auth.uid()
    OR org_id IN (SELECT org_id FROM org_members WHERE user_id = auth.uid())
  );

CREATE POLICY jobs_insert ON conversion_jobs
  FOR INSERT WITH CHECK (user_id = auth.uid());
```

---

## Section 06 — Templates

### 6.1 templates

```sql
CREATE TABLE templates (
  id               UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
  owner_user_id    UUID        REFERENCES users (id) ON DELETE SET NULL,
  owner_org_id     UUID        REFERENCES organizations (id) ON DELETE SET NULL,
  name             TEXT        NOT NULL,
  description      TEXT,
  category         TEXT        NOT NULL DEFAULT 'General',
  slug             TEXT        UNIQUE,
  is_public        BOOLEAN     NOT NULL DEFAULT FALSE,
  r2_key           TEXT        NOT NULL,
  preview_r2_key   TEXT,
  spdf_version     TEXT        NOT NULL DEFAULT '1.0',
  variable_schema  JSONB       NOT NULL DEFAULT '{"required":[],"optional":[]}',
  use_count        INTEGER     NOT NULL DEFAULT 0,
  created_by       UUID        REFERENCES users (id) ON DELETE SET NULL,
  created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at       TIMESTAMPTZ
);

CREATE INDEX idx_templates_owner_user ON templates (owner_user_id)
  WHERE deleted_at IS NULL;
CREATE INDEX idx_templates_owner_org  ON templates (owner_org_id)
  WHERE deleted_at IS NULL;
CREATE INDEX idx_templates_public     ON templates (category, use_count DESC)
  WHERE is_public = TRUE AND deleted_at IS NULL;
CREATE UNIQUE INDEX idx_templates_slug ON templates (slug)
  WHERE slug IS NOT NULL AND deleted_at IS NULL;

ALTER TABLE templates ENABLE ROW LEVEL SECURITY;

CREATE POLICY templates_select ON templates
  FOR SELECT USING (
    deleted_at IS NULL AND (
      is_public = TRUE
      OR owner_user_id = auth.uid()
      OR owner_org_id IN (SELECT org_id FROM org_members WHERE user_id = auth.uid())
    )
  );

CREATE POLICY templates_insert ON templates
  FOR INSERT WITH CHECK (
    owner_user_id = auth.uid()
    OR owner_org_id IN (
      SELECT org_id FROM org_members
      WHERE user_id = auth.uid() AND role IN ('OWNER','ADMIN')
    )
  );

CREATE TRIGGER trg_templates_updated_at
  BEFORE UPDATE ON templates
  FOR EACH ROW EXECUTE FUNCTION update_updated_at();
```

---

## Section 07 — Asset Registry

### 7.1 assets

```sql
CREATE TABLE assets (
  id               UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
  asset_id         TEXT        NOT NULL,
  owner_user_id    UUID        REFERENCES users (id) ON DELETE SET NULL,
  owner_org_id     UUID        REFERENCES organizations (id) ON DELETE SET NULL,
  asset_type       TEXT        NOT NULL
                               CHECK (asset_type IN ('FONT','IMAGE','VECTOR','ATTACHMENT','ICC_PROFILE')),
  mime_type        TEXT        NOT NULL,
  original_filename TEXT,
  sha256_hash      TEXT        NOT NULL,
  file_size_bytes  BIGINT      NOT NULL,
  r2_key           TEXT        NOT NULL,
  r2_bucket        TEXT        NOT NULL DEFAULT 'spdf-documents',
  font_family      TEXT,
  font_weight      SMALLINT,
  font_style       TEXT        CHECK (font_style IN ('NORMAL','ITALIC','OBLIQUE')),
  font_subsetted   BOOLEAN,
  reference_count  INTEGER     NOT NULL DEFAULT 1,
  created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  last_used_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_assets_dedup_org  ON assets (sha256_hash, owner_org_id)
  WHERE owner_org_id IS NOT NULL;
CREATE UNIQUE INDEX idx_assets_dedup_user ON assets (sha256_hash, owner_user_id)
  WHERE owner_org_id IS NULL AND owner_user_id IS NOT NULL;
CREATE INDEX idx_assets_owner_org  ON assets (owner_org_id, asset_type);
CREATE INDEX idx_assets_owner_user ON assets (owner_user_id, asset_type)
  WHERE owner_org_id IS NULL;

ALTER TABLE assets ENABLE ROW LEVEL SECURITY;

CREATE POLICY assets_select ON assets
  FOR SELECT USING (
    owner_user_id = auth.uid()
    OR owner_org_id IN (SELECT org_id FROM org_members WHERE user_id = auth.uid())
  );

CREATE POLICY assets_insert ON assets
  FOR INSERT WITH CHECK (
    owner_user_id = auth.uid()
    OR owner_org_id IN (
      SELECT org_id FROM org_members
      WHERE user_id = auth.uid() AND role IN ('OWNER','ADMIN','MEMBER')
    )
  );
```

---

### 7.2 document_assets

```sql
CREATE TABLE document_assets (
  document_id  UUID  NOT NULL REFERENCES documents (id) ON DELETE CASCADE,
  asset_id     UUID  NOT NULL REFERENCES assets (id) ON DELETE RESTRICT,
  usage_type   TEXT  NOT NULL
               CHECK (usage_type IN ('FONT','IMAGE','VECTOR','ATTACHMENT')),
  element_eid  TEXT,
  PRIMARY KEY (document_id, asset_id, usage_type)
);

CREATE INDEX idx_doc_assets_asset ON document_assets (asset_id);

ALTER TABLE document_assets ENABLE ROW LEVEL SECURITY;

CREATE POLICY doc_assets_select ON document_assets
  FOR SELECT USING (
    document_id IN (
      SELECT id FROM documents
      WHERE owner_user_id = auth.uid()
         OR owner_org_id IN (SELECT org_id FROM org_members WHERE user_id = auth.uid())
    )
  );
```

---

## Section 08 — Billing

### 8.1 subscriptions

```sql
CREATE TABLE subscriptions (
  id                      UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id                 UUID        REFERENCES users (id) ON DELETE CASCADE,
  org_id                  UUID        REFERENCES organizations (id) ON DELETE CASCADE,
  stripe_subscription_id  TEXT        UNIQUE,
  stripe_customer_id      TEXT        NOT NULL,
  stripe_price_id         TEXT,
  plan                    TEXT        NOT NULL
                                      CHECK (plan IN ('FREE','PRO','TEAM','ENTERPRISE')),
  status                  TEXT        NOT NULL
                                      CHECK (status IN ('ACTIVE','TRIALING','PAST_DUE','CANCELLED')),
  current_period_start    TIMESTAMPTZ,
  current_period_end      TIMESTAMPTZ,
  trial_end               TIMESTAMPTZ,
  cancel_at               TIMESTAMPTZ,
  cancelled_at            TIMESTAMPTZ,
  created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE subscriptions ADD CONSTRAINT chk_subs_one_owner
  CHECK (
    (user_id IS NOT NULL AND org_id IS NULL)
    OR (user_id IS NULL AND org_id IS NOT NULL)
  );

CREATE UNIQUE INDEX idx_subs_user   ON subscriptions (user_id)  WHERE user_id IS NOT NULL;
CREATE UNIQUE INDEX idx_subs_org    ON subscriptions (org_id)   WHERE org_id  IS NOT NULL;
CREATE INDEX        idx_subs_stripe ON subscriptions (stripe_subscription_id);

ALTER TABLE subscriptions ENABLE ROW LEVEL SECURITY;

CREATE POLICY subs_select ON subscriptions
  FOR SELECT USING (
    user_id = auth.uid()
    OR org_id IN (
      SELECT org_id FROM org_members
      WHERE user_id = auth.uid() AND role IN ('OWNER','ADMIN')
    )
  );

CREATE TRIGGER trg_subs_updated_at
  BEFORE UPDATE ON subscriptions
  FOR EACH ROW EXECUTE FUNCTION update_updated_at();
```

---

### 8.2 usage_events

Append-only. Never updated — only inserted.

```sql
CREATE TABLE usage_events (
  id            UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id       UUID        REFERENCES users (id) ON DELETE SET NULL,
  org_id        UUID        REFERENCES organizations (id) ON DELETE SET NULL,
  event_type    TEXT        NOT NULL
                            CHECK (event_type IN (
                              'CONVERSION','GENERATION','EXTRACTION',
                              'API_CALL','SIGNING','AI_QUERY'
                            )),
  document_id   UUID        REFERENCES documents (id) ON DELETE SET NULL,
  job_id        UUID        REFERENCES conversion_jobs (id) ON DELETE SET NULL,
  api_endpoint  TEXT,
  units         INTEGER     NOT NULL DEFAULT 1,
  billable      BOOLEAN     NOT NULL DEFAULT TRUE,
  billed        BOOLEAN     NOT NULL DEFAULT FALSE,
  unit_cost_usd NUMERIC(10,6),
  model_used    TEXT,
  occurred_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_usage_user_type_day ON usage_events (user_id, event_type, occurred_at DESC)
  WHERE billable = TRUE;
CREATE INDEX idx_usage_unbilled      ON usage_events (user_id, occurred_at)
  WHERE billed = FALSE AND billable = TRUE;
CREATE INDEX idx_usage_org_day       ON usage_events (org_id, event_type, occurred_at DESC)
  WHERE org_id IS NOT NULL;

ALTER TABLE usage_events ENABLE ROW LEVEL SECURITY;

CREATE POLICY usage_select ON usage_events
  FOR SELECT USING (
    user_id = auth.uid()
    OR org_id IN (
      SELECT org_id FROM org_members
      WHERE user_id = auth.uid() AND role IN ('OWNER','ADMIN')
    )
  );
```

---

## Section 09 — API Keys

### 9.1 api_keys

```sql
CREATE TABLE api_keys (
  id            UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id       UUID        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
  key_hash      TEXT        NOT NULL,
  key_prefix    TEXT        NOT NULL,
  key_type      TEXT        NOT NULL DEFAULT 'LIVE'
                            CHECK (key_type IN ('LIVE','TEST')),
  is_active     BOOLEAN     NOT NULL DEFAULT TRUE,
  revoked_at    TIMESTAMPTZ,
  revoked_reason TEXT,
  last_used_at  TIMESTAMPTZ,
  last_used_ip  INET,
  use_count     BIGINT      NOT NULL DEFAULT 0,
  created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_api_keys_active_user ON api_keys (user_id)
  WHERE is_active = TRUE;
CREATE INDEX idx_api_keys_prefix ON api_keys (key_prefix);

ALTER TABLE api_keys ENABLE ROW LEVEL SECURITY;

CREATE POLICY api_keys_select ON api_keys
  FOR SELECT USING (user_id = auth.uid());
```

---

## Section 10 — System Audit Log

### 10.1 audit_log

System-level audit trail. Distinct from per-document `audit.json` embedded in each SPDF file.

```sql
CREATE TABLE audit_log (
  id               UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
  actor_user_id    UUID        REFERENCES users (id) ON DELETE SET NULL,
  actor_ip         INET,
  actor_user_agent TEXT,
  action           TEXT        NOT NULL
                               CHECK (action IN (
                                 'USER_CREATED','USER_DELETED',
                                 'API_KEY_CREATED','API_KEY_ROTATED','API_KEY_REVOKED',
                                 'DOCUMENT_DELETED','DOCUMENT_SIGNED','DOCUMENT_CERTIFIED',
                                 'DOCUMENT_REDACTED',
                                 'ORG_CREATED','ORG_MEMBER_ADDED','ORG_MEMBER_REMOVED',
                                 'SUBSCRIPTION_CHANGED','SUBSCRIPTION_CANCELLED',
                                 'LOGIN_SUCCESS','LOGIN_FAILED'
                               )),
  target_type      TEXT,
  target_id        TEXT,
  request_id       TEXT,
  metadata         JSONB       NOT NULL DEFAULT '{}',
  occurred_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_actor  ON audit_log (actor_user_id, occurred_at DESC);
CREATE INDEX idx_audit_target ON audit_log (target_type, target_id, occurred_at DESC);
CREATE INDEX idx_audit_recent ON audit_log (occurred_at DESC);

ALTER TABLE audit_log ENABLE ROW LEVEL SECURITY;

CREATE POLICY audit_select_own ON audit_log
  FOR SELECT USING (actor_user_id = auth.uid());

CREATE POLICY audit_select_org_admin ON audit_log
  FOR SELECT USING (
    target_type = 'ORG' AND target_id IN (
      SELECT org_id::TEXT FROM org_members
      WHERE user_id = auth.uid() AND role IN ('OWNER','ADMIN')
    )
  );
```

---

## Section 11 — Alembic Migration Strategy

### 11.1 Migration File Order

```
migrations/versions/
  001_initial_extensions.py
  002_users_and_orgs.py
  003_documents.py
  004_conversion_jobs.py
  005_templates.py
  006_assets.py
  007_billing.py
  008_api_keys.py
  009_audit_log.py
  010_rls_policies.py
  011_indexes.py
  012_triggers.py
```

### 11.2 Migration Discipline Rules

- One concern per migration file — never combine table creation with index creation
- All migrations are transactional — failure rolls back fully
- RLS policies in a dedicated file (`010_rls_policies.py`) — can be updated independently
- Never use `op.execute()` for DML in migrations — schema changes only
- Downgrade functions required on all migrations

### 11.3 Adding RLS (Alembic pattern)

```python
def upgrade():
    op.execute("ALTER TABLE documents ENABLE ROW LEVEL SECURITY")
    op.execute("ALTER TABLE documents FORCE ROW LEVEL SECURITY")
    op.execute("DROP POLICY IF EXISTS docs_select ON documents")
    op.execute("""
        CREATE POLICY docs_select ON documents
          FOR SELECT USING (
            deleted_at IS NULL AND (
              owner_user_id = auth.uid()
              OR owner_org_id IN (
                SELECT org_id FROM org_members WHERE user_id = auth.uid()
              )
            )
          )
    """)

def downgrade():
    op.execute("DROP POLICY IF EXISTS docs_select ON documents")
    op.execute("ALTER TABLE documents DISABLE ROW LEVEL SECURITY")
```

---

## Section 12 — SQLAlchemy ORM Models

### 12.1 Base and Mixins

```python
# services/api/models/base.py
from sqlalchemy.orm import DeclarativeBase, mapped_column, Mapped
from sqlalchemy import DateTime, func
from datetime import datetime

class Base(DeclarativeBase):
    pass

class TimestampMixin:
    created_at: Mapped[datetime] = mapped_column(
        DateTime(timezone=True), server_default=func.now(), nullable=False
    )
    updated_at: Mapped[datetime] = mapped_column(
        DateTime(timezone=True), server_default=func.now(),
        onupdate=func.now(), nullable=False
    )

class SoftDeleteMixin:
    deleted_at: Mapped[datetime | None] = mapped_column(
        DateTime(timezone=True), nullable=True, default=None
    )

    @property
    def is_deleted(self) -> bool:
        return self.deleted_at is not None
```

### 12.2 Document Model

```python
# services/api/models/document.py
from sqlalchemy import String, Text, BigInteger, SmallInteger, Boolean, Numeric, ARRAY, ForeignKey
from sqlalchemy.dialects.postgresql import UUID, JSONB
from sqlalchemy.orm import Mapped, mapped_column, relationship
from .base import Base, TimestampMixin, SoftDeleteMixin
import uuid

class Document(Base, TimestampMixin, SoftDeleteMixin):
    __tablename__ = "documents"

    id:               Mapped[uuid.UUID]        = mapped_column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    spdf_doc_id:      Mapped[str]              = mapped_column(Text, unique=True, nullable=False)
    title:            Mapped[str]              = mapped_column(Text, nullable=False)
    owner_user_id:    Mapped[uuid.UUID | None]  = mapped_column(UUID(as_uuid=True), ForeignKey("users.id"), nullable=True)
    owner_org_id:     Mapped[uuid.UUID | None]  = mapped_column(UUID(as_uuid=True), ForeignKey("organizations.id"), nullable=True)
    document_type:    Mapped[str]              = mapped_column(Text, nullable=False, default="General")
    locale:           Mapped[str]              = mapped_column(String(20), nullable=False, default="en-US")
    spdf_version:     Mapped[str]              = mapped_column(String(10), nullable=False, default="1.0")
    state:            Mapped[str]              = mapped_column(Text, nullable=False, default="DRAFT")
    r2_key:           Mapped[str]              = mapped_column(Text, nullable=False)
    r2_bucket:        Mapped[str]              = mapped_column(Text, nullable=False, default="spdf-documents")
    file_size_bytes:  Mapped[int | None]       = mapped_column(BigInteger, nullable=True)
    page_count:       Mapped[int | None]       = mapped_column(SmallInteger, nullable=True)
    has_render_layer: Mapped[bool]             = mapped_column(Boolean, nullable=False, default=True)
    source_format:    Mapped[str]              = mapped_column(Text, nullable=False, default="NATIVE")
    template_id:      Mapped[uuid.UUID | None]  = mapped_column(UUID(as_uuid=True), ForeignKey("templates.id"), nullable=True)
    conversion_job_id:Mapped[uuid.UUID | None]  = mapped_column(UUID(as_uuid=True), ForeignKey("conversion_jobs.id"), nullable=True)
    confidence_score: Mapped[float | None]     = mapped_column(Numeric(4, 3), nullable=True)
    model_used:       Mapped[str | None]       = mapped_column(Text, nullable=True)
    meta_author:      Mapped[str | None]       = mapped_column(Text, nullable=True)
    meta_currency:    Mapped[str | None]       = mapped_column(String(3), nullable=True)
    meta_tags:        Mapped[list[str] | None]  = mapped_column(ARRAY(Text), nullable=True)
    metadata:         Mapped[dict]             = mapped_column(JSONB, nullable=False, default=dict)
```

---

## Section 13 — Key Queries

### 13.1 Rate Limit Check (every API call)

```sql
SELECT COUNT(*) FROM usage_events
WHERE user_id = $1
  AND event_type = 'CONVERSION'
  AND occurred_at >= date_trunc('day', NOW() AT TIME ZONE 'UTC')
  AND billable = TRUE;
-- Index: idx_usage_user_type_day
```

### 13.2 Document List (dashboard)

```sql
SELECT id, spdf_doc_id, title, document_type, state,
       page_count, file_size_bytes, confidence_score, created_at, updated_at
FROM documents
WHERE owner_user_id = $1
  AND deleted_at IS NULL
  AND ($2::text IS NULL OR state = $2)
  AND ($3::text IS NULL OR document_type = $3)
  AND ($4::text IS NULL OR title ILIKE '%' || $4 || '%')
ORDER BY created_at DESC
LIMIT $5;
-- Index: idx_docs_owner_user, idx_docs_title_trgm
```

### 13.3 Job Dequeue (Celery worker — atomic)

```sql
UPDATE conversion_jobs
SET status = 'PROCESSING',
    started_at = NOW(),
    attempt_count = attempt_count + 1
WHERE id = (
  SELECT id FROM conversion_jobs
  WHERE status = 'QUEUED' AND expires_at > NOW()
  ORDER BY queued_at ASC
  LIMIT 1
  FOR UPDATE SKIP LOCKED
)
RETURNING *;
-- Index: idx_jobs_dequeue
-- SKIP LOCKED is critical — prevents two workers claiming the same job
```

### 13.4 Asset Deduplication Lookup

```sql
SELECT id, asset_id, r2_key FROM assets
WHERE sha256_hash = $1 AND owner_org_id = $2;
-- Index: idx_assets_dedup_org
```

---

## Section 14 — Maintenance Jobs

| Job | Schedule | Purpose |
|---|---|---|
| `cleanup_expired_jobs` | Every hour | Remove COMPLETED/FAILED/CANCELLED job records older than 7 days |
| `cleanup_orphaned_uploads` | Every 6 hours | Remove R2 uploads where job failed and no document was created |
| `flush_usage_to_stripe` | Hourly | Report metered usage to Stripe for API billing |
| `update_template_use_count` | Daily | Refresh template usage stats |

---

## Section 15 — Entity Relationship Summary

```
users ──────────────────────── org_members ──── organizations
  │                                                    │
  ├── documents ──────────────────────────────── (owner_org_id)
  │     │                                              │
  │     ├── conversion_jobs                      templates
  │     ├── document_assets ── assets (org-scoped dedup)
  │     └── (template_id) ─── templates
  │
  ├── subscriptions
  ├── usage_events
  ├── api_keys
  └── audit_log
```

---

## Appendix A — Index Summary

| Index | Table | Type | Purpose |
|---|---|---|---|
| `idx_users_clerk_id` | users | B-tree | Clerk webhook lookup |
| `idx_docs_owner_user` | documents | B-tree | Dashboard list |
| `idx_docs_title_trgm` | documents | GIN (trigram) | Full-text search |
| `idx_docs_spdf_id` | documents | Unique B-tree | Spec ID lookup |
| `idx_jobs_user_status` | conversion_jobs | B-tree | Job polling |
| `idx_jobs_dequeue` | conversion_jobs | B-tree (partial) | Atomic worker dequeue |
| `idx_assets_dedup_org` | assets | Unique B-tree | Org-scoped dedup |
| `idx_usage_user_type_day` | usage_events | B-tree | Rate limiting |
| `idx_usage_unbilled` | usage_events | B-tree (partial) | Billing flush |

---

## Appendix B — Change Log

| Version | Date | Author | Changes |
|---|---|---|---|
| 1.0 | March 2025 | Founder + Claude AI | Initial release — all sections, 12 tables, full RLS |

---

*— End of SPDF Database Schema Design v1.0 —*  
*SPDF Platform · Internal Engineering Document · Confidential*
