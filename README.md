# OnePurge

A macOS desktop app that finds the real disk space hogs on your Mac and lets you safely reclaim storage without breaking anything.

![Built with Tauri](https://img.shields.io/badge/Built_with-Tauri-blue?logo=tauri)
![React](https://img.shields.io/badge/React-19-blue?logo=react)
![Rust](https://img.shields.io/badge/Rust-Backend-orange?logo=rust)
![License: MIT](https://img.shields.io/badge/License-MIT-green)

## The Problem

macOS users, especially developers, lose hundreds of gigabytes to invisible clutter: simulator runtimes, Docker images, dependency caches, old IDE data, and build artifacts. Apple's built-in storage breakdown is vague ("System Data: 231 GB") and offers no way to act on it.

OnePurge finds what's actually taking up space, explains it in plain language, and lets you clean it up safely.

## What It Scans

### Developer Tools

| Category | Examples | Safety |
|----------|----------|--------|
| JavaScript | `node_modules`, npm/yarn/pnpm caches | Green |
| Rust | `target/` build artifacts, Cargo registry & git cache | Green |
| Python | pip cache, conda packages, virtual environments (venv/.venv/.tox) | Green |
| Go | Module cache, build cache | Green |
| Java / Maven | `~/.m2/repository` | Green |
| Gradle / Android | Build caches, wrapper distributions | Green |
| Ruby | Gem cache, Bundler cache | Green |
| .NET | NuGet package cache, .NET SDK artifacts | Green |
| Flutter / Dart | Pub cache, Flutter tool artifacts | Green |
| CocoaPods | Pod specs and sources | Green |
| PHP / Composer | Composer download cache | Green |
| Docker | Unused images, build cache | Green |

### Xcode / Apple Development

| Category | Examples | Safety |
|----------|----------|--------|
| Simulators | iOS/watchOS/tvOS simulator devices and runtimes | Green |
| DerivedData | Xcode build artifacts per project | Green |
| Device Support | Debug symbols for connected iOS devices | Green |
| Archives | Archived app builds | Yellow |

### macOS System Data

| Category | Examples | Safety |
|----------|----------|--------|
| Browser Caches | Chrome, Safari, Firefox, Arc, Brave, Edge, Opera, Vivaldi | Green |
| System Caches | Per-app caches in ~/Library/Caches (50 MB+) | Green |
| System Logs | Application and diagnostic logs | Green |
| Homebrew | Downloaded package files | Green |
| IDE Caches | VS Code, Cursor, JetBrains workspace storage | Green |
| iOS Backups | Local iPhone/iPad backups | Yellow |
| Trash | Files deleted but not emptied | Yellow |
| Old Downloads | Files in Downloads older than 90 days | Yellow |
| Time Machine | Local Time Machine snapshots | Yellow |

**Safety levels:**
- **Green** -- Safe to remove. Always regenerable. Auto-selected for cleanup.
- **Yellow** -- Review first. Probably safe but check before removing.
- **Red** -- Be careful. Surfaced for awareness only.

## Features

- **29 scan categories** across dev tools, Apple development, and macOS system data
- **Three-tier safety system** with git-status awareness (node_modules in projects with uncommitted changes get downgraded to Yellow)
- **Search and filter** scan results by name, path, or safety level
- **Select all / unselect all** controls
- **Dark and light theme** (shadcn zinc palette)
- **Cleanup audit log** with full history of what was removed
- **Built-in guide** explaining what each category is and why it's safe

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
git clone https://github.com/panda-zw/one-purge.git
cd one-purge

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
  scanner/        # Parallel recipe-based scanning engine (29 recipes)
  classifier/     # Safety classification with git-status awareness
  cleanup/        # Cleanup executors (file delete, Docker CLI, xcrun, brew, macOS)
  db/             # SQLite database layer (audit log + scan cache)
  commands/       # Tauri IPC command handlers
  models/         # Shared domain models

src/
  components/     # React UI components
  lib/stores/     # Zustand state management
  lib/api.ts      # Tauri invoke wrappers
  lib/types.ts    # TypeScript type definitions
```

## Security

- Restrictive Content Security Policy
- Path validation with allowlist before any file deletion
- Symlink resolution to prevent path traversal attacks
- All database paths re-verified at cleanup time
- No telemetry, no cloud, fully offline

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License -- see the [LICENSE](LICENSE) file for details.
