import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { DocumentCard } from "@/components/documents/DocumentCard";
import { type DocumentEntry } from "@/stores/document-store";

const mockDoc: DocumentEntry = {
  id: "doc_test_001",
  name: "invoice.spdf",
  state: "Draft",
  size: 2048,
  uploadedAt: "2026-03-26T10:00:00Z",
  blob: new Blob(["test"]),
};

describe("DocumentCard", () => {
  it("renders document name and state", () => {
    render(
      <MemoryRouter>
        <DocumentCard
          document={mockDoc}
          onDownload={vi.fn()}
          onDelete={vi.fn()}
        />
      </MemoryRouter>,
    );
    expect(screen.getByText("invoice.spdf")).toBeInTheDocument();
    expect(screen.getByText("Draft")).toBeInTheDocument();
  });

  it("renders file size", () => {
    render(
      <MemoryRouter>
        <DocumentCard
          document={mockDoc}
          onDownload={vi.fn()}
          onDelete={vi.fn()}
        />
      </MemoryRouter>,
    );
    expect(screen.getByText("2.0 KB")).toBeInTheDocument();
  });

  it("calls onDelete when delete button is clicked", () => {
    const onDelete = vi.fn();
    render(
      <MemoryRouter>
        <DocumentCard
          document={mockDoc}
          onDownload={vi.fn()}
          onDelete={onDelete}
        />
      </MemoryRouter>,
    );
    // Find all buttons, the delete one has the trash icon
    const buttons = screen.getAllByRole("button");
    const deleteBtn = buttons[buttons.length - 1];
    fireEvent.click(deleteBtn);
    expect(onDelete).toHaveBeenCalledWith("doc_test_001");
  });

  it("calls onDownload when download button is clicked", () => {
    const onDownload = vi.fn();
    render(
      <MemoryRouter>
        <DocumentCard
          document={mockDoc}
          onDownload={onDownload}
          onDelete={vi.fn()}
        />
      </MemoryRouter>,
    );
    const buttons = screen.getAllByRole("button");
    const downloadBtn = buttons[buttons.length - 2];
    fireEvent.click(downloadBtn);
    expect(onDownload).toHaveBeenCalledWith(mockDoc);
  });

  it("renders different state colors", () => {
    const signedDoc = { ...mockDoc, state: "Signed" };
    const { container } = render(
      <MemoryRouter>
        <DocumentCard
          document={signedDoc}
          onDownload={vi.fn()}
          onDelete={vi.fn()}
        />
      </MemoryRouter>,
    );
    expect(screen.getByText("Signed")).toBeInTheDocument();
  });
});
