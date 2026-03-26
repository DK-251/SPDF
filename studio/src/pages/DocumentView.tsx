import { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { ArrowLeft, Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { PdfPreview } from "@/components/viewer/PdfPreview";
import { ElementTree } from "@/components/viewer/ElementTree";
import { PropertyPanel } from "@/components/viewer/PropertyPanel";
import { DocumentActions } from "@/components/documents/DocumentActions";
import { Separator } from "@/components/ui/separator";
import { useDocumentStore } from "@/stores/document-store";
import { parseDocument, type ParseResult } from "@/lib/api-client";

export function DocumentView() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const documents = useDocumentStore((s) => s.documents);
  const doc = documents.find((d) => d.id === id);

  const [parsed, setParsed] = useState<ParseResult | null>(null);
  const [loading, setLoading] = useState(true);
  const [selectedEid, setSelectedEid] = useState<string | null>(null);
  const [selectedElement, setSelectedElement] = useState<Record<string, unknown> | null>(null);

  useEffect(() => {
    if (!doc) return;
    setLoading(true);

    const reader = new FileReader();
    reader.onload = async () => {
      try {
        // Try to extract semantic from the blob by sending to parse endpoint
        // For now, we'll use the validate approach to get document info
        const text = reader.result as string;
        // The blob is an SPDF file, not raw JSON. We'll extract via the API.
        setParsed(null);
      } catch {
        setParsed(null);
      } finally {
        setLoading(false);
      }
    };
    reader.onerror = () => setLoading(false);
    reader.readAsText(doc.blob.slice(0, 100));

    // Also try extracting document structure via extract endpoint
    extractStructure();

    async function extractStructure() {
      if (!doc) return;
      try {
        const form = new FormData();
        form.append("file", doc.blob, "doc.spdf");
        const res = await fetch("/api/v1/documents/validate", {
          method: "POST",
          headers: {
            Authorization: `Bearer ${localStorage.getItem("spdf_api_key") || ""}`,
          },
          body: form,
        });
        if (res.ok) {
          const data = await res.json();
          setParsed({ document: data, validation: data });
        }
      } catch {
        // silently fail
      } finally {
        setLoading(false);
      }
    }
  }, [doc]);

  if (!doc) {
    return (
      <div className="flex flex-col items-center justify-center gap-4 pt-20">
        <p className="text-sm text-muted-foreground">Document not found</p>
        <Button variant="outline" onClick={() => navigate("/")}>
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back to Documents
        </Button>
      </div>
    );
  }

  return (
    <div className="flex h-full flex-col gap-4">
      {/* Top bar */}
      <div className="flex items-center gap-4">
        <Button variant="ghost" size="sm" onClick={() => navigate("/")}>
          <ArrowLeft className="mr-1.5 h-4 w-4" />
          Back
        </Button>
        <div className="min-w-0 flex-1">
          <h2 className="truncate text-sm font-semibold">{doc.name}</h2>
        </div>
        <DocumentActions document={doc} />
      </div>

      <Separator />

      {/* Split pane */}
      <div className="grid flex-1 grid-cols-5 gap-4 overflow-hidden">
        {/* PDF Preview — 60% */}
        <div className="col-span-3 overflow-hidden rounded-xl border">
          <PdfPreview blob={doc.blob} />
        </div>

        {/* Right panel — 40% */}
        <div className="col-span-2 flex flex-col gap-4 overflow-hidden">
          <div className="flex-1 overflow-y-auto rounded-xl border p-4">
            <h3 className="mb-3 text-xs font-semibold uppercase tracking-wider text-muted-foreground">
              Element Tree
            </h3>
            {loading ? (
              <div className="flex items-center justify-center py-8">
                <Loader2 className="h-5 w-5 animate-spin text-muted-foreground" />
              </div>
            ) : parsed?.document?.pages ? (
              <ElementTree
                pages={parsed.document.pages as never[]}
                selectedEid={selectedEid}
                onSelect={(eid, el) => {
                  setSelectedEid(eid);
                  setSelectedElement(el);
                }}
              />
            ) : (
              <p className="text-xs text-muted-foreground">
                Upload a valid SPDF to see its element tree
              </p>
            )}
          </div>

          <div className="h-64 shrink-0 overflow-y-auto rounded-xl border p-4">
            <h3 className="mb-3 text-xs font-semibold uppercase tracking-wider text-muted-foreground">
              Properties
            </h3>
            <PropertyPanel element={selectedElement} />
          </div>
        </div>
      </div>
    </div>
  );
}
