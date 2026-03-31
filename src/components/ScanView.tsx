import { useMemo, useState } from "react";
import { useScanStore } from "../lib/stores/scan";
import { formatBytes } from "../lib/utils/format";
import type { CategoryResult, SafetyLevel } from "../lib/types";
import { CategoryCard } from "./CategoryCard";

type SafetyFilter = "all" | SafetyLevel;

export function ScanView() {
  const { result, isScanning, selectedIds, selectAllGreen, clearSelection } =
    useScanStore();
  const [search, setSearch] = useState("");
  const [safetyFilter, setSafetyFilter] = useState<SafetyFilter>("all");

  const filtered = useMemo(() => {
    if (!result) return [];
    const query = search.toLowerCase().trim();

    return result.categories
      .map((cat): CategoryResult | null => {
        let items = cat.items;

        if (safetyFilter !== "all") {
          items = items.filter((i) => i.safety === safetyFilter);
        }

        if (query) {
          items = items.filter(
            (i) =>
              i.display_name.toLowerCase().includes(query) ||
              i.path.toLowerCase().includes(query) ||
              i.description.toLowerCase().includes(query) ||
              cat.display_name.toLowerCase().includes(query),
          );
        }

        if (items.length === 0) return null;

        return {
          ...cat,
          items,
          total_bytes: items.reduce((sum, i) => sum + i.size_bytes, 0),
        };
      })
      .filter((c): c is CategoryResult => c !== null);
  }, [result, search, safetyFilter]);

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
  const allSelected =
    selectedIds.size >= totalGreenItems && totalGreenItems > 0;

  const isFiltering = search || safetyFilter !== "all";

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

      <div className="scan-toolbar">
        <input
          type="text"
          className="scan-toolbar__search"
          placeholder="Search items..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
        <div className="scan-toolbar__filters">
          {(["all", "green", "yellow", "red"] as const).map((level) => (
            <button
              key={level}
              className={`scan-toolbar__filter ${safetyFilter === level ? "active" : ""} ${level !== "all" ? `scan-toolbar__filter--${level}` : ""}`}
              onClick={() => setSafetyFilter(level)}
            >
              {level === "all" ? "All" : level.charAt(0).toUpperCase() + level.slice(1)}
            </button>
          ))}
        </div>
      </div>

      <div className="scan-categories">
        {filtered.length === 0 ? (
          <p className="scan-view__no-results">
            No items match your {search ? "search" : "filter"}.
          </p>
        ) : (
          filtered.map((cat) => (
            <CategoryCard key={cat.category} category={cat} />
          ))
        )}
        {isFiltering && filtered.length > 0 && (
          <p className="scan-view__filter-note">
            Showing {filtered.reduce((s, c) => s + c.items.length, 0)} items in{" "}
            {filtered.length} categories
          </p>
        )}
      </div>
    </div>
  );
}
