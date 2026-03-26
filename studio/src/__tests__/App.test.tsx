import { describe, it, expect, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { App } from "@/App";
import { useAuthStore } from "@/stores/auth-store";

function renderApp(route = "/") {
  return render(
    <MemoryRouter initialEntries={[route]}>
      <App />
    </MemoryRouter>,
  );
}

describe("App", () => {
  beforeEach(() => {
    useAuthStore.setState({ apiKey: "sk_test_123", isAuthenticated: true });
  });

  it("renders the sidebar with navigation links", () => {
    renderApp();
    expect(screen.getByText("SPDF Studio")).toBeInTheDocument();
    expect(screen.getByText("Documents")).toBeInTheDocument();
    expect(screen.getByText("Generate")).toBeInTheDocument();
    expect(screen.getByText("Templates")).toBeInTheDocument();
    expect(screen.getByText("Settings")).toBeInTheDocument();
  });

  it("renders the dashboard at root route", () => {
    renderApp("/");
    expect(screen.getByText("Upload, generate, and manage SPDF documents")).toBeInTheDocument();
  });

  it("renders NotFound for unknown routes", () => {
    renderApp("/unknown-page");
    expect(screen.getByText("Page Not Found")).toBeInTheDocument();
  });

  it("shows auth gate when not authenticated", () => {
    useAuthStore.setState({ apiKey: null, isAuthenticated: false });
    renderApp();
    expect(screen.getByText("Welcome to SPDF Studio")).toBeInTheDocument();
  });
});
