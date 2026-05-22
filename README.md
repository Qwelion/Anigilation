# Annihilation

A secure file shredder for Linux, designed to minimize data recovery chances by forcing OS cache flushes and triggering controller-level cleanup.

## 𓃮 Architecture
Annihilation interacts directly with Linux I/O primitives to ensure data is pushed from the OS cache to the physical storage media.

- **Kernel-level Sync:** Dual `fsync` invocations force IO barrier completion ◼️
- **Atomic Renaming:** Filenames are obfuscated before removal to prevent metadata recovery ◾
- **SSD Lifecycle Management:** Post-wipe `FITRIM` triggers controller-level deallocation ◼️

## ⚠️ Important Note on SSDs
**Please read before use:** Because modern SSDs use a **Flash Translation Layer (FTL)** and **wear-leveling algorithms**, 100% physical overwrite cannot be guaranteed from user-space.

* **Free space matters:** If your drive has plenty of free space, the controller may prefer writing to new empty blocks to save wear, potentially leaving the original physical data intact.
* **Full drives:** If the drive is nearly full, the controller is forced to reuse existing blocks, which significantly increases the chance of a physical overwrite.
* **Our approach:** This tool provides the highest level of sanitization available at the filesystem level by forcing cache flushes and issuing `fstrim`.

## 🗡 Security Model
1. **Zeroing:** Overwrites binary data with `0` via `io::repeat(0)` ⬛
2. **Barrier Compliance:** `sync_all` ensures data reaches physical media ◾
3. **Journal Sanitization:** `rename` and `remove` operations to clear file system metadata ⬛
4. **Hardware Trim:** Executes `fstrim` to hint the controller for garbage collection ◼️

## ✘ Performance Metrics
| Metric | Value | Constraint |
| :--- | :--- | :--- |
| **IO Latency** | ~7.9 ms/file | Hardware sync overhead |
| **Throughput** | Limited by Drive | Bus speed / NAND limit |
| **Implementation**| Zero-copy I/O | Syscall overhead |

## ᯓ★ Deployment
```bash
sudo ./target/release/annihilation
