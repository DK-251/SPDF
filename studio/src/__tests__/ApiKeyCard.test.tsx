import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { Settings } from "@/pages/Settings";
import { useAuthStore } from "@/stores/auth-store";

function mockFetchResponses() {
  let callCount = 0;
  vi.mocked(global.fetch).mockImplementation(async (url) => {
    const urlStr = typeof url === "string" ? url : url.toString();
    if (urlStr.includes("/account/api-key") && !urlStr.includes("rotate")) {
      return {
        ok: true,
        status: 200,
        headers: new Headers({ "content-type": "application/json" }),
        json: () => Promise.resolve({ key_prefix: "sk_live", created_at: "2026-01-01", user_id: "u1" }),
        blob: () => Promise.resolve(new Blob()),
      } as Response;
    }
    if (urlStr.includes("/account/usage")) {
      return {
        ok: true,
        status: 200,
        headers: new Headers({ "content-type": "application/json" }),
        json: () => Promise.resolve({ date: "2026-03-26", usage: { documents: { used: 5, limit: 100 } }, tier: "FREE" }),
        blob: () => Promise.resolve(new Blob()),
      } as Response;
    }
    if (urlStr.includes("/billing/subscription")) {
      return {
        ok: true,
        status: 200,
        headers: new Headers({ "content-type": "application/json" }),
        json: () => Promise.resolve({ tier: "FREE", status: "active", current_period_end: null }),
        blob: () => Promise.resolve(new Blob()),
      } as Response;
    }
    return {
      ok: true,
      status: 200,
      headers: new Headers({ "content-type": "application/json" }),
      json: () => Promise.resolve({}),
      blob: () => Promise.resolve(new Blob()),
    } as Response;
  });
}

describe("Settings page", () => {
  beforeEach(() => {
    useAuthStore.setState({ apiKey: "sk_test_123", isAuthenticated: true });
    vi.mocked(global.fetch).mockReset();
    mockFetchResponses();
  });

  it("renders settings sections", async () => {
    render(
      <MemoryRouter>
        <Settings />
      </MemoryRouter>,
    );
    expect(screen.getByText("API Key")).toBeInTheDocument();
    expect(screen.getByText("Today's Usage")).toBeInTheDocument();
    expect(screen.getByText("Billing")).toBeInTheDocument();
  });

  it("shows API key prefix after loading", async () => {
    render(
      <MemoryRouter>
        <Settings />
      </MemoryRouter>,
    );
    await waitFor(() => {
      expect(screen.getByText("sk_live...")).toBeInTheDocument();
    });
  });

  it("shows usage tier after loading", async () => {
    render(
      <MemoryRouter>
        <Settings />
      </MemoryRouter>,
    );
    await waitFor(() => {
      expect(screen.getByText("FREE")).toBeInTheDocument();
    });
  });

  it("has a Rotate Key button", () => {
    render(
      <MemoryRouter>
        <Settings />
      </MemoryRouter>,
    );
    expect(screen.getByText("Rotate Key")).toBeInTheDocument();
  });

  it("has a Disconnect button", () => {
    render(
      <MemoryRouter>
        <Settings />
      </MemoryRouter>,
    );
    expect(screen.getByText("Disconnect")).toBeInTheDocument();
  });

  it("clears auth state on disconnect", () => {
    render(
      <MemoryRouter>
        <Settings />
      </MemoryRouter>,
    );
    fireEvent.click(screen.getByText("Disconnect"));
    expect(useAuthStore.getState().isAuthenticated).toBe(false);
  });
});
