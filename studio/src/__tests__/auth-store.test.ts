import { describe, it, expect, beforeEach } from "vitest";
import { useAuthStore } from "@/stores/auth-store";

describe("auth-store", () => {
  beforeEach(() => {
    useAuthStore.setState({ apiKey: null, isAuthenticated: false });
    localStorage.clear();
  });

  it("starts unauthenticated when no key in localStorage", () => {
    const state = useAuthStore.getState();
    expect(state.isAuthenticated).toBe(false);
    expect(state.apiKey).toBeNull();
  });

  it("setApiKey stores key and marks authenticated", () => {
    useAuthStore.getState().setApiKey("sk_test_123");
    const state = useAuthStore.getState();
    expect(state.apiKey).toBe("sk_test_123");
    expect(state.isAuthenticated).toBe(true);
    expect(localStorage.getItem("spdf_api_key")).toBe("sk_test_123");
  });

  it("clearApiKey removes key and marks unauthenticated", () => {
    useAuthStore.getState().setApiKey("sk_test_123");
    useAuthStore.getState().clearApiKey();
    const state = useAuthStore.getState();
    expect(state.apiKey).toBeNull();
    expect(state.isAuthenticated).toBe(false);
    expect(localStorage.getItem("spdf_api_key")).toBeNull();
  });

  it("setApiKey overwrites previous key", () => {
    useAuthStore.getState().setApiKey("sk_test_old");
    useAuthStore.getState().setApiKey("sk_test_new");
    expect(useAuthStore.getState().apiKey).toBe("sk_test_new");
    expect(localStorage.getItem("spdf_api_key")).toBe("sk_test_new");
  });
});
