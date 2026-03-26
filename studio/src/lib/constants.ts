export const API_BASE = "/api/v1";

export const ROUTES = {
  DASHBOARD: "/",
  GENERATE: "/generate",
  DOCUMENT: "/documents/:id",
  TEMPLATES: "/templates",
  SETTINGS: "/settings",
} as const;

export const DOCUMENT_STATES = {
  DRAFT: "Draft",
  REVIEW: "Review",
  SIGNED: "Signed",
  ARCHIVED: "Archived",
  REVOKED: "Revoked",
} as const;

export const STATE_COLORS: Record<string, string> = {
  Draft: "bg-zinc-500/20 text-zinc-400",
  Review: "bg-amber-500/20 text-amber-400",
  Signed: "bg-emerald-500/20 text-emerald-400",
  Archived: "bg-blue-500/20 text-blue-400",
  Revoked: "bg-rose-500/20 text-rose-400",
};

export const LOCAL_STORAGE_KEYS = {
  API_KEY: "spdf_api_key",
  SIDEBAR_COLLAPSED: "spdf_sidebar_collapsed",
} as const;
