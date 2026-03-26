import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { UploadZone } from "@/components/documents/UploadZone";

describe("UploadZone", () => {
  it("renders upload prompt", () => {
    render(<UploadZone onFileSelect={vi.fn()} />);
    expect(screen.getByText("Drop an SPDF file here")).toBeInTheDocument();
    expect(screen.getByText("or click to browse")).toBeInTheDocument();
  });

  it("has a hidden file input", () => {
    render(<UploadZone onFileSelect={vi.fn()} />);
    const input = screen.getByLabelText("Upload file");
    expect(input).toBeInTheDocument();
    expect(input).toHaveAttribute("type", "file");
  });

  it("calls onFileSelect when a file is selected via input", () => {
    const onSelect = vi.fn();
    render(<UploadZone onFileSelect={onSelect} />);
    const input = screen.getByLabelText("Upload file");
    const file = new File(["content"], "test.spdf", {
      type: "application/octet-stream",
    });
    fireEvent.change(input, { target: { files: [file] } });
    expect(onSelect).toHaveBeenCalledWith(file);
  });

  it("shows drag state text on dragover", () => {
    render(<UploadZone onFileSelect={vi.fn()} />);
    const zone = screen.getByRole("button");
    fireEvent.dragOver(zone);
    expect(screen.getByText("Drop to upload")).toBeInTheDocument();
  });

  it("reverts drag state on dragleave", () => {
    render(<UploadZone onFileSelect={vi.fn()} />);
    const zone = screen.getByRole("button");
    fireEvent.dragOver(zone);
    fireEvent.dragLeave(zone);
    expect(screen.getByText("Drop an SPDF file here")).toBeInTheDocument();
  });

  it("accepts custom accept prop", () => {
    render(<UploadZone onFileSelect={vi.fn()} accept=".pdf,.spdf" />);
    const input = screen.getByLabelText("Upload file");
    expect(input).toHaveAttribute("accept", ".pdf,.spdf");
  });
});
