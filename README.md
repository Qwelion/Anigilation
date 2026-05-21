# Annihilation

A high-performance secure shredder for Linux, leveraging low-level I/O primitives to ensure non-recoverable data destruction.

## 𓃮 Architecture
Annihilation bypasses OS-level abstractions by implementing a hardware-synchronized destruction pipeline.

- **Kernel-level Sync:** Dual `fsync` invocations force IO barrier completion ◼️
- **Atomic Renaming:** Filenames are obfuscated before removal ◾
- **SSD Lifecycle Management:** Post-wipe `FITRIM` triggers controller-level deallocation ◼️

## ✘ Performance Metrics
| Metric | Value | Constraint |
| :--- | :--- | :--- |
| **IO Latency** | ~7.9 ms/file | Hardware sync overhead |
| **Throughput** | Sequential NVMe Max | Bus speed / NAND limit |
| **Implementation**| Zero-copy I/O | Syscall overhead |

## 🗡 Security Model
1. **Zeroing:** Overwrites binary data with `0` via `io::repeat(0)` ⬛
2. **Barrier Compliance:** `sync_all` ensures data reaches physical media ◾
3. **Journal Sanitization:** `rename` and `remove` operations ⬛
4. **Hardware Trim:** Executes `fstrim` for controller-level GC ◼️

## ᯓ★ Deployment
```bash
sudo ./target/release/annihilation
