import { useState } from "react";
import {
  PenTool,
  ShieldCheck,
  ArrowRightLeft,
  Download,
  FileDown,
  Loader2,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Badge } from "@/components/ui/badge";
import { type DocumentEntry } from "@/stores/document-store";
import { useDocument } from "@/hooks/use-document";
import { STATE_COLORS } from "@/lib/constants";
import { cn } from "@/lib/utils";

interface DocumentActionsProps {
  document: DocumentEntry;
}

export function DocumentActions({ document: doc }: DocumentActionsProps) {
  const { sign, verify, download, render, verifyState } = useDocument();
  const [signOpen, setSignOpen] = useState(false);
  const [signerName, setSignerName] = useState("");
  const [signerEmail, setSignerEmail] = useState("");
  const [signing, setSigning] = useState(false);
  const [rendering, setRendering] = useState(false);

  const handleSign = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!signerName || !signerEmail) return;
    setSigning(true);
    try {
      await sign(doc, signerName, signerEmail);
      setSignOpen(false);
    } catch {
      // error handled by hook
    } finally {
      setSigning(false);
    }
  };

  const handleVerify = async () => {
    await verify(doc);
  };

  const handleDownloadPdf = async () => {
    setRendering(true);
    try {
      const pdfBlob = await render(doc);
      const url = URL.createObjectURL(pdfBlob);
      const a = document.createElement("a");
      a.href = url;
      a.download = doc.name.replace(/\.spdf$/, ".pdf");
      a.click();
      URL.revokeObjectURL(url);
    } catch {
      // error handled upstream
    } finally {
      setRendering(false);
    }
  };

  const stateColor = STATE_COLORS[doc.state] || STATE_COLORS.Draft;

  return (
    <>
      <div className="flex flex-wrap items-center gap-2">
        <Badge variant="secondary" className={cn("text-xs", stateColor)}>
          {doc.state}
        </Badge>

        {doc.state === "Review" && (
          <Button variant="outline" size="sm" onClick={() => setSignOpen(true)}>
            <PenTool className="mr-1.5 h-3.5 w-3.5" />
            Sign
          </Button>
        )}

        <Button variant="outline" size="sm" onClick={handleVerify}>
          <ShieldCheck className="mr-1.5 h-3.5 w-3.5" />
          Verify
        </Button>

        <Button variant="outline" size="sm" onClick={() => download(doc)}>
          <Download className="mr-1.5 h-3.5 w-3.5" />
          SPDF
        </Button>

        <Button
          variant="outline"
          size="sm"
          onClick={handleDownloadPdf}
          disabled={rendering}
        >
          {rendering ? (
            <Loader2 className="mr-1.5 h-3.5 w-3.5 animate-spin" />
          ) : (
            <FileDown className="mr-1.5 h-3.5 w-3.5" />
          )}
          PDF
        </Button>
      </div>

      {verifyState.data && (
        <div className="mt-3 rounded-lg border p-3 text-sm">
          <p>
            Signatures: <strong>{verifyState.data.signature_count}</strong>
          </p>
          <p>
            Valid:{" "}
            <strong className={verifyState.data.valid ? "text-emerald-400" : "text-rose-400"}>
              {verifyState.data.valid ? "Yes" : "No"}
            </strong>
          </p>
          {verifyState.data.tamper_detected && (
            <p className="mt-1 font-medium text-rose-400">
              Tamper detected!
            </p>
          )}
        </div>
      )}

      <Dialog open={signOpen} onOpenChange={setSignOpen}>
        <DialogContent onClose={() => setSignOpen(false)}>
          <DialogHeader>
            <DialogTitle>Sign Document</DialogTitle>
          </DialogHeader>
          <form onSubmit={handleSign} className="mt-4 space-y-4">
            <div>
              <label className="mb-1.5 block text-sm font-medium">Name</label>
              <Input
                value={signerName}
                onChange={(e) => setSignerName(e.target.value)}
                placeholder="John Doe"
                required
              />
            </div>
            <div>
              <label className="mb-1.5 block text-sm font-medium">Email</label>
              <Input
                type="email"
                value={signerEmail}
                onChange={(e) => setSignerEmail(e.target.value)}
                placeholder="john@example.com"
                required
              />
            </div>
            <Button type="submit" className="w-full" disabled={signing}>
              {signing && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
              Sign Document
            </Button>
          </form>
        </DialogContent>
      </Dialog>
    </>
  );
}
