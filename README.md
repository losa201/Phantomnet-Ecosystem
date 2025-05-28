<<<<<<< HEAD
# PhantomNet Ecosystem

This archive contains the complete infrastructure, operator, dashboard, and automation scripts for PhantomNet.
=======
# 🚀 PhantomNet

> A modular, zk-enabled tangle protocol designed for next-gen decentralized systems.

[![Version](https://img.shields.io/badge/version-1.9.0-blue.svg)](https://github.com/losa201/phantomnet-ecosystem)
[![Build](https://github.com/losa201/phantomnet-ecosystem/actions/workflows/ci.yml/badge.svg)](https://github.com/losa201/phantomnet-ecosystem/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

---

## 🧠 What is PhantomNet?

PhantomNet is a high-performance Layer-1 protocol designed around privacy, scalability, and modular zkRollup computation. It features:

- 🧩 Zero-knowledge tangle (zkTangle)
- 🧱 Stateless rollup compression
- 📡 Gossip-style mesh propagation
- 📊 Native Prometheus metrics
- 🐳 One-command Docker deploy
- ✅ CI + test coverage built-in

---

## 🏗 Architecture

```
[ zkNode ] -> [ Merkle Tree ] -> [ RollupNode ] -> [ zktangle.log ]
       ↘
       REST API <-> Axum Router <-> Metrics / API / Mesh / Dashboard
```

---

## 🧪 Quickstart with Docker

```bash
cd docker
docker-compose up --build
```

### 📍 Local Access:

- `http://localhost:3000/api/tangle`
- `http://localhost:3000/zk/rollup/create`
- `http://localhost:3000/metrics`
- `http://localhost:9090` (Prometheus UI)

---

## 🧬 API Reference

- `POST /api/tangle` – Submit zkNode
- `POST /zk/rollup/create` – Trigger zkRollup
- `GET /metrics` – Prometheus metrics output

---

## 👩‍💻 Developer Guide

### 📂 Repo Layout
```
phantomnetd/        → Core daemon
zkrollup/           → zk batch & proof logic
zktangle/           → Rollup persistence
metrics/            → Prometheus interface
docker/             → Docker + compose setup
tests/              → Integration/unit tests
```

### 🧪 Run Tests
```bash
cargo test --all
```

---

## 📜 License

MIT © Jean-Pierre Kemper — Contributions welcome!
>>>>>>> 1ca2900 (🧠 Initial refined push — 2025-05-28 13:28:07)
