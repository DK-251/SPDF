import { Routes, Route } from "react-router-dom";
import { Shell } from "@/components/layout/Shell";
import { Dashboard } from "@/pages/Dashboard";
import { Generate } from "@/pages/Generate";
import { DocumentView } from "@/pages/DocumentView";
import { Templates } from "@/pages/Templates";
import { Settings } from "@/pages/Settings";
import { NotFound } from "@/pages/NotFound";
import { ROUTES } from "@/lib/constants";

export function App() {
  return (
    <Shell>
      <Routes>
        <Route path={ROUTES.DASHBOARD} element={<Dashboard />} />
        <Route path={ROUTES.GENERATE} element={<Generate />} />
        <Route path={ROUTES.DOCUMENT} element={<DocumentView />} />
        <Route path={ROUTES.TEMPLATES} element={<Templates />} />
        <Route path={ROUTES.SETTINGS} element={<Settings />} />
        <Route path="*" element={<NotFound />} />
      </Routes>
    </Shell>
  );
}
