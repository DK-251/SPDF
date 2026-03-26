import { useCallback } from "react";
import { useDocumentStore, type DocumentEntry } from "@/stores/document-store";
import {
  generateDocument,
  validateDocument,
  renderDocument,
  extractInvoice,
  signDocument,
  verifyDocument,
  type GeneratePayload,
  type ValidationReport,
  type VerificationReport,
  type InvoiceData,
} from "@/lib/api-client";
import { useApi } from "./use-api";

function generateId(): string {
  return `doc_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`;
}

export function useDocument() {
  const addDocument = useDocumentStore((s) => s.addDocument);
  const updateDocument = useDocumentStore((s) => s.updateDocument);
  const generateApi = useApi<Blob>();
  const validateApi = useApi<ValidationReport>();
  const verifyApi = useApi<VerificationReport>();
  const extractApi = useApi<InvoiceData>();

  const upload = useCallback(
    (file: File) => {
      const entry: DocumentEntry = {
        id: generateId(),
        name: file.name,
        state: "Unknown",
        size: file.size,
        uploadedAt: new Date().toISOString(),
        blob: file,
      };
      addDocument(entry);
      return entry;
    },
    [addDocument],
  );

  const generate = useCallback(
    async (payload: GeneratePayload, name: string) => {
      const blob = await generateApi.execute(() => generateDocument(payload));
      if (!blob) return null;
      const entry: DocumentEntry = {
        id: generateId(),
        name,
        state: "Draft",
        size: blob.size,
        uploadedAt: new Date().toISOString(),
        blob,
      };
      addDocument(entry);
      return entry;
    },
    [addDocument, generateApi],
  );

  const validate = useCallback(
    (doc: DocumentEntry) =>
      validateApi.execute(() => validateDocument(doc.blob)),
    [validateApi],
  );

  const verify = useCallback(
    (doc: DocumentEntry) =>
      verifyApi.execute(() => verifyDocument(doc.blob)),
    [verifyApi],
  );

  const extract = useCallback(
    (doc: DocumentEntry) =>
      extractApi.execute(() => extractInvoice(doc.blob)),
    [extractApi],
  );

  const sign = useCallback(
    async (doc: DocumentEntry, signerName: string, signerEmail: string) => {
      const signedBlob = await signDocument(doc.blob, signerName, signerEmail);
      updateDocument(doc.id, {
        blob: signedBlob,
        state: "Signed",
        size: signedBlob.size,
      });
      return signedBlob;
    },
    [updateDocument],
  );

  const render = useCallback(async (doc: DocumentEntry) => {
    const pdfBlob = await renderDocument(doc.blob);
    return pdfBlob;
  }, []);

  const download = useCallback((doc: DocumentEntry) => {
    const url = URL.createObjectURL(doc.blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = doc.name.endsWith(".spdf") ? doc.name : `${doc.name}.spdf`;
    a.click();
    URL.revokeObjectURL(url);
  }, []);

  return {
    upload,
    generate,
    validate,
    verify,
    extract,
    sign,
    render,
    download,
    generateState: generateApi,
    validateState: validateApi,
    verifyState: verifyApi,
    extractState: extractApi,
  };
}
