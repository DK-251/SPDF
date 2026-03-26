import { describe, it, expect, vi, beforeEach } from "vitest";
import { useAuthStore } from "@/stores/auth-store";

// Must import after setup so fetch mock is ready
import {
  getHealth,
  generateDocument,
  listTemplates,
  createTemplate,
  deleteTemplate,
  getApiKeyInfo,
  rotateApiKey,
  getUsage,
  getSubscription,
  ApiError,
} from "@/lib/api-client";

function mockFetch(body: unknown, status = 200) {
  (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
    ok: status >= 200 && status < 300,
    status,
    headers: new Headers({ "content-type": "application/json" }),
    json: () => Promise.resolve(body),
    blob: () => Promise.resolve(new Blob()),
  });
}

describe("api-client", () => {
  beforeEach(() => {
    vi.mocked(global.fetch).mockReset();
    useAuthStore.setState({ apiKey: "sk_test_abc123", isAuthenticated: true });
  });

  it("getHealth returns status and version", async () => {
    mockFetch({ status: "ok", engine_version: "0.1.0" });
    const data = await getHealth();
    expect(data.status).toBe("ok");
    expect(data.engine_version).toBe("0.1.0");
  });

  it("includes Authorization header when API key is set", async () => {
    mockFetch({ status: "ok", engine_version: "0.1.0" });
    await getHealth();
    const call = vi.mocked(global.fetch).mock.calls[0];
    const headers = call[1]?.headers as Record<string, string>;
    expect(headers.Authorization).toBe("Bearer sk_test_abc123");
  });

  it("throws ApiError on non-OK response", async () => {
    mockFetch({ detail: "Unauthorized" }, 401);
    await expect(getHealth()).rejects.toThrow(ApiError);
    try {
      mockFetch({ detail: "Forbidden" }, 403);
      await getHealth();
    } catch (e) {
      expect(e).toBeInstanceOf(ApiError);
      expect((e as ApiError).status).toBe(403);
    }
  });

  it("generateDocument sends POST with JSON body", async () => {
    (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
      ok: true,
      status: 200,
      headers: new Headers({ "content-type": "application/octet-stream" }),
      json: () => Promise.reject("not json"),
      blob: () => Promise.resolve(new Blob(["fake-spdf"])),
    });
    const blob = await generateDocument({ semantic: { test: true } });
    expect(blob).toBeInstanceOf(Blob);
    const call = vi.mocked(global.fetch).mock.calls[0];
    expect(call[0]).toContain("/documents/generate");
    expect(call[1]?.method).toBe("POST");
  });

  it("listTemplates builds query params", async () => {
    mockFetch({ templates: [], next_cursor: null });
    await listTemplates("abc", 10);
    const url = vi.mocked(global.fetch).mock.calls[0][0] as string;
    expect(url).toContain("limit=10");
    expect(url).toContain("cursor=abc");
  });

  it("createTemplate sends name and description", async () => {
    mockFetch({ id: "t1", name: "Test", description: "", created_at: "", updated_at: "", semantic_template: null });
    await createTemplate("Test", "A test template");
    const call = vi.mocked(global.fetch).mock.calls[0];
    const body = JSON.parse(call[1]?.body as string);
    expect(body.name).toBe("Test");
    expect(body.description).toBe("A test template");
  });

  it("deleteTemplate sends DELETE method", async () => {
    (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
      ok: true,
      status: 204,
      headers: new Headers(),
      json: () => Promise.resolve(undefined),
      blob: () => Promise.resolve(new Blob()),
    });
    await deleteTemplate("t1");
    const call = vi.mocked(global.fetch).mock.calls[0];
    expect(call[1]?.method).toBe("DELETE");
    expect(call[0]).toContain("/templates/t1");
  });

  it("getApiKeyInfo fetches from account endpoint", async () => {
    mockFetch({ key_prefix: "sk_live", created_at: "2026-01-01", user_id: "u1" });
    const info = await getApiKeyInfo();
    expect(info.key_prefix).toBe("sk_live");
  });

  it("rotateApiKey sends POST", async () => {
    mockFetch({ api_key: "sk_live_new123", key_prefix: "sk_live" });
    const result = await rotateApiKey();
    expect(result.api_key).toBe("sk_live_new123");
  });

  it("getUsage returns usage data", async () => {
    mockFetch({ date: "2026-03-26", usage: {}, tier: "FREE" });
    const data = await getUsage();
    expect(data.tier).toBe("FREE");
  });

  it("getSubscription returns tier info", async () => {
    mockFetch({ tier: "PRO", status: "active", current_period_end: null });
    const sub = await getSubscription();
    expect(sub.tier).toBe("PRO");
  });
});
