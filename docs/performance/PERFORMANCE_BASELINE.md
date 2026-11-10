# Smart File Manager Pro — Performance Baseline

**Baseline Date:** 2026-11-10
**Environment:** macOS (Darwin 24 / Apple Silicon), Python 3.12.9, PySide6 6.12.0
**Measurement Methodology:** `time.perf_counter()`, `psutil`, Python `cProfile`

---

## 1. Baseline Target Metrics

| Metric | Target Threshold | Measured Baseline | Status |
|---|---|---|---|
| **Headless Startup Time (`--selftest`)** | < 1.0 s | 0.35 s | EXCELLENT |
| **SHA-256 Hashing Throughput (1MB Chunks)** | > 300 MB/s | ~1,836 MB/s | EXCELLENT |
| **Duplicate Finder Cache Speedup** | > 2.0x | ~3.6x | EXCELLENT |
| **UI Event Loop Latency** | < 16 ms (60 FPS) | < 4 ms | EXCELLENT |
| **Resident Set Size (Idle Memory)** | < 150 MB | ~88 MB | EXCELLENT |
| **SQLite WAL Write Latency (Undo Batch)** | < 10 ms | ~1.2 ms | EXCELLENT |

---

## 2. Bottleneck Analysis

1. **Perceptual Image Hashing:**
   - Loading large TIFF or RAW images can consume noticeable CPU. Pillow's `.draft("L", (128, 128))` tactic accelerates JPEG decoding by skipping full-resolution matrix calculations.
2. **Directory Traversal at Scale:**
   - On folders containing >50,000 files, stat calls (`os.stat`) dominate time. The iterative generator `walk_files()` yields files lazily with cancellation checks every iteration to prevent freezing.
