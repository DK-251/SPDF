import { useState, useCallback } from "react";
import { ApiError } from "@/lib/api-client";

interface UseApiState<T> {
  data: T | null;
  error: string | null;
  loading: boolean;
}

export function useApi<T>() {
  const [state, setState] = useState<UseApiState<T>>({
    data: null,
    error: null,
    loading: false,
  });

  const execute = useCallback(async (apiCall: () => Promise<T>) => {
    setState({ data: null, error: null, loading: true });
    try {
      const data = await apiCall();
      setState({ data, error: null, loading: false });
      return data;
    } catch (err) {
      const message =
        err instanceof ApiError
          ? err.detail
          : err instanceof Error
            ? err.message
            : "Unknown error";
      setState({ data: null, error: message, loading: false });
      throw err;
    }
  }, []);

  const reset = useCallback(() => {
    setState({ data: null, error: null, loading: false });
  }, []);

  return { ...state, execute, reset };
}
