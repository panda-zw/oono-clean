const sections = [
  {
    title: "Getting Started",
    steps: [
      {
        heading: "Run a scan",
        body: 'Click "Scan Now" in the top right corner. OneSweep will search common locations on your Mac for caches, build artifacts, and other reclaimable files.',
      },
      {
        heading: "Review results",
        body: "Results are grouped by category. Each item shows its name, size, and a safety badge. Green items are always safe to remove - they'll be re-downloaded or rebuilt automatically when needed.",
      },
      {
        heading: "Select and clean",
        body: 'Green items are selected by default. Uncheck anything you want to keep, then click the "Free" button at the bottom to reclaim your space.',
      },
    ],
  },
  {
    title: "What Gets Scanned",
    items: [
      {
        label: "JavaScript Dependencies",
        desc: "node_modules folders across your projects, plus npm, Yarn, and pnpm caches.",
      },
      {
        label: "Docker",
        desc: "Unused container images and build cache. Requires Docker or OrbStack to be running.",
      },
      {
        label: "Xcode",
        desc: "Simulator devices, runtimes, and DerivedData build artifacts.",
      },
      {
        label: "Gradle / Android",
        desc: "Build caches and wrapper distributions in ~/.gradle.",
      },
      {
        label: "System Caches",
        desc: "Per-app caches in ~/Library/Caches (only items over 50 MB are shown).",
      },
      {
        label: "Homebrew",
        desc: "Downloaded package files that are no longer needed after installation.",
      },
    ],
  },
  {
    title: "Safety Levels",
    items: [
      {
        label: "Green - Safe to remove",
        desc: "These items are always regenerable. Dependency caches, build artifacts, and package manager downloads. They'll be re-created automatically when needed.",
      },
      {
        label: "Yellow - Review first",
        desc: "Probably safe, but you should check. For example, node_modules in a project with uncommitted git changes will show as yellow.",
      },
      {
        label: "Red - Be careful",
        desc: "OneSweep surfaces these for awareness but won't offer one-click deletion. These may contain data you care about.",
      },
    ],
  },
  {
    title: "Tips",
    items: [
      {
        label: "Run scans regularly",
        desc: "Developer caches grow quickly. A monthly scan can easily free 10–50 GB.",
      },
      {
        label: "Check the History tab",
        desc: "Every cleanup is logged with timestamps and sizes so you can track what was removed.",
      },
      {
        label: "Don't worry about mistakes",
        desc: "Everything OneSweep removes in the green tier is automatically regenerated. Deleting node_modules just means running npm install next time you open that project.",
      },
    ],
  },
];

export function Guide() {
  return (
    <div className="guide">
      <h2>How to Use OneSweep</h2>
      <p className="guide__intro">
        OneSweep finds the real space hogs on your Mac and lets you safely
        reclaim storage without breaking anything.
      </p>

      {sections.map((section) => (
        <div key={section.title} className="guide__section">
          <h3>{section.title}</h3>

          {"steps" in section &&
            section.steps?.map((step, i) => (
              <div key={step.heading} className="guide__step">
                <span className="guide__step-num">{i + 1}</span>
                <div>
                  <strong>{step.heading}</strong>
                  <p>{step.body}</p>
                </div>
              </div>
            ))}

          {"items" in section &&
            section.items?.map((item) => (
              <div key={item.label} className="guide__item">
                <strong>{item.label}</strong>
                <p>{item.desc}</p>
              </div>
            ))}
        </div>
      ))}
    </div>
  );
}
