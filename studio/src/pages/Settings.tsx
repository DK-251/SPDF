import { useState, useEffect } from "react";
import { Key, BarChart3, CreditCard, Loader2, Copy, Check, LogOut } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { useAuthStore } from "@/stores/auth-store";
import {
  getApiKeyInfo,
  rotateApiKey,
  getUsage,
  getSubscription,
  type ApiKeyInfo,
  type UsageInfo,
  type SubscriptionInfo,
} from "@/lib/api-client";

export function Settings() {
  const clearApiKey = useAuthStore((s) => s.clearApiKey);
  const setApiKey = useAuthStore((s) => s.setApiKey);
  const [keyInfo, setKeyInfo] = useState<ApiKeyInfo | null>(null);
  const [usage, setUsage] = useState<UsageInfo | null>(null);
  const [subscription, setSubscription] = useState<SubscriptionInfo | null>(null);
  const [rotating, setRotating] = useState(false);
  const [newKey, setNewKey] = useState<string | null>(null);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    getApiKeyInfo().then(setKeyInfo).catch(() => {});
    getUsage().then(setUsage).catch(() => {});
    getSubscription().then(setSubscription).catch(() => {});
  }, []);

  const handleRotate = async () => {
    setRotating(true);
    try {
      const result = await rotateApiKey();
      setNewKey(result.api_key);
      setApiKey(result.api_key);
      setKeyInfo((prev) =>
        prev ? { ...prev, key_prefix: result.key_prefix } : null,
      );
    } catch {
      // silent
    } finally {
      setRotating(false);
    }
  };

  const handleCopy = async () => {
    if (!newKey) return;
    await navigator.clipboard.writeText(newKey);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="mx-auto max-w-3xl space-y-6">
      <div>
        <h2 className="text-xl font-semibold tracking-tight">Settings</h2>
        <p className="text-sm text-muted-foreground">
          Manage your API key, usage, and billing
        </p>
      </div>

      {/* API Key */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-base">
            <Key className="h-4 w-4" />
            API Key
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          {keyInfo && (
            <div className="flex items-center gap-3">
              <code className="rounded bg-muted px-3 py-1.5 font-mono text-sm">
                {keyInfo.key_prefix}...
              </code>
              <Badge variant="secondary" className="text-xs">
                Active
              </Badge>
            </div>
          )}

          {newKey && (
            <div className="rounded-lg border border-amber-500/30 bg-amber-500/10 p-3">
              <p className="mb-2 text-xs font-medium text-amber-400">
                New key (copy now — won't be shown again):
              </p>
              <div className="flex items-center gap-2">
                <code className="flex-1 truncate font-mono text-xs">
                  {newKey}
                </code>
                <Button variant="ghost" size="icon" className="h-7 w-7" onClick={handleCopy}>
                  {copied ? (
                    <Check className="h-3.5 w-3.5 text-emerald-400" />
                  ) : (
                    <Copy className="h-3.5 w-3.5" />
                  )}
                </Button>
              </div>
            </div>
          )}

          <div className="flex gap-2">
            <Button variant="outline" size="sm" onClick={handleRotate} disabled={rotating}>
              {rotating && <Loader2 className="mr-2 h-3.5 w-3.5 animate-spin" />}
              Rotate Key
            </Button>
            <Button variant="outline" size="sm" className="text-destructive" onClick={clearApiKey}>
              <LogOut className="mr-1.5 h-3.5 w-3.5" />
              Disconnect
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* Usage */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-base">
            <BarChart3 className="h-4 w-4" />
            Today's Usage
          </CardTitle>
        </CardHeader>
        <CardContent>
          {usage ? (
            <div className="space-y-3">
              <div className="flex items-center gap-2">
                <Badge variant="secondary" className="text-xs">
                  {usage.tier}
                </Badge>
                <span className="text-xs text-muted-foreground">
                  {usage.date}
                </span>
              </div>
              <div className="space-y-2">
                {Object.entries(usage.usage).map(([family, stats]) => {
                  const pct = stats.limit > 0 ? (stats.used / stats.limit) * 100 : 0;
                  return (
                    <div key={family}>
                      <div className="flex items-center justify-between text-xs">
                        <span className="font-medium capitalize">{family}</span>
                        <span className="text-muted-foreground">
                          {stats.used} / {stats.limit === -1 ? "\u221e" : stats.limit}
                        </span>
                      </div>
                      <div className="mt-1 h-1.5 rounded-full bg-muted">
                        <div
                          className="h-full rounded-full bg-primary transition-all"
                          style={{ width: `${Math.min(pct, 100)}%` }}
                        />
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>
          ) : (
            <p className="text-sm text-muted-foreground">Loading usage data...</p>
          )}
        </CardContent>
      </Card>

      {/* Billing */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-base">
            <CreditCard className="h-4 w-4" />
            Billing
          </CardTitle>
        </CardHeader>
        <CardContent>
          {subscription ? (
            <div className="space-y-3">
              <div className="flex items-center gap-3">
                <span className="text-sm font-medium">{subscription.tier} Plan</span>
                <Badge
                  variant="secondary"
                  className={
                    subscription.status === "active"
                      ? "bg-emerald-500/20 text-emerald-400"
                      : "text-xs"
                  }
                >
                  {subscription.status}
                </Badge>
              </div>
              {subscription.current_period_end && (
                <p className="text-xs text-muted-foreground">
                  Current period ends{" "}
                  {new Date(subscription.current_period_end).toLocaleDateString()}
                </p>
              )}
            </div>
          ) : (
            <p className="text-sm text-muted-foreground">Loading billing info...</p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
