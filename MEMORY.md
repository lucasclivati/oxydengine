# Oxyd Engine - Long-Term Memory & Project Rules

## 🚀 Engine Identity & Versioning
- **Engine Name**: **Oxyd Engine** (Strictly clean branding. No third-party trademarks).
- **Version**: **`v0.0.1`** across all manifests (`Cargo.toml`), titles, locales, and documentation.
- **Windows Executable**: Uses `#![windows_subsystem = "windows"]` in `src/main.rs` to prevent background CMD terminal windows on launch.
- **Window Icon**: Configured with official `logo.jpg` via `winit::window::Icon`.

## 🌐 Localization System & 30 Global Languages
- **Base Language**: 100% **English (`en`)**.
- **Supported Locales (30 languages)**: `en`, `pt`, `zh`, `es`, `hi`, `ar`, `bn`, `fr`, `ru`, `ja`, `de`, `ko`, `it`, `tr`, `vi`, `pl`, `nl`, `uk`, `id`, `th`, `sv`, `cs`, `el`, `ro`, `hu`, `fi`, `da`, `no`, `he`, `ms`.
- **CRITICAL UNTRANSLATED MENU RULE**: The top menu button for language selection MUST ALWAYS remain titled **`LANGUAGE`** in plain English.
- **NEW FEATURE REQUIREMENT**: Whenever a new UI button or feature is added, translation keys MUST be added across all 30 locale JSON files in `assets/locales/`.

## 🖥️ UI & Layout Rules
- **Typography**: Google Inter Open Source Font (`assets/fonts/Inter-Regular.ttf`, SIL Open Font License).
- **Selection Mode Popup**: Contains options for Selection (Shift+1), Landscape (Shift+2), Foliage (Shift+3), Mesh Paint (Shift+4), Modeling (Shift+5), Fracture (Shift+6), Brush Editing (Shift+7), Animation (Shift+8), PCG (Shift+9).
- **Bottom Docking Bar**:
  - Connected tabs: `📁 Content Drawer`, `📋 Output Log`, `>_ Cmd`.
  - Content Drawer slides UP/DOWN when clicked or via shortcut `Ctrl + Space`.
  - Interactive console input in `Cmd` executes live scene/engine commands (`spawn`, `delete`, `play`, `stop`, `setlang`).
- **3D World Physics & Simulation**:
  - Objects are stationary by default when loading a project. They only rotate/simulate when `is_playing == true` (Play Mode).
