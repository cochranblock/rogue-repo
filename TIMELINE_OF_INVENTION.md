<!-- Copyright (c) 2026 The Cochran Block, LLC (Pending). All rights reserved. -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why.*

> Every entry maps to real commits. Run `git log --oneline` to verify.

---

## Entries

### 2026-04-02 — Security Hardening + Honest README

**What:** SESSION_SECRET enforcement via OnceLock (panics in release if not set). Auth gating on mutation endpoints (f126: 401/403). Login/Logout nav toggle. Coming Soon card styling. README rewritten with honest "What Works" vs "What Doesn't Work Yet" tables. .env.example updated.
**Why:** Guest analysis found hardcoded session secret fallback (auth bypass), no authorization on POST endpoints (IDOR), and README that overstated functionality.
**Commit:** `9c7cb7c`
**AI Role:** AI performed guest analysis and implemented fixes. Human directed priorities and approved plan.

### 2026-04-02 — Stripe↔ISO 8583 Stubs + WebP Icons + Pixel Forge Dependency

**What:** Stripe-to-ISO 8583 translation layer staged as stubs (f120-f123): full mapping table (Stripe events → ISO MTIs, decline codes → response codes), function signatures, no implementation. App icons compressed from 57MB PNGs to 85KB WebPs (192x192). Pixel Forge dependency notice added to PWA and README with links to [github.com/cochranblock/pixel-forge](https://github.com/cochranblock/pixel-forge). All Coming Soon items link to their blocking dependency.
**Why:** Stage the payment bridge architecture without shipping it. Fix page load times. Make the game dependency chain visible to visitors.
**Commit:** `ac07fae`
**AI Role:** AI implemented stubs, compression, and dependency notices. Human specified "stubs only, not functional" and Pixel Forge dependency pattern.

### 2026-03-30 — Zero-Downtime Hot Reload

**What:** SO_REUSEPORT + PID lockfile pattern. New binary binds port overlapping with old instance, registers with approuter, writes PID, then SIGTERM/SIGKILL old process. Zero-downtime deploy: copy binary, run it, old one dies.
**Why:** Production deployment without interrupting active connections.
**Commit:** `46c93f8`
**AI Role:** AI implemented SO_REUSEPORT binding and PID lifecycle. Human specified the deploy pattern.

### 2026-03-27 — ISO 8583 Expansion + Tokenization + Release Profile

**What:** Added MTI 0100 (authorization request), 0210 (response parse), 0400 (reversal) to ISO 8583 engine. P13 tokenization applied (t33, f20, c10, t34, t35). Release profile for smallest binary (opt-level='z', LTO, strip). Economy routes wired to real ledger, vault, and ISO 8583 switch.
**Why:** A payment engine needs more than purchase requests. Auth holds, response parsing, and reversals are table stakes for ISO 8583.
**Commits:** `3adc034`, `1edefd6`, `c4410a4`
**AI Role:** AI implemented all 3 new MTIs, tokenization, and release optimization. Human specified ISO 8583 message type requirements and economy route wiring.

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
