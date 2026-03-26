import { create } from "zustand";
import { LOCAL_STORAGE_KEYS } from "@/lib/constants";

interface AuthState {
  apiKey: string | null;
  isAuthenticated: boolean;
  setApiKey: (key: string) => void;
  clearApiKey: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  apiKey: localStorage.getItem(LOCAL_STORAGE_KEYS.API_KEY),
  isAuthenticated: !!localStorage.getItem(LOCAL_STORAGE_KEYS.API_KEY),

  setApiKey: (key: string) => {
    localStorage.setItem(LOCAL_STORAGE_KEYS.API_KEY, key);
    set({ apiKey: key, isAuthenticated: true });
  },

  clearApiKey: () => {
    localStorage.removeItem(LOCAL_STORAGE_KEYS.API_KEY);
    set({ apiKey: null, isAuthenticated: false });
  },
}));
