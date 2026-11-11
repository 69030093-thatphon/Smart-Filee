# Smart-Filee — Workspace Structure Specification

**Workspace Root:** `Smart-file-rust/`  
**Configuration:** Cargo Virtual Workspace  
**Rust Edition:** 2021  

---

## 1. Cargo.toml Workspace Specification

The Cargo workspace utilizes workspace-level dependency inheritance (`[workspace.dependencies]`) to ensure consistent library versions, prevent dependency drift, and streamline compilation across all crates.

```toml
[workspace]
resolver = "2"
members = [
    "crates/common",
    "crates/localization",
    "crates/database",
    "crates/file_engine",
    "crates/organizer",
    "crates/duplicate",
    "crates/image_engine",
    "crates/disk_analyzer",
    "crates/cleanup",
    "crates/search",
    "crates/renamer",
    "crates/backup",
    "crates/system_monitor",
    "crates/network",
    "crates/auth",
    "crates/core",
    "crates/ui",
    "crates/app",
]

[workspace.package]
version = "4.3.1"
edition = "2021"
authors = ["Smart-Filee Team", "xmxmxxx <69030093@kmitl.ac.th>"]
license = "MIT"
repository = "https://github.com/69030093-thatphon/Smart-Filee.git"

[workspace.dependencies]
# Serialization & Utilities
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
regex = "1.10"
glob = "0.3"

# Concurrency & Async
tokio = { version = "1.40", features = ["full"] }
rayon = "1.10"
crossbeam-channel = "0.5"

# Database
rusqlite = { version = "0.32", features = ["bundled", "backup", "chrono"] }

# Networking & Cloud Auth
reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }
dotenvy = "0.15"

# Media & Hashing
sha2 = "0.10"
image = { version = "0.25", default-features = false, features = ["png", "jpeg", "webp", "bmp", "tiff"] }
kamadak-exif = "0.5"

# System & OS Integration
sysinfo = "0.31"
trash = "5.1"
notify = "6.1"

# Desktop UI
eframe = { version = "0.28", default-features = false, features = ["default_fonts", "glow", "persistence"] }
egui = "0.28"
egui_extras = { version = "0.28", features = ["image"] }
```

---

## 2. Crate Boundary Rules

1. **`crates/common`**: Foundational types (`PathBuf`, error types, file size formatters, duration formatters, cancellation tokens). No dependencies on higher-level crates.
2. **`crates/localization`**: Strings and bilingual translations. Can depend on `crates/common`.
3. **`crates/database`**: SQLite connection, migrations, undo batch records, hash cache, scan logs. Depends on `crates/common`, `rusqlite`.
4. **`crates/file_engine`**: Filesystem safety guards, traversal, collision detection, atomic file moving, undo execution. Depends on `crates/common`, `crates/database`, `trash`.
5. **Domain Crates (`organizer`, `duplicate`, `image_engine`, `disk_analyzer`, `cleanup`, `search`, `renamer`, `backup`)**:
   - Each domain crate implements pure algorithms testable in isolation.
   - Depends on `crates/file_engine`, `crates/common`.
   - Zero UI dependencies.
6. **`crates/system_monitor` & `crates/network`**:
   - OS-level inspection and diagnostics.
   - Depends on `crates/common`, `sysinfo`.
7. **`crates/auth`**:
   - Supabase Auth REST calls, local loopback listener on port 8765.
   - Depends on `crates/common`, `reqwest`, `tokio`.
8. **`crates/core`**:
   - High-level orchestration, tidy score algorithm, health summary coordinator.
   - Aggregates domain engines and coordinates tasks.
9. **`crates/ui`**:
   - Presentation layer built on `egui`/`eframe`.
   - Theme styling, custom widgets (`Ring`, `BarList`, `Toast`), 14 view pages, auth modal dialogs.
   - Communicates with `crates/core` via message passing.
10. **`crates/app`**:
    - Application binary entry point (`main.rs`).
    - Parses command-line arguments (including headless `--selftest`).
    - Configures logging and launches `eframe::run_native`.
