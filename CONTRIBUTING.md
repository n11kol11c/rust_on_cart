# Contributing to rust-on-cart

💛 Thank you for your interest in contributing!  
We welcome contributions of all kinds — code, documentation, tests, issue triage, and feedback.

This document explains how to get started, the project workflow, and contribution guidelines.

---

## 🧱 Project Goals

`rust-on-cart` is designed to be:

- **Simple** — easy to understand and use.
- **Performant** — leveraging Rust’s zero-cost abstractions.
- **Modular** — components should be composable and reusable.
- **Safe** — correctness and maintainability come first.

Keep these principles in mind when proposing or implementing changes.

---

## 🏗 Project Setup

### Prerequisites

- Rust (stable) → Install via https://rustup.rs
- `cargo` (comes with Rust)
- `just` (optional, if the project uses `justfile`)
- A recent Linux/macOS environment preferred (Windows supported)

### Clone and Build

```bash
git clone https://github.com/<your-org>/rust-on-cart.git
cd rust-on-cart
cargo build
