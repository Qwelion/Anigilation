# Annihilation

**Annihilation** is a high-performance Linux utility for secure data destruction. It ensures physical data erasure by enforcing hardware-level synchronization, rendering recovery impossible.

## Performance
Designed for maximum hardware throughput.

| Metric | Result (100 files, 1KB each) |
| :--- | :--- |
| **Avg Latency** | 7.9 ms per file |
| **Throughput** | Hardware-limited (NVMe peak) |

## Tech Stack
* **Language:** Rust
* **Sync Mechanism:** Dual `fsync` call per file
* **Optimization:** Zero-copy truncation & hardware `fstrim`

## Installation
```bash
cargo build --release
