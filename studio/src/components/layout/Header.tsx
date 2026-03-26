import { useLocation } from "react-router-dom";
import { Key, Shield } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { useAuthStore } from "@/stores/auth-store";

const routeTitles: Record<string, string> = {
  "/": "Documents",
  "/generate": "Generate Document",
  "/templates": "Templates",
  "/settings": "Settings",
};

export function Header() {
  const location = useLocation();
  const isAuthenticated = useAuthStore((s) => s.isAuthenticated);
  const apiKey = useAuthStore((s) => s.apiKey);

  const title =
    routeTitles[location.pathname] ||
    (location.pathname.startsWith("/documents/") ? "Document Viewer" : "SPDF Studio");

  const keyPrefix = apiKey ? apiKey.slice(0, 7) + "..." : null;

  return (
    <header className="flex h-14 items-center justify-between border-b px-6">
      <h1 className="text-sm font-semibold tracking-tight">{title}</h1>
      <div className="flex items-center gap-3">
        {isAuthenticated ? (
          <Badge variant="secondary" className="gap-1.5 font-mono text-xs">
            <Key className="h-3 w-3" />
            {keyPrefix}
          </Badge>
        ) : (
          <Badge variant="outline" className="gap-1.5 text-xs text-amber-400">
            <Shield className="h-3 w-3" />
            No API Key
          </Badge>
        )}
      </div>
    </header>
  );
}
