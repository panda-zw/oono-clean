import { useScanStore } from "../lib/stores/scan";
import { formatBytes } from "../lib/utils/format";
import { CategoryCard } from "./CategoryCard";

export function ScanView() {
  const { result, isScanning, selectedIds, selectAllGreen, clearSelection } =
    useScanStore();

  if (isScanning) {
    return (
      <div className="scan-view scan-view--loading">
        <div className="spinner" />
        <p>Scanning your system for reclaimable space...</p>
      </div>
    );
  }

  if (!result) return null;

  const totalGreenItems = result.categories
    .flatMap((c) => c.items)
    .filter((i) => i.safety === "green").length;
  const allSelected = selectedIds.size >= totalGreenItems && totalGreenItems > 0;

  return (
    <div className="scan-view">
      <div className="scan-summary">
        <h2>
          Found{" "}
          <span className="scan-summary__bytes">
            {formatBytes(result.total_bytes)}
          </span>{" "}
          that can be safely freed
        </h2>
        <div className="scan-summary__row">
          <p className="scan-summary__categories">
            across {result.categories.length} categories
          </p>
          <div className="scan-summary__actions">
            <button
              className="btn btn--secondary btn--sm"
              onClick={selectAllGreen}
              disabled={allSelected}
            >
              Select all
            </button>
            <button
              className="btn btn--secondary btn--sm"
              onClick={clearSelection}
              disabled={selectedIds.size === 0}
            >
              Unselect all
            </button>
          </div>
        </div>
      </div>
      <div className="scan-categories">
        {result.categories.map((cat) => (
          <CategoryCard key={cat.category} category={cat} />
        ))}
      </div>
    </div>
  );
}
