# OneSweep

A macOS desktop app that finds the real disk space hogs on your Mac and lets you safely reclaim storage without breaking anything.

![Built with Tauri](https://img.shields.io/badge/Built_with-Tauri-blue?logo=tauri)
![React](https://img.shields.io/badge/React-19-blue?logo=react)
![Rust](https://img.shields.io/badge/Rust-Backend-orange?logo=rust)
![License: MIT](https://img.shields.io/badge/License-MIT-green)

## The Problem

macOS users, especially developers, lose hundreds of gigabytes to invisible clutter: simulator runtimes, Docker images, dependency caches, old IDE data, and build artifacts. Apple's built-in storage breakdown is vague ("System Data: 231 GB") and offers no way to act on it.

OneSweep finds what's actually taking up space, explains it in plain language, and lets you clean it up safely.

## What It Scans

| Category | Examples | Safety |
|----------|----------|--------|
| JavaScript Dependencies | `node_modules`, npm/yarn/pnpm caches | Green |
| Docker | Unused images, build cache | Green |
| Xcode | Simulators, DerivedData | Green |
| Gradle / Android | Build caches, wrapper distributions | Green |
| System Caches | Per-app caches in ~/Library/Caches | Green |
| Homebrew | Downloaded package files | Green |

**Safety levels:**
- **Green** -- Safe to remove. Always regenerable.
- **Yellow** -- Review first. Probably safe but check (e.g., project has uncommitted git changes).
- **Red** -- Be careful. Surfaced for awareness only.

## Screenshots

_Coming soon_

## Getting Started

### Prerequisites

- macOS
- [Rust](https://www.rust-lang.org/tools/install) (1.77.2+)
- [Node.js](https://nodejs.org/) (20+)
- [pnpm](https://pnpm.io/installation)

### Development

```bash
# Clone the repo
git clone https://github.com/panda-zw/oono-clean.git
cd oono-clean

# Install frontend dependencies
pnpm install

# Run in development mode
pnpm tauri dev
```

### Building

```bash
# Create a production build
pnpm tauri build
```

The built app will be in `src-tauri/target/release/bundle/`.

## Tech Stack

- **Tauri v2** -- Lightweight native app shell (Rust backend + web frontend)
- **React 19** -- Frontend UI with TypeScript
- **Zustand** -- State management
- **rusqlite** -- SQLite for audit log and scan cache
- **tokio** -- Async runtime for parallel scanning

## Architecture

```
src-tauri/src/
  scanner/        # Parallel recipe-based scanning engine
  classifier/     # Safety classification with git-status awareness
  cleanup/        # Cleanup executors (file delete, Docker CLI, xcrun, brew)
  db/             # SQLite database layer (audit log + scan cache)
  commands/       # Tauri IPC command handlers
  models/         # Shared domain models

src/
  components/     # React UI components
  lib/stores/     # Zustand state management
  lib/api.ts      # Tauri invoke wrappers
  lib/types.ts    # TypeScript type definitions
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License -- see the [LICENSE](LICENSE) file for details.
