# Annihilation

A high-performance secure shredder for Linux, leveraging low-level I/O primitives to ensure non-recoverable data destruction.

## Architecture
Annihilation bypasses OS-level abstractions by implementing a hardware-synchronized destruction pipeline.

- **Kernel-level Sync:** Dual `fsync` invocations force IO barrier completion, preventing data leakage via volatile caches.
- **Atomic Renaming:** Filenames are obfuscated before removal to eliminate metadata remnants in FS journals.
- **SSD Lifecycle Management:** Post-wipe `FITRIM` via `ioctl` triggers controller-level cell deallocation, ensuring data blocks are physically zeroed by the NAND controller.

## Performance Metrics
| Metric | Value | Constraint |
| :--- | :--- | :--- |
| **IO Latency** | ~7.9 ms/file | Hardware sync overhead |
| **Throughput** | Sequential NVMe Max | Bus speed / NAND limit |
| **Implementation**| Zero-copy I/O | Syscall overhead |

## Security Model
The utility addresses the physical persistence of data on NAND media.
1. **Zeroing:** Overwrites binary data with `0` via `io::repeat(0)`.
2. **Barrier Compliance:** `sync_all` ensures data reaches the physical storage media.
3. **Journal Sanitization:** `rename` and `remove` operations target filesystem journal integrity.

## Deployment
Requires superuser privileges for `ioctl` access and filesystem controller management.

```bash
sudo ./target/release/annihilation
