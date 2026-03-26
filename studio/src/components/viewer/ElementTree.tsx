import { useState } from "react";
import {
  ChevronRight,
  ChevronDown,
  Type,
  AlignLeft,
  Table,
  FileSignature,
  Image,
  List,
  Hash,
  Receipt,
  CreditCard,
  Shield,
  Stamp,
} from "lucide-react";
import { cn } from "@/lib/utils";

const ELEMENT_ICONS: Record<string, React.ElementType> = {
  Heading: Type,
  Paragraph: AlignLeft,
  Table: Table,
  LineItemTable: Table,
  SignatureBlock: FileSignature,
  Image: Image,
  List: List,
  InvoiceHeader: Receipt,
  PaymentTerms: CreditCard,
  RedactionMarker: Shield,
  Stamp: Stamp,
};

interface ElementNode {
  eid: string;
  element_type: string;
  [key: string]: unknown;
}

interface PageNode {
  eid: string;
  page_number: number;
  elements: ElementNode[];
}

interface ElementTreeProps {
  pages: PageNode[];
  selectedEid: string | null;
  onSelect: (eid: string, element: ElementNode) => void;
}

export function ElementTree({ pages, selectedEid, onSelect }: ElementTreeProps) {
  const [expandedPages, setExpandedPages] = useState<Set<number>>(() => {
    return new Set(pages.map((_, i) => i));
  });

  const togglePage = (index: number) => {
    setExpandedPages((prev) => {
      const next = new Set(prev);
      if (next.has(index)) next.delete(index);
      else next.add(index);
      return next;
    });
  };

  return (
    <div className="space-y-1 text-sm">
      {pages.map((page, pageIndex) => {
        const isExpanded = expandedPages.has(pageIndex);
        return (
          <div key={page.eid}>
            <button
              onClick={() => togglePage(pageIndex)}
              className="flex w-full items-center gap-1.5 rounded-md px-2 py-1 text-left text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
            >
              {isExpanded ? (
                <ChevronDown className="h-3.5 w-3.5" />
              ) : (
                <ChevronRight className="h-3.5 w-3.5" />
              )}
              <span className="font-medium">Page {page.page_number}</span>
              <span className="ml-auto text-xs text-muted-foreground">
                {page.elements.length}
              </span>
            </button>

            {isExpanded && (
              <div className="ml-4 space-y-0.5 border-l pl-2">
                {page.elements.map((el) => {
                  const Icon = ELEMENT_ICONS[el.element_type] || Hash;
                  const isSelected = selectedEid === el.eid;
                  return (
                    <button
                      key={el.eid}
                      onClick={() => onSelect(el.eid, el)}
                      className={cn(
                        "flex w-full items-center gap-2 rounded-md px-2 py-1 text-left transition-colors",
                        isSelected
                          ? "bg-primary/10 text-primary"
                          : "text-muted-foreground hover:bg-accent hover:text-foreground",
                      )}
                    >
                      <Icon className="h-3.5 w-3.5 shrink-0" />
                      <span className="truncate">{el.element_type}</span>
                      <span className="ml-auto truncate font-mono text-[10px] text-muted-foreground">
                        {el.eid.slice(0, 12)}
                      </span>
                    </button>
                  );
                })}
              </div>
            )}
          </div>
        );
      })}
    </div>
  );
}
