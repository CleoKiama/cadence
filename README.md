# Cadence

A minimal desktop habit tracker built with Tauri (Rust backend + React frontend). It reads Markdown journal files with YAML front matter to extract numeric metrics, caches them in SQLite, and renders dashboards with streaks and weekly analytics.

## Features

- Parses metrics from journal front matter (non-recursive directory watch)
- Current/longest streaks, weekly averages, monthly totals
- Recent activity and weekly activity analytics
- SQLite caching for fast queries; automatic resync on file changes

## Quick Start

- Requirements: Node 18+, pnpm, Rust toolchain, Tauri OS dependencies
- Install: `pnpm install`
- Run desktop app: `pnpm tauri dev`
- Build desktop app: `pnpm tauri build`

## First-Time Configuration

- Open Settings and select your Journal Path (a single folder is watched, non-recursive).
- Add the metric names you want to track (must match keys in the front matter exactly).
- Edit your journal files; changes are ingested automatically.

## Journal File Format

- File name: `YYYY-MM-DD.md` (example: `2025-10-20.md`)
- Front matter must be delimited by `---` lines. Metrics are simple key: value pairs, one per line:

```
---
pages_read: 12
dsa_solved: 3
---
```

Only metrics you add in Settings are ingested. Files outside the front matter or with different names are ignored.

