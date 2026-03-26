import { useState, useEffect } from "react";
import { FileWarning, Loader2 } from "lucide-react";
import { renderDocument } from "@/lib/api-client";

interface PdfPreviewProps {
  blob: Blob;
}

export function PdfPreview({ blob }: PdfPreviewProps) {
  const [pdfUrl, setPdfUrl] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    setError(null);

    renderDocument(blob)
      .then((pdfBlob) => {
        if (cancelled) return;
        const url = URL.createObjectURL(pdfBlob);
        setPdfUrl(url);
        setLoading(false);
      })
      .catch((err) => {
        if (cancelled) return;
        setError(err instanceof Error ? err.message : "Failed to render PDF");
        setLoading(false);
      });

    return () => {
      cancelled = true;
      if (pdfUrl) URL.revokeObjectURL(pdfUrl);
    };
    // Only re-render when the blob identity changes
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [blob]);

  if (loading) {
    return (
      <div className="flex h-full items-center justify-center">
        <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex h-full flex-col items-center justify-center gap-2 text-muted-foreground">
        <FileWarning className="h-8 w-8" />
        <p className="text-sm">{error}</p>
      </div>
    );
  }

  return (
    <iframe
      src={pdfUrl ?? undefined}
      className="h-full w-full rounded-lg border-0"
      title="PDF Preview"
    />
  );
}
