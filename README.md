# 🧠 Habit Tracker Core

- This is the core logic layer for a minimalist, always-on-top habit tracker widget. It reads structured markdown journal files and extracts habit metrics to compute streaks, monthly heatmaps, and more — all in Rust

## ✨ Features

- Parses journal files with frontmatter in YAML format
- Extracts and aggregates habit data
- Supports aggregation by day, week, month
- SQLite caching layer for performance
- File-watching to respond to journal updates
- integrated later with a GTK desktop GUI

## 📊 Project Overview

**Goal**: Transform the journal parser into a fully functional desktop widget with real-time habit tracking and visualization.

**Current Status**: ✅ Core foundation complete (parsing, database, basic UI)
**Progress**: 25% complete

---

## 🎯 Core Development Tasks

### 1. **Connect UI to Database**

- [ ] Modify `ui_main()` to initialize and pass database connection
- [ ] Use `db::queries::get_metrics_by_date()` in UI components
- [ ] Replace mock data with real journal metrics
- [ ] Add error handling for database failures
- **Expected Outcome**: Widget displays live journal data

### 2. **Implement Real-time Updates**

- [ ] Add file system watcher for journal directory
- [ ] Update `current_week.rs` to show actual completion status (✅/❌)
- [ ] Create database change notifications
- [ ] Auto-refresh UI when files are modified
- **Expected Outcome**: Widget updates automatically without manual refresh

### 3. **Build Metric Display System**

- [ ] Create components for specific metrics
- [ ] Add metric icons and color coding
- [ ] Implement streak counters and statistics
- [ ] Show weekly completion percentages
- **Expected Outcome**: Clear visual representation of each tracked habit

### 4. **Create Monthly Heatmap**

- [ ] Build calendar-style grid component
- [ ] Color-code days based on habit completion density
- [ ] Add hover tooltips with daily details
- [ ] Implement month navigation
- **Expected Outcome**: GitHub-style contribution heatmap for habits

### 5. **Add Interactivity & Configuration**

- [ ] Implement click handlers for viewing daily details
- [ ] Add file picker for choosing journal directory
- [ ] Create settings panel for appearance and behavior
- [ ] Build UI for adding/removing tracked metrics
- **Expected Outcome**: Fully interactive and customizable widget

### 6. **Polish & Distribution**

- [ ] Implement always-on-top window behavior
- [ ] Add dark/light mode themes with CSS
- [ ] Optimize database queries and UI performance
- [ ] Create build system for distributing desktop widget
- **Expected Outcome**: Production-ready desktop application
