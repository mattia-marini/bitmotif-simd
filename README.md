# bitmotif-simd

High-performance code for the representation and manipulation of small (order 2 to 5), higher-order motifs using bitwise operations and SIMD-accelerated primitives.

This repository contains the core heuristics and data structures utilized in [higher-order-motifs](https://github.com/mattia-marini/higher-order-motifs). By representing motifs and node sets as compressed bitwise integers, the implementation achieves high efficiency in isomorphism classification and motif enumeration.

---

## Core Concepts

To maximize performance, the code avoids generic abstractions. Instead, it employs type-specific, optimized implementations tailored to individual motif orders.

### Primary Data Structures

| Structure | Description |
| :--- | :--- |
| `CompressedMotifX` | Represents a motif as an edge set stored within a single unsigned integer. |
| `CompressedNodeSet` | Represents a collection of nodes as a single unsigned integer bitmask. |
| `FingerprintX` | Computes a canonical invariant fingerprint, facilitating efficient isomorphism class grouping. |

---

## Key Functionalities

### 1. Compile-Time Optimization
**`generate_bitmaskX`**
Required bitmask structures are generated at compile time. This approach replaces runtime branching with specialized logic, reducing instruction count during critical loops.

### 2. High-Speed Motif Enumeration
**`CompressedMotifX::enum_labelings`**
Algorithms for generating connected motifs with hyperedge sizes in the range [2, X]. The compressed representation eliminates heap allocations and pointer chasing, maintaining the working set within CPU registers and cache lines.

---

## Technical Approach

The library is optimized for performance-critical path analysis:

*   **Bitwise Encoding:** Motif topology is packed into primitive types to leverage native CPU instruction sets.
*   **SIMD and Bit Manipulation:** The implementation utilizes `Popcount`, `CTZ` (Count Trailing Zeros), and related bitwise operations to process bitsets efficiently.
*   **Heuristic Fingerprinting:** The fingerprinting strategy employs isomorphism invariants valid for constrained motif orders (typically < 5).
