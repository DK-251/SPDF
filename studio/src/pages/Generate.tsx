import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { Loader2, Sparkles } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useDocument } from "@/hooks/use-document";

const SAMPLE_SEMANTIC = {
  version: "1.0",
  document_id: "doc-preview-001",
  title: "Sample Invoice",
  locale: "en-US",
  direction: "Ltr",
  document_state: "Draft",
  pages: [
    {
      eid: "pg-001",
      page_number: 1,
      elements: [
        {
          element_type: "Heading",
          eid: "el-heading-001",
          level: 1,
          text: "Invoice #INV-2026-001",
          timestamps: { created_at: new Date().toISOString(), updated_at: new Date().toISOString() },
        },
        {
          element_type: "InvoiceHeader",
          eid: "el-inv-001",
          invoice_number: "INV-2026-001",
          issue_date: "2026-03-26",
          due_date: "2026-04-26",
          currency: "USD",
          seller_name: "SPDF Corp",
          buyer_name: "Acme Inc",
          timestamps: { created_at: new Date().toISOString(), updated_at: new Date().toISOString() },
        },
        {
          element_type: "PaymentTerms",
          eid: "el-pay-001",
          subtotal: "2000.00",
          tax: "200.00",
          total: "2200.00",
          payment_method: "Bank Transfer",
          timestamps: { created_at: new Date().toISOString(), updated_at: new Date().toISOString() },
        },
      ],
    },
  ],
};

export function Generate() {
  const navigate = useNavigate();
  const { generate, generateState } = useDocument();
  const [name, setName] = useState("Untitled Document");
  const [semantic, setSemantic] = useState(
    JSON.stringify(SAMPLE_SEMANTIC, null, 2),
  );
  const [jsonError, setJsonError] = useState<string | null>(null);

  const handleGenerate = async () => {
    setJsonError(null);
    let parsed: Record<string, unknown>;
    try {
      parsed = JSON.parse(semantic);
    } catch (err) {
      setJsonError(err instanceof Error ? err.message : "Invalid JSON");
      return;
    }

    try {
      const entry = await generate({ semantic: parsed }, name);
      if (entry) {
        navigate(`/documents/${entry.id}`);
      }
    } catch {
      // error state handled by hook
    }
  };

  const handleLoadSample = () => {
    setSemantic(JSON.stringify(SAMPLE_SEMANTIC, null, 2));
    setName("Sample Invoice");
    setJsonError(null);
  };

  return (
    <div className="mx-auto max-w-4xl space-y-6">
      <div>
        <h2 className="text-xl font-semibold tracking-tight">
          Generate Document
        </h2>
        <p className="text-sm text-muted-foreground">
          Create an SPDF document from semantic JSON
        </p>
      </div>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Document Details</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div>
            <label className="mb-1.5 block text-sm font-medium">
              Document Name
            </label>
            <Input
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="My Document"
            />
          </div>

          <div>
            <div className="mb-1.5 flex items-center justify-between">
              <label className="text-sm font-medium">Semantic JSON</label>
              <Button
                variant="ghost"
                size="sm"
                onClick={handleLoadSample}
                className="text-xs"
              >
                <Sparkles className="mr-1 h-3 w-3" />
                Load Sample
              </Button>
            </div>
            <Textarea
              value={semantic}
              onChange={(e) => {
                setSemantic(e.target.value);
                setJsonError(null);
              }}
              className="min-h-[400px] font-mono text-xs"
              placeholder="Paste semantic JSON here..."
            />
            {jsonError && (
              <p className="mt-1.5 text-xs text-destructive">{jsonError}</p>
            )}
            {generateState.error && (
              <p className="mt-1.5 text-xs text-destructive">
                {generateState.error}
              </p>
            )}
          </div>

          <Button
            onClick={handleGenerate}
            disabled={generateState.loading}
            className="w-full"
          >
            {generateState.loading && (
              <Loader2 className="mr-2 h-4 w-4 animate-spin" />
            )}
            Generate SPDF
          </Button>
        </CardContent>
      </Card>
    </div>
  );
}
