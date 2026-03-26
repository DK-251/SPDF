import { useState, type ReactNode } from "react";
import { Key } from "lucide-react";
import { useAuthStore } from "@/stores/auth-store";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@/components/ui/dialog";

interface AuthGateProps {
  children: ReactNode;
}

export function AuthGate({ children }: AuthGateProps) {
  const isAuthenticated = useAuthStore((s) => s.isAuthenticated);
  const setApiKey = useAuthStore((s) => s.setApiKey);
  const [key, setKey] = useState("");
  const [error, setError] = useState("");

  if (isAuthenticated) {
    return <>{children}</>;
  }

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const trimmed = key.trim();
    if (!trimmed) {
      setError("API key is required");
      return;
    }
    if (!trimmed.startsWith("sk_")) {
      setError("API key must start with sk_");
      return;
    }
    setApiKey(trimmed);
  };

  return (
    <Dialog open={true} onOpenChange={() => {}}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-primary/10">
            <Key className="h-6 w-6 text-primary" />
          </div>
          <DialogTitle className="text-center">Welcome to SPDF Studio</DialogTitle>
          <DialogDescription className="text-center">
            Enter your API key to get started. You can find this in your account
            settings.
          </DialogDescription>
        </DialogHeader>
        <form onSubmit={handleSubmit} className="mt-4 space-y-4">
          <div>
            <Input
              type="password"
              placeholder="sk_live_..."
              value={key}
              onChange={(e) => {
                setKey(e.target.value);
                setError("");
              }}
              className="font-mono"
              autoFocus
            />
            {error && (
              <p className="mt-1.5 text-xs text-destructive">{error}</p>
            )}
          </div>
          <Button type="submit" className="w-full">
            Connect
          </Button>
        </form>
      </DialogContent>
    </Dialog>
  );
}
