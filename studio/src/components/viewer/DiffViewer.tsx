import { Badge } from "@/components/ui/badge";
import { type DiffReport } from "@/lib/api-client";

const IMPACT_COLORS: Record<string, string> = {
  Critical: "bg-rose-500/20 text-rose-400",
  High: "bg-amber-500/20 text-amber-400",
  Medium: "bg-yellow-500/20 text-yellow-400",
  Low: "bg-blue-500/20 text-blue-400",
  None: "bg-zinc-500/20 text-zinc-400",
};

interface DiffViewerProps {
  report: DiffReport;
}

export function DiffViewer({ report }: DiffViewerProps) {
  if (report.identical) {
    return (
      <div className="rounded-xl border p-6 text-center text-sm text-muted-foreground">
        Documents are identical. No differences found.
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-2">
        <h3 className="text-sm font-semibold">
          {report.changes.length} changes
        </h3>
        {report.metadata_changes.length > 0 && (
          <Badge variant="secondary" className="text-xs">
            +{report.metadata_changes.length} metadata
          </Badge>
        )}
      </div>

      <div className="space-y-2">
        {report.changes.map((change, i) => (
          <div
            key={i}
            className="rounded-lg border p-3 text-sm"
          >
            <div className="flex items-center gap-2">
              <Badge variant="outline" className="text-[10px]">
                {change.change_type}
              </Badge>
              <span className="font-medium">{change.element_type}</span>
              {change.field && (
                <span className="text-muted-foreground">.{change.field}</span>
              )}
              <Badge
                variant="secondary"
                className={`ml-auto text-[10px] ${IMPACT_COLORS[change.impact] || ""}`}
              >
                {change.impact}
              </Badge>
            </div>
            {change.old_value !== undefined && (
              <div className="mt-2 grid grid-cols-2 gap-2 font-mono text-xs">
                <div className="rounded bg-rose-500/10 p-2">
                  <span className="text-rose-400">-</span>{" "}
                  {JSON.stringify(change.old_value)}
                </div>
                <div className="rounded bg-emerald-500/10 p-2">
                  <span className="text-emerald-400">+</span>{" "}
                  {JSON.stringify(change.new_value)}
                </div>
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
