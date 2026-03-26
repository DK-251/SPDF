import { Separator } from "@/components/ui/separator";

interface PropertyPanelProps {
  element: Record<string, unknown> | null;
}

function formatValue(value: unknown): string {
  if (value === null || value === undefined) return "\u2014";
  if (typeof value === "boolean") return value ? "Yes" : "No";
  if (typeof value === "object") return JSON.stringify(value, null, 2);
  return String(value);
}

const SKIP_KEYS = new Set(["timestamps"]);

export function PropertyPanel({ element }: PropertyPanelProps) {
  if (!element) {
    return (
      <div className="flex h-full items-center justify-center text-sm text-muted-foreground">
        Select an element to view properties
      </div>
    );
  }

  const entries = Object.entries(element).filter(
    ([key]) => !SKIP_KEYS.has(key),
  );

  return (
    <div className="space-y-3">
      <h3 className="text-sm font-semibold">
        {String(element.element_type || "Element")}
      </h3>
      <Separator />
      <div className="space-y-2">
        {entries.map(([key, value]) => (
          <div key={key}>
            <dt className="text-[11px] font-medium uppercase tracking-wider text-muted-foreground">
              {key.replace(/_/g, " ")}
            </dt>
            <dd className="mt-0.5 break-all font-mono text-xs">
              {typeof value === "object" && value !== null ? (
                <pre className="max-h-32 overflow-auto rounded bg-muted/50 p-2 text-[11px]">
                  {formatValue(value)}
                </pre>
              ) : (
                formatValue(value)
              )}
            </dd>
          </div>
        ))}
      </div>
    </div>
  );
}
