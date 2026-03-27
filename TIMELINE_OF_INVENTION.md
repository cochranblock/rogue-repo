<!-- Copyright (c) 2026 The Cochran Block, LLC (Pending). All rights reserved. -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why.*

> Every entry maps to real commits. Run `git log --oneline` to verify.

---

## Entries

### 2026-03-22 — CODEOWNERS + Governance

**What:** Added CODEOWNERS and OWNERS.yaml. Repository governance structure.
**Commit:** `67f35a4`
**AI Role:** AI generated governance files. Human decided ownership model.

### 2026-03-20 — Rogue Runner: Full Game Pipeline

**What:** Complete game: HTML canvas (1000 procedural levels, zone-aware generation), WASM cross-platform build, asset pipeline, authenticated binary downloads.
**Why:** An app store needs at least one real app. Rogue Runner proves the entire distribution pipeline works.
**Commit:** `f7f80ba`
**AI Role:** AI generated game engine and level generation. Human designed game mechanics, zone system, and delivery architecture.

### 2026-03-18 — Foundational Founders v0.2.0

**What:** Version bump, contributor attribution locked, copyright headers unified under The Cochran Block, LLC.
**Commit:** `5c5034a`
**AI Role:** AI applied headers. Human decided licensing model.

### 2026-03-16 — Railway Deployment + Docker

**What:** Dockerfile and railway.toml for deployment.
**Commit:** `2ba2c2a`
**AI Role:** AI wrote deployment configs. Human specified runtime requirements.

### 2026-03-14 — Initial Architecture

**What:** Full sovereign app store: ISO 8583 payment engine (bitvec bitmask, MTI 0200), AES-256-GCM PAN vault, PostgreSQL ledger with ACID transactions, Rogue Bucks economy, Argon2 auth, PWA shell.
**Why:** Build an app store that doesn't depend on Apple, Google, or any payment processor. Sovereign from payment rail to binary delivery.
**AI Role:** AI generated implementation across both crates (rogue-repo modules: vault, switch, ledger, auth, pwa, routes, downloads; rogue-runner). Human architected the payment flow, security model, and economy design.

---

*Part of the [CochranBlock](https://cochranblock.org) architecture.*
