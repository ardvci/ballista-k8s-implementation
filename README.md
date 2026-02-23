# 🦀 Rusty Distributed Computation: Ballista Implementation

This project is a high-performance distributed data processing reference implementation. It demonstrates how to leverage **Apache Arrow**, **DataFusion**, and **Apache Ballista** on a **Kubernetes** cluster to process massive datasets with sub-second latency.

## Performance Highlight
* **Dataset Size:** 200 Million Rows (Generated via custom Rust tool).
* **Execution Time:** ~1.21s (Aggregating, filtering, and sorting 200M records).
* **Stack:** Rust, Ballista, Arrow Flight (gRPC), Kubernetes.

---

## Architecture Overview

The project consists of three core components designed to showcase the synergy of the Rust data ecosystem:

1.  **Data Generator (`data-gen`):** A high-throughput Rust binary utilizing `arrow-rs` and `parquet` to generate 200M rows of synthetic log data, partitioned into 40 Parquet files for optimal parallel scanning.
2.  **Ballista Cluster:** A distributed scheduler and executor architecture running on Kubernetes, orchestrating query fragments across multiple pods.
3.  **Query Runner (`run`):** A remote session client that submits complex analytical queries to the Ballista cluster using the DataFusion logical plan API.

---

## Technical Deep Dive

### Why it's fast?
* **Vectorized Execution (SIMD):** DataFusion leverages Rust's memory safety and SIMD instructions to perform operations on Arrow arrays at CPU-register speeds.
* **Zero-Copy with Arrow Flight:** Data is transferred between executors using the Arrow Flight gRPC protocol, eliminating the heavy serialization/deserialization "tax" found in JVM-based engines.
* **Predicate & Projection Pushdown:** The engine prunes Parquet row groups at the scan level based on `status_code`, significantly reducing I/O overhead.

### Physical Plan Analysis
The engine breaks down the query into a multi-stage execution plan:
1.  **Partial Aggregate:** Local aggregation on each executor to minimize data shuffle.
2.  **Hash Repartitioning:** Redistributing data based on `user_id` across 16 partitions.
3.  **Final Aggregate:** Merging partial results to calculate the final `avg` and `count`.

---

## Deployment

### Prerequisites
* Kubernetes Cluster (Minikube / Kind / Colima / Rancher)
* Protobuf
* Rust 1.80+
* Helm 3

### Quick Start
Run the automated deployment pipeline:
```bash
chmod +x deploy.sh
./deploy.sh