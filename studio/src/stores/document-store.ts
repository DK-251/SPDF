import { create } from "zustand";

export interface DocumentEntry {
  id: string;
  name: string;
  state: string;
  size: number;
  uploadedAt: string;
  blob: Blob;
}

interface DocumentState {
  documents: DocumentEntry[];
  activeDocumentId: string | null;
  addDocument: (entry: DocumentEntry) => void;
  removeDocument: (id: string) => void;
  setActiveDocument: (id: string | null) => void;
  getActiveDocument: () => DocumentEntry | undefined;
  updateDocument: (id: string, updates: Partial<DocumentEntry>) => void;
}

export const useDocumentStore = create<DocumentState>((set, get) => ({
  documents: [],
  activeDocumentId: null,

  addDocument: (entry) =>
    set((state) => ({
      documents: [entry, ...state.documents],
    })),

  removeDocument: (id) =>
    set((state) => ({
      documents: state.documents.filter((d) => d.id !== id),
      activeDocumentId:
        state.activeDocumentId === id ? null : state.activeDocumentId,
    })),

  setActiveDocument: (id) => set({ activeDocumentId: id }),

  getActiveDocument: () => {
    const state = get();
    return state.documents.find((d) => d.id === state.activeDocumentId);
  },

  updateDocument: (id, updates) =>
    set((state) => ({
      documents: state.documents.map((d) =>
        d.id === id ? { ...d, ...updates } : d,
      ),
    })),
}));
