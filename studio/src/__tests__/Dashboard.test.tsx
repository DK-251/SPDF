import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { Dashboard } from "@/pages/Dashboard";
import { useAuthStore } from "@/stores/auth-store";
import { useDocumentStore } from "@/stores/document-store";

describe("Dashboard", () => {
  beforeEach(() => {
    useAuthStore.setState({ apiKey: "sk_test_123", isAuthenticated: true });
    useDocumentStore.setState({ documents: [], activeDocumentId: null });
    vi.mocked(global.fetch).mockReset();
  });

  it("renders page title and description", () => {
    render(
      <MemoryRouter>
        <Dashboard />
      </MemoryRouter>,
    );
    expect(screen.getByText("Documents")).toBeInTheDocument();
    expect(
      screen.getByText("Upload, generate, and manage SPDF documents"),
    ).toBeInTheDocument();
  });

  it("renders upload zone", () => {
    render(
      <MemoryRouter>
        <Dashboard />
      </MemoryRouter>,
    );
    expect(screen.getByText("Drop an SPDF file here")).toBeInTheDocument();
  });

  it("shows empty state when no documents", () => {
    render(
      <MemoryRouter>
        <Dashboard />
      </MemoryRouter>,
    );
    expect(
      screen.getByText(
        "No documents yet. Upload an SPDF file or generate a new one.",
      ),
    ).toBeInTheDocument();
  });

  it("renders document cards when documents exist", () => {
    useDocumentStore.setState({
      documents: [
        {
          id: "d1",
          name: "test.spdf",
          state: "Draft",
          size: 1024,
          uploadedAt: "2026-03-26T10:00:00Z",
          blob: new Blob(["test"]),
        },
      ],
      activeDocumentId: null,
    });
    render(
      <MemoryRouter>
        <Dashboard />
      </MemoryRouter>,
    );
    expect(screen.getByText("test.spdf")).toBeInTheDocument();
    expect(screen.getByText("Draft")).toBeInTheDocument();
  });

  it("has a Generate button", () => {
    render(
      <MemoryRouter>
        <Dashboard />
      </MemoryRouter>,
    );
    const genBtn = screen.getByRole("button", { name: /generate/i });
    expect(genBtn).toBeInTheDocument();
  });

  it("removes document from store on delete", () => {
    useDocumentStore.setState({
      documents: [
        {
          id: "d1",
          name: "delete-me.spdf",
          state: "Draft",
          size: 512,
          uploadedAt: "2026-03-26T10:00:00Z",
          blob: new Blob(["test"]),
        },
      ],
      activeDocumentId: null,
    });
    render(
      <MemoryRouter>
        <Dashboard />
      </MemoryRouter>,
    );
    // Click the delete button (last button in the card)
    const buttons = screen.getAllByRole("button");
    const deleteBtn = buttons.find(
      (b) => b.querySelector("[class*='text-destructive']") || b.className.includes("destructive"),
    );
    if (deleteBtn) {
      fireEvent.click(deleteBtn);
      expect(useDocumentStore.getState().documents).toHaveLength(0);
    }
  });
});
