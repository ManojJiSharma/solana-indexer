# Solana Indexer (Rust)

A **real-time Solana transaction indexer** built in Rust that streams blockchain data via WebSockets, processes it asynchronously with backpressure, persists it efficiently in Postgres, and exposes indexed data through a clean REST API using Axum.

This project focuses on **correct async architecture, performance, and production-ready design**.

---

## ✨ Features

- 🔌 **Real-time Solana ingestion**
  - WebSocket `logsSubscribe`
  - Finalized transaction data
- 🔁 **Async processing pipeline**
  - `tokio::mpsc` channels
  - Backpressure-aware design
- 📦 **Batching for high throughput**
  - Size-based & time-based flushing
  - `tokio::select!`
- 🗄️ **Postgres persistence**
  - SQLx + migrations
  - Batched inserts
  - Conflict-safe writes
- 🌐 **REST API (Axum)**
  - `/health`
  - `/tx/{signature}`
  - `/slot/{slot}/txs?limit=N`
- 📊 **Structured logging**
  - `tracing` for observability

---

## 🚀 Getting Started

Follow these steps to run the Solana Indexer locally.

---

### 1️⃣ Prerequisites

Make sure you have the following installed:

- **Rust** (stable)
- **PostgreSQL**
- **Solana RPC access** (default: devnet)

---

### 2️⃣ Clone the Repository

```bash
git clone <your-repo-url>
cd solana-indexer


3️⃣ Create a .env file in the project root:

DATABASE_URL=postgres://user:password@localhost:5432/solana_indexer

4️⃣ Install SQLx CLI
cargo install sqlx-cli --no-default-features --features postgres

5️⃣ Run Database Migrations
sqlx migrate run

6️⃣ Run the Application
cargo run