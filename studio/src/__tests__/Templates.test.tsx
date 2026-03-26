import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { Templates } from "@/pages/Templates";
import { useAuthStore } from "@/stores/auth-store";

describe("Templates page", () => {
  beforeEach(() => {
    useAuthStore.setState({ apiKey: "sk_test_123", isAuthenticated: true });
    vi.mocked(global.fetch).mockReset();
  });

  it("renders templates page title", () => {
    vi.mocked(global.fetch).mockResolvedValueOnce({
      ok: true,
      status: 200,
      headers: new Headers({ "content-type": "application/json" }),
      json: () => Promise.resolve({ templates: [], next_cursor: null }),
      blob: () => Promise.resolve(new Blob()),
    } as Response);

    render(
      <MemoryRouter>
        <Templates />
      </MemoryRouter>,
    );
    expect(screen.getByText("Templates")).toBeInTheDocument();
    expect(
      screen.getByText("Reusable document templates for quick generation"),
    ).toBeInTheDocument();
  });

  it("has a New Template button", () => {
    vi.mocked(global.fetch).mockResolvedValueOnce({
      ok: true,
      status: 200,
      headers: new Headers({ "content-type": "application/json" }),
      json: () => Promise.resolve({ templates: [], next_cursor: null }),
      blob: () => Promise.resolve(new Blob()),
    } as Response);

    render(
      <MemoryRouter>
        <Templates />
      </MemoryRouter>,
    );
    expect(screen.getByText("New Template")).toBeInTheDocument();
  });

  it("shows empty state when no templates", async () => {
    vi.mocked(global.fetch).mockResolvedValueOnce({
      ok: true,
      status: 200,
      headers: new Headers({ "content-type": "application/json" }),
      json: () => Promise.resolve({ templates: [], next_cursor: null }),
      blob: () => Promise.resolve(new Blob()),
    } as Response);

    render(
      <MemoryRouter>
        <Templates />
      </MemoryRouter>,
    );
    await waitFor(() => {
      expect(
        screen.getByText("No templates yet. Create one to get started."),
      ).toBeInTheDocument();
    });
  });

  it("renders templates when available", async () => {
    vi.mocked(global.fetch).mockResolvedValueOnce({
      ok: true,
      status: 200,
      headers: new Headers({ "content-type": "application/json" }),
      json: () =>
        Promise.resolve({
          templates: [
            {
              id: "t1",
              name: "Invoice Template",
              description: "Standard invoice",
              semantic_template: null,
              created_at: "2026-03-26T10:00:00Z",
              updated_at: "2026-03-26T10:00:00Z",
            },
          ],
          next_cursor: null,
        }),
      blob: () => Promise.resolve(new Blob()),
    } as Response);

    render(
      <MemoryRouter>
        <Templates />
      </MemoryRouter>,
    );
    await waitFor(() => {
      expect(screen.getByText("Invoice Template")).toBeInTheDocument();
      expect(screen.getByText("Standard invoice")).toBeInTheDocument();
    });
  });

  it("opens create dialog when New Template is clicked", async () => {
    vi.mocked(global.fetch).mockResolvedValueOnce({
      ok: true,
      status: 200,
      headers: new Headers({ "content-type": "application/json" }),
      json: () => Promise.resolve({ templates: [], next_cursor: null }),
      blob: () => Promise.resolve(new Blob()),
    } as Response);

    render(
      <MemoryRouter>
        <Templates />
      </MemoryRouter>,
    );

    await waitFor(() => {
      expect(screen.getByText("New Template")).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText("New Template"));
    expect(screen.getByText("New Template", { selector: "h2" })).toBeInTheDocument();
    expect(screen.getByPlaceholderText("Invoice Template")).toBeInTheDocument();
  });
});
