import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { Generate } from "@/pages/Generate";
import { useAuthStore } from "@/stores/auth-store";

describe("Generate page", () => {
  beforeEach(() => {
    useAuthStore.setState({ apiKey: "sk_test_123", isAuthenticated: true });
    vi.mocked(global.fetch).mockReset();
  });

  it("renders the generate form", () => {
    render(
      <MemoryRouter>
        <Generate />
      </MemoryRouter>,
    );
    expect(screen.getByText("Generate Document")).toBeInTheDocument();
    expect(screen.getByText("Document Name")).toBeInTheDocument();
    expect(screen.getByText("Semantic JSON")).toBeInTheDocument();
  });

  it("has a Load Sample button", () => {
    render(
      <MemoryRouter>
        <Generate />
      </MemoryRouter>,
    );
    expect(screen.getByText("Load Sample")).toBeInTheDocument();
  });

  it("shows JSON error for invalid input", () => {
    render(
      <MemoryRouter>
        <Generate />
      </MemoryRouter>,
    );
    // Clear the textarea and type invalid JSON
    const textarea = screen.getByPlaceholderText("Paste semantic JSON here...");
    fireEvent.change(textarea, { target: { value: "{invalid json" } });
    fireEvent.click(screen.getByText("Generate SPDF"));
    // Should show a JSON parse error
    const errorEl = document.querySelector(".text-destructive");
    expect(errorEl).toBeInTheDocument();
  });

  it("populates sample when Load Sample is clicked", () => {
    render(
      <MemoryRouter>
        <Generate />
      </MemoryRouter>,
    );
    fireEvent.click(screen.getByText("Load Sample"));
    const textarea = screen.getByPlaceholderText("Paste semantic JSON here...") as HTMLTextAreaElement;
    expect(textarea.value).toContain("Sample Invoice");
  });
});
