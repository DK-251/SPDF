import { Plus } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { DocumentCard } from "@/components/documents/DocumentCard";
import { UploadZone } from "@/components/documents/UploadZone";
import { useDocumentStore } from "@/stores/document-store";
import { useDocument } from "@/hooks/use-document";

export function Dashboard() {
  const documents = useDocumentStore((s) => s.documents);
  const removeDocument = useDocumentStore((s) => s.removeDocument);
  const { upload, download } = useDocument();
  const navigate = useNavigate();

  const handleFileSelect = (file: File) => {
    const entry = upload(file);
    if (entry) {
      navigate(`/documents/${entry.id}`);
    }
  };

  return (
    <div className="mx-auto max-w-5xl space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-xl font-semibold tracking-tight">Documents</h2>
          <p className="text-sm text-muted-foreground">
            Upload, generate, and manage SPDF documents
          </p>
        </div>
        <Button onClick={() => navigate("/generate")}>
          <Plus className="mr-2 h-4 w-4" />
          Generate
        </Button>
      </div>

      <UploadZone onFileSelect={handleFileSelect} />

      {documents.length === 0 ? (
        <div className="rounded-xl border border-dashed p-12 text-center">
          <p className="text-sm text-muted-foreground">
            No documents yet. Upload an SPDF file or generate a new one.
          </p>
        </div>
      ) : (
        <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
          {documents.map((doc) => (
            <DocumentCard
              key={doc.id}
              document={doc}
              onDownload={download}
              onDelete={removeDocument}
            />
          ))}
        </div>
      )}
    </div>
  );
}
