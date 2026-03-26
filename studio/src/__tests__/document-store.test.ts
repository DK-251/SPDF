import { describe, it, expect, beforeEach } from "vitest";
import { useDocumentStore, type DocumentEntry } from "@/stores/document-store";

function makeDoc(overrides: Partial<DocumentEntry> = {}): DocumentEntry {
  return {
    id: `doc_${Date.now()}_${Math.random().toString(36).slice(2, 6)}`,
    name: "test.spdf",
    state: "Draft",
    size: 1024,
    uploadedAt: new Date().toISOString(),
    blob: new Blob(["test"]),
    ...overrides,
  };
}

describe("document-store", () => {
  beforeEach(() => {
    useDocumentStore.setState({
      documents: [],
      activeDocumentId: null,
    });
  });

  it("starts with empty documents", () => {
    expect(useDocumentStore.getState().documents).toHaveLength(0);
  });

  it("addDocument prepends to list", () => {
    const doc1 = makeDoc({ id: "d1", name: "first.spdf" });
    const doc2 = makeDoc({ id: "d2", name: "second.spdf" });
    useDocumentStore.getState().addDocument(doc1);
    useDocumentStore.getState().addDocument(doc2);
    const docs = useDocumentStore.getState().documents;
    expect(docs).toHaveLength(2);
    expect(docs[0].id).toBe("d2");
    expect(docs[1].id).toBe("d1");
  });

  it("removeDocument filters by id", () => {
    const doc = makeDoc({ id: "d1" });
    useDocumentStore.getState().addDocument(doc);
    useDocumentStore.getState().removeDocument("d1");
    expect(useDocumentStore.getState().documents).toHaveLength(0);
  });

  it("removeDocument clears activeDocumentId if it matches", () => {
    const doc = makeDoc({ id: "d1" });
    useDocumentStore.getState().addDocument(doc);
    useDocumentStore.getState().setActiveDocument("d1");
    useDocumentStore.getState().removeDocument("d1");
    expect(useDocumentStore.getState().activeDocumentId).toBeNull();
  });

  it("removeDocument preserves activeDocumentId for other docs", () => {
    const doc1 = makeDoc({ id: "d1" });
    const doc2 = makeDoc({ id: "d2" });
    useDocumentStore.getState().addDocument(doc1);
    useDocumentStore.getState().addDocument(doc2);
    useDocumentStore.getState().setActiveDocument("d1");
    useDocumentStore.getState().removeDocument("d2");
    expect(useDocumentStore.getState().activeDocumentId).toBe("d1");
  });

  it("setActiveDocument updates activeDocumentId", () => {
    useDocumentStore.getState().setActiveDocument("d1");
    expect(useDocumentStore.getState().activeDocumentId).toBe("d1");
  });

  it("getActiveDocument returns matching document", () => {
    const doc = makeDoc({ id: "d1" });
    useDocumentStore.getState().addDocument(doc);
    useDocumentStore.getState().setActiveDocument("d1");
    const active = useDocumentStore.getState().getActiveDocument();
    expect(active?.id).toBe("d1");
  });

  it("getActiveDocument returns undefined when no match", () => {
    const active = useDocumentStore.getState().getActiveDocument();
    expect(active).toBeUndefined();
  });

  it("updateDocument merges partial updates", () => {
    const doc = makeDoc({ id: "d1", state: "Draft" });
    useDocumentStore.getState().addDocument(doc);
    useDocumentStore.getState().updateDocument("d1", { state: "Signed" });
    const updated = useDocumentStore.getState().documents.find((d) => d.id === "d1");
    expect(updated?.state).toBe("Signed");
    expect(updated?.name).toBe("test.spdf");
  });
});
