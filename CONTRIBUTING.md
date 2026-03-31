# Contributing to OneSweep

Thanks for your interest in contributing! Here's how to get started.

## Development Setup

1. Fork the repo and clone your fork
2. Install prerequisites: Rust (1.77.2+), Node.js (20+), pnpm
3. Run `pnpm install` to install frontend dependencies
4. Run `pnpm tauri dev` to start the app in development mode

## Making Changes

1. Create a branch from `main` for your changes
2. Make your changes, keeping commits focused and descriptive
3. Ensure the project compiles:
   - Frontend: `npx tsc --noEmit`
   - Backend: `cd src-tauri && cargo check`
4. Test your changes by running the app with `pnpm tauri dev`

## Pull Requests

- Keep PRs focused on a single change
- Write a clear description of what your PR does and why
- Include screenshots for UI changes
- Make sure the project builds without errors

## Adding a Scanner Recipe

Scanner recipes live in `src-tauri/src/scanner/recipes/`. To add a new one:

1. Create a new file in `src-tauri/src/scanner/recipes/` (e.g., `ruby.rs`)
2. Implement an async function that returns `Result<Vec<ScanItem>>`
3. Register it in `src-tauri/src/scanner/recipes/mod.rs`
4. Add it to the `JoinSet` in `src-tauri/src/scanner/mod.rs`
5. If it needs a custom cleanup executor, add one in `src-tauri/src/cleanup/executors/`

## Adding a Cleanup Executor

Cleanup executors live in `src-tauri/src/cleanup/executors/`. Each one handles deletion for a specific category (file deletion, Docker CLI, etc.).

## Code Style

- **Rust**: Follow standard Rust conventions. `cargo check` should pass cleanly.
- **TypeScript/React**: Follow existing patterns. `tsc --noEmit` should pass cleanly.
- Don't add unnecessary dependencies.

## Reporting Issues

- Use GitHub Issues to report bugs or suggest features
- Include your macOS version and steps to reproduce for bugs
- Check existing issues before creating a new one

## Code of Conduct

Be respectful and constructive. We're all here to build something useful.
