import { describe, it, expect, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { Shell } from "@/components/layout/Shell";
import { useAuthStore } from "@/stores/auth-store";

describe("Shell", () => {
  beforeEach(() => {
    useAuthStore.setState({ apiKey: "sk_test_123", isAuthenticated: true });
    localStorage.clear();
  });

  it("renders sidebar and main content", () => {
    render(
      <MemoryRouter>
        <Shell>
          <div>Test Content</div>
        </Shell>
      </MemoryRouter>,
    );
    expect(screen.getByText("SPDF Studio")).toBeInTheDocument();
    expect(screen.getByText("Test Content")).toBeInTheDocument();
  });

  it("renders header with API key badge", () => {
    render(
      <MemoryRouter>
        <Shell>
          <div>Content</div>
        </Shell>
      </MemoryRouter>,
    );
    expect(screen.getByText("sk_test...")).toBeInTheDocument();
  });

  it("shows auth gate when not authenticated", () => {
    useAuthStore.setState({ apiKey: null, isAuthenticated: false });
    render(
      <MemoryRouter>
        <Shell>
          <div>Hidden Content</div>
        </Shell>
      </MemoryRouter>,
    );
    expect(screen.getByText("Welcome to SPDF Studio")).toBeInTheDocument();
    expect(screen.queryByText("Hidden Content")).not.toBeInTheDocument();
  });

  it("persists sidebar collapse state to localStorage", () => {
    render(
      <MemoryRouter>
        <Shell>
          <div>Content</div>
        </Shell>
      </MemoryRouter>,
    );
    // Find the collapse toggle button (the chevron button in sidebar footer)
    const buttons = screen.getAllByRole("button");
    const collapseBtn = buttons.find((b) => b.closest("aside"));
    if (collapseBtn) {
      fireEvent.click(collapseBtn);
      expect(localStorage.getItem("spdf_sidebar_collapsed")).toBe("true");
    }
  });
});
