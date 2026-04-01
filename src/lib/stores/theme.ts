import { create } from "zustand";

type Theme = "dark" | "light";

interface ThemeState {
  theme: Theme;
  toggle: () => void;
}

function getInitialTheme(): Theme {
  const stored = localStorage.getItem("onepurge-theme");
  if (stored === "light" || stored === "dark") return stored;
  // Respect system preference
  if (window.matchMedia("(prefers-color-scheme: light)").matches) return "light";
  return "dark";
}

function applyTheme(theme: Theme) {
  document.documentElement.setAttribute("data-theme", theme);
  localStorage.setItem("onepurge-theme", theme);
}

export const useThemeStore = create<ThemeState>((set, get) => {
  // Apply initial theme immediately
  const initial = getInitialTheme();
  applyTheme(initial);

  return {
    theme: initial,
    toggle() {
      const next = get().theme === "dark" ? "light" : "dark";
      applyTheme(next);
      set({ theme: next });
    },
  };
});
