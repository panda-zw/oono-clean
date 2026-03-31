import { useScanStore } from "../lib/stores/scan";
import { useThemeStore } from "../lib/stores/theme";

interface Props {
  view: "scan" | "audit" | "guide";
  onViewChange: (view: "scan" | "audit" | "guide") => void;
}

export function Header({ view, onViewChange }: Props) {
  const { isScanning, startScan } = useScanStore();
  const { theme, toggle } = useThemeStore();

  return (
    <header className="header">
      <div className="header__left">
        <img
          src="/logo.png"
          alt="OneSweep"
          className="header__logo"
        />
        <h1 className="header__title">OneSweep</h1>
      </div>
      <nav className="header__nav">
        <button
          className={`header__tab ${view === "scan" ? "active" : ""}`}
          onClick={() => onViewChange("scan")}
        >
          Scan
        </button>
        <button
          className={`header__tab ${view === "audit" ? "active" : ""}`}
          onClick={() => onViewChange("audit")}
        >
          History
        </button>
        <button
          className={`header__tab ${view === "guide" ? "active" : ""}`}
          onClick={() => onViewChange("guide")}
        >
          Guide
        </button>
      </nav>
      <div className="header__right">
        <button
          className="theme-toggle"
          onClick={toggle}
          title={`Switch to ${theme === "dark" ? "light" : "dark"} mode`}
        >
          {theme === "dark" ? "\u2600" : "\u263E"}
        </button>
        <button
          className="btn btn--primary"
          onClick={startScan}
          disabled={isScanning}
        >
          {isScanning ? "Scanning..." : "Scan Now"}
        </button>
      </div>
    </header>
  );
}
