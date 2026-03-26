import { API_BASE } from "./constants";
import { useAuthStore } from "@/stores/auth-store";

export class ApiError extends Error {
  constructor(
    public status: number,
    public detail: string,
    public requestId?: string,
  ) {
    super(detail);
    this.name = "ApiError";
  }
}

async function request<T>(
  path: string,
  options: RequestInit = {},
): Promise<T> {
  const apiKey = useAuthStore.getState().apiKey;
  const headers: Record<string, string> = {
    ...(options.headers as Record<string, string>),
  };

  if (apiKey) {
    headers["Authorization"] = `Bearer ${apiKey}`;
  }

  const res = await fetch(`${API_BASE}${path}`, {
    ...options,
    headers,
  });

  if (!res.ok) {
    let detail = `HTTP ${res.status}`;
    let requestId: string | undefined;
    try {
      const body = await res.json();
      detail = body.detail || body.message || detail;
      requestId = body.request_id;
    } catch {
      // response body not JSON
    }
    throw new ApiError(res.status, detail, requestId);
  }

  const contentType = res.headers.get("content-type");
  if (contentType?.includes("application/json")) {
    return res.json();
  }
  return res.blob() as unknown as T;
}

function postFile(path: string, file: File | Blob, data?: Record<string, string>) {
  const form = new FormData();
  const filename = file instanceof File ? file.name : "document.spdf";
  form.append("file", file, filename);
  if (data) {
    for (const [key, value] of Object.entries(data)) {
      form.append(key, value);
    }
  }
  return request<Blob>(path, { method: "POST", body: form });
}

function postTwoFiles(path: string, fileA: File | Blob, fileB: File | Blob) {
  const form = new FormData();
  const nameA = fileA instanceof File ? fileA.name : "doc_a.spdf";
  const nameB = fileB instanceof File ? fileB.name : "doc_b.spdf";
  form.append("file_a", fileA, nameA);
  form.append("file_b", fileB, nameB);
  return request(path, { method: "POST", body: form });
}

// --- Health ---

export function getHealth() {
  return request<{ status: string; engine_version: string }>("/health");
}

// --- Documents ---

export interface GeneratePayload {
  semantic: Record<string, unknown>;
  layout?: Record<string, unknown>;
  styles?: Record<string, unknown>;
  metadata?: Record<string, unknown>;
  audit?: Record<string, unknown>;
}

export function generateDocument(payload: GeneratePayload) {
  return request<Blob>("/documents/generate", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });
}

export interface ValidationReport {
  valid: boolean;
  errors: Array<{ code: string; message: string; severity: string }>;
  error_count: number;
  fatal_count: number;
}

export function validateDocument(file: File | Blob) {
  return postFile("/documents/validate", file) as unknown as Promise<ValidationReport>;
}

export function renderDocument(file: File | Blob) {
  return postFile("/documents/render", file);
}

export interface ParseResult {
  document: Record<string, unknown>;
  validation: ValidationReport;
}

export function parseDocument(semantic: string) {
  return request<ParseResult>("/documents/parse", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ semantic }),
  });
}

export interface InvoiceData {
  has_invoice: boolean;
  invoice_number: string | null;
  total: string | null;
  currency: string | null;
}

export function extractInvoice(file: File | Blob) {
  return postFile("/documents/extract", file) as unknown as Promise<InvoiceData>;
}

export function signDocument(file: File | Blob, signerName: string, signerEmail: string) {
  return postFile("/documents/sign", file, {
    signer_name: signerName,
    signer_email: signerEmail,
  });
}

export interface VerificationReport {
  valid: boolean;
  tamper_detected: boolean;
  signature_count: number;
  signatures: Array<{
    valid: boolean;
    signer_name: string;
    signer_email: string;
    signed_at: string;
  }>;
}

export function verifyDocument(file: File | Blob) {
  const form = new FormData();
  form.append("file", file, "doc.spdf");
  return request<VerificationReport>("/documents/verify", {
    method: "POST",
    body: form,
  });
}

export function transitionDocument(file: File | Blob, targetState: string) {
  return postFile("/documents/transition", file, { target_state: targetState });
}

export interface DiffReport {
  identical: boolean;
  changes: Array<{
    change_type: string;
    element_type: string;
    eid: string;
    field: string | null;
    old_value: unknown;
    new_value: unknown;
    impact: string;
  }>;
  metadata_changes: Array<{
    field: string | null;
    old_value: unknown;
    new_value: unknown;
  }>;
}

export function diffDocuments(fileA: File | Blob, fileB: File | Blob) {
  return postTwoFiles("/documents/diff", fileA, fileB) as Promise<DiffReport>;
}

export function redactElement(file: File | Blob, targetEid: string, reason: string) {
  return postFile("/documents/redact", file, {
    target_eid: targetEid,
    reason,
  });
}

export interface RedactionEntry {
  redacted_eid: string;
  reason: string;
  proof_hash: string;
}

export function listRedactions(file: File | Blob) {
  const form = new FormData();
  form.append("file", file, "doc.spdf");
  return request<{ redactions: RedactionEntry[] }>("/documents/redactions", {
    method: "POST",
    body: form,
  });
}

export function verifyRedaction(file: File | Blob, redactionEid: string) {
  const form = new FormData();
  form.append("file", file, "doc.spdf");
  form.append("redaction_eid", redactionEid);
  return request<{ found: boolean; proof_hash: string }>("/documents/verify-redaction", {
    method: "POST",
    body: form,
  });
}

// --- Account ---

export interface ApiKeyInfo {
  key_prefix: string;
  created_at: string;
  user_id: string;
}

export function getApiKeyInfo() {
  return request<ApiKeyInfo>("/account/api-key");
}

export function rotateApiKey() {
  return request<{ api_key: string; key_prefix: string }>("/account/api-key/rotate", {
    method: "POST",
  });
}

export interface UsageInfo {
  date: string;
  usage: Record<string, { used: number; limit: number }>;
  tier: string;
}

export function getUsage() {
  return request<UsageInfo>("/account/usage");
}

// --- Billing ---

export interface SubscriptionInfo {
  tier: string;
  status: string;
  current_period_end: string | null;
}

export function getSubscription() {
  return request<SubscriptionInfo>("/billing/subscription");
}

// --- Templates ---

export interface Template {
  id: string;
  name: string;
  description: string;
  semantic_template: Record<string, unknown> | null;
  created_at: string;
  updated_at: string;
}

export interface TemplateListResponse {
  templates: Template[];
  next_cursor: string | null;
}

export function listTemplates(cursor?: string, limit = 20) {
  const params = new URLSearchParams({ limit: String(limit) });
  if (cursor) params.set("cursor", cursor);
  return request<TemplateListResponse>(`/templates?${params}`);
}

export function getTemplate(id: string) {
  return request<Template>(`/templates/${id}`);
}

export function createTemplate(name: string, description = "", semanticTemplate?: Record<string, unknown>) {
  return request<Template>("/templates", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      name,
      description,
      semantic_template: semanticTemplate ?? null,
    }),
  });
}

export function updateTemplate(id: string, updates: Partial<Pick<Template, "name" | "description" | "semantic_template">>) {
  return request<Template>(`/templates/${id}`, {
    method: "PATCH",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(updates),
  });
}

export function deleteTemplate(id: string) {
  return request<void>(`/templates/${id}`, { method: "DELETE" });
}
