import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { ElementTree } from "@/components/viewer/ElementTree";

const mockPages = [
  {
    eid: "pg-001",
    page_number: 1,
    elements: [
      { eid: "el-h1-001", element_type: "Heading", level: 1, text: "Title" },
      { eid: "el-p-001", element_type: "Paragraph", text: "Body text" },
    ],
  },
  {
    eid: "pg-002",
    page_number: 2,
    elements: [
      { eid: "el-t-001", element_type: "Table", rows: 3 },
    ],
  },
];

describe("ElementTree", () => {
  it("renders all pages expanded by default", () => {
    render(
      <ElementTree pages={mockPages} selectedEid={null} onSelect={vi.fn()} />,
    );
    expect(screen.getByText("Page 1")).toBeInTheDocument();
    expect(screen.getByText("Page 2")).toBeInTheDocument();
    expect(screen.getByText("Heading")).toBeInTheDocument();
    expect(screen.getByText("Paragraph")).toBeInTheDocument();
    expect(screen.getByText("Table")).toBeInTheDocument();
  });

  it("shows element count per page", () => {
    render(
      <ElementTree pages={mockPages} selectedEid={null} onSelect={vi.fn()} />,
    );
    expect(screen.getByText("2")).toBeInTheDocument(); // Page 1 has 2 elements
    expect(screen.getByText("1")).toBeInTheDocument(); // Page 2 has 1 element
  });

  it("collapses page on click", () => {
    render(
      <ElementTree pages={mockPages} selectedEid={null} onSelect={vi.fn()} />,
    );
    fireEvent.click(screen.getByText("Page 1"));
    expect(screen.queryByText("Heading")).not.toBeInTheDocument();
    expect(screen.getByText("Table")).toBeInTheDocument();
  });

  it("calls onSelect when element is clicked", () => {
    const onSelect = vi.fn();
    render(
      <ElementTree pages={mockPages} selectedEid={null} onSelect={onSelect} />,
    );
    fireEvent.click(screen.getByText("Heading"));
    expect(onSelect).toHaveBeenCalledWith("el-h1-001", expect.objectContaining({
      eid: "el-h1-001",
      element_type: "Heading",
    }));
  });

  it("highlights selected element", () => {
    render(
      <ElementTree
        pages={mockPages}
        selectedEid="el-h1-001"
        onSelect={vi.fn()}
      />,
    );
    const headingBtn = screen.getByText("Heading").closest("button");
    expect(headingBtn?.className).toContain("primary");
  });

  it("shows truncated EID for each element", () => {
    render(
      <ElementTree pages={mockPages} selectedEid={null} onSelect={vi.fn()} />,
    );
    expect(screen.getByText("el-h1-001")).toBeInTheDocument();
  });
});
