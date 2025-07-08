# 🧠 Habit Tracker Core

- This is the core logic layer for a minimalist, always-on-top habit tracker widget. It reads structured markdown journal files and extracts habit metrics to compute streaks, monthly heatmaps, and more — all in Rust

## ✨ Features

- Parses journal files with frontmatter in YAML format
- Extracts and aggregates habit data
- Supports aggregation by day, week, month
- SQLite caching layer for performance
- File-watching to respond to journal updates
- integrated with a GTK desktop GUI

## 📊 Project Overview

**Goal**: Transform the journal parser into a fully functional desktop widget with real-time habit tracking and visualization.

**Current Status**: ✅ Core foundation complete (parsing, database, basic UI)
