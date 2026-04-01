import { useScanStore } from "../lib/stores/scan";

export function EmptyState() {
  const { startScan } = useScanStore();

  return (
    <div className="empty-state">
      <h2>Let's find out what's taking up your space</h2>
      <p>
        OnePurge scans for developer caches, build artifacts, and other
        reclaimable files on your Mac.
      </p>
      <button className="btn btn--primary btn--large" onClick={startScan}>
        Start Scan
      </button>
    </div>
  );
}
