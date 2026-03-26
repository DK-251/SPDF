import { useNavigate } from "react-router-dom";
import { FileText, Download, Trash2, MoreVertical } from "lucide-react";
import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { type DocumentEntry } from "@/stores/document-store";
import { STATE_COLORS } from "@/lib/constants";
import { fileSize, formatDate } from "@/lib/utils";
import { cn } from "@/lib/utils";

interface DocumentCardProps {
  document: DocumentEntry;
  onDownload: (doc: DocumentEntry) => void;
  onDelete: (id: string) => void;
}

export function DocumentCard({
  document: doc,
  onDownload,
  onDelete,
}: DocumentCardProps) {
  const navigate = useNavigate();

  const stateColor = STATE_COLORS[doc.state] || STATE_COLORS.Draft;

  return (
    <Card
      className="group cursor-pointer transition-all hover:border-zinc-600 hover:shadow-md"
      onClick={() => navigate(`/documents/${doc.id}`)}
    >
      <CardContent className="p-4">
        <div className="flex items-start justify-between">
          <div className="flex items-start gap-3">
            <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-primary/10">
              <FileText className="h-5 w-5 text-primary" />
            </div>
            <div className="min-w-0">
              <p className="truncate text-sm font-medium">{doc.name}</p>
              <div className="mt-1 flex items-center gap-2">
                <Badge
                  variant="secondary"
                  className={cn("text-[10px]", stateColor)}
                >
                  {doc.state}
                </Badge>
                <span className="text-xs text-muted-foreground">
                  {fileSize(doc.size)}
                </span>
              </div>
            </div>
          </div>
          <div className="flex gap-1 opacity-0 transition-opacity group-hover:opacity-100">
            <Button
              variant="ghost"
              size="icon"
              className="h-8 w-8"
              onClick={(e) => {
                e.stopPropagation();
                onDownload(doc);
              }}
            >
              <Download className="h-3.5 w-3.5" />
            </Button>
            <Button
              variant="ghost"
              size="icon"
              className="h-8 w-8 text-destructive hover:text-destructive"
              onClick={(e) => {
                e.stopPropagation();
                onDelete(doc.id);
              }}
            >
              <Trash2 className="h-3.5 w-3.5" />
            </Button>
          </div>
        </div>
        <p className="mt-2 text-[11px] text-muted-foreground">
          {formatDate(doc.uploadedAt)}
        </p>
      </CardContent>
    </Card>
  );
}
