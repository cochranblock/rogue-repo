<!-- Copyright (c) 2026 The Cochran Block, LLC (Pending). All rights reserved. -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why.*

> Every entry maps to real commits. Run `git log --oneline` to verify.

---

## Human Revelations — Invented Techniques

*Novel ideas that came from human insight, not AI suggestion. These are original contributions to the field.*

### PID Relay Binary Self-Replacement — Gemini Man Pattern (March 2026)

**Invention:** Zero-downtime binary deployment where the new binary binds to the port (SO_REUSEPORT) while the old binary is still running, registers with approuter, writes its PID to the lockfile, then sends SIGTERM to the old process. If the old process doesn't die within 5 seconds, SIGKILL. Zero dropped connections.

**The Problem:** Deploying a new binary version requires stopping the old one first, causing downtime. Even "graceful" shutdowns drop in-flight requests. Blue-green deployment solves this but requires two servers or a load balancer. For a solo developer running one server, there's no clean deploy story.

**The Insight:** The movie "Gemini Man" — Will Smith fights a younger clone of himself. The clone doesn't wait for the original to die; it takes over immediately. Apply to binary deployment: the new binary starts, takes over the port, and kills the old one. The old binary is the "original" being replaced by its own updated clone.

**The Technique:**
1. New binary starts, binds to port with SO_REUSEPORT (both old and new can listen simultaneously)
2. New binary registers with approuter (traffic starts routing to new binary)
3. New binary writes its PID to lockfile, overwriting old PID
4. New binary sends SIGTERM to old PID
5. After 5s grace period, SIGKILL if old process still alive
6. Result: zero gap in port binding, zero dropped requests during transition

**Result:** Deploy = copy new binary + run it. Old binary dies automatically. No orchestrator, no load balancer, no container restart. Works on bare metal with one server.

**Named:** Gemini Man Pattern
**Commit:** `46c93f8`
**Origin:** Watching "Gemini Man" (2019) and realizing the clone-replaces-original plot is the exact deploy pattern needed for single-server zero-downtime. Named by Michael Cochran.

### ISO 8583 Message Builder in Rust (March 2026)

**Invention:** A from-scratch ISO 8583 financial message builder using Rust's bitvec crate for bitmap construction — MTI 0100 (authorization), 0200 (purchase), 0210 (response parse), 0400 (reversal) — with AES-256-GCM encrypted PAN vault and type-safe field definitions.

**The Problem:** ISO 8583 is the standard for card payment messages between merchants and banks. Every implementation is in Java or C. Rust implementations don't exist in the open source ecosystem. The spec is behind a paywall and the bitmap encoding is error-prone (a 128-bit primary+secondary bitmap where each bit indicates which data element is present).

**The Insight:** Rust's bitvec crate makes ISO 8583 bitmap construction type-safe and readable. Each data element maps to a bit position. Building a message = setting bits and appending field data. The type system prevents common errors (wrong field length, missing required fields, bitmap/data mismatch).

**The Technique:**
1. `switch/iso8583.rs`: bitvec-based bitmap construction for primary (64-bit) and secondary (128-bit) bitmaps
2. MTI construction: 0100, 0200, 0210, 0400 with correct field sets per spec
3. PAN vault: AES-256-GCM encryption of card numbers, stored in sled
4. Ledger: ACID transactions via SQLite for payment recording
5. Rogue Bucks economy: internal currency using the same ISO 8583 rails

**Result:** A complete payment message builder in Rust — no Java, no C, no external payment processor. The app store has its own payment rail from card swipe to settlement, all in one binary.

**Named:** Rust ISO 8583
**Commit:** See initial architecture commit and `3adc034` (expansion)
**Origin:** Building a sovereign app store requires sovereign payments. Every existing payment integration (Stripe, Square) is a dependency. ISO 8583 is the foundation layer — implement it, and you don't need anyone else's payment API.

### 2026-04-08 — Human Revelations Documentation Pass

**What:** Documented novel human-invented techniques across the full CochranBlock portfolio. Added Human Revelations section with Gemini Man Pattern and Rust ISO 8583 builder.
**Commit:** `8db139c`
**AI Role:** AI formatted and wrote the sections. Human identified which techniques were genuinely novel, provided the origin stories, and directed the documentation pass.

---

## Entries

### 2026-04-09 — Backlog Sprint + Payment Hardening

**What:** Executed 6 of 20 backlog items in a single sprint. Bank TCP endpoint wired (BACKLOG #1): f127/f128/f129 in switch/tcp.rs — 2-byte big-endian length prefix wire format (Postilion/Base24), f87 buy-bucks now sends ISO 0200 over TCP when SWITCH_HOST configured, returns 402 on bank decline, 502 on TCP error. Stripe webhook HMAC-SHA256 implemented (BACKLOG #2): f123 parses Stripe-Signature header (t=, v1=), verifies via STRIPE_WEBHOOK_SECRET, hex compare — 6 unit tests. Free-bucks exploit killed (BACKLOG #3): None arm in f87 now returns 503 instead of falling through to ledger credit — removed bank_approved bool entirely. 23 new tests added across both crates (BACKLOG #9, #12, #20): rogue-runner PRNG/zone/level-gen/GameState tests (17), Stripe mapping tests (6), Null Terminal HTTP test (1). Total test count: 128 (was 65+).
**Why:** Backlog prioritized from guest analysis and P23 paranoia lens. BACKLOG #3 was a financial exploit — unauthenticated bucks credit when no bank configured. Shipped same day as discovery.
**Commits:** `0696873`, `e91aa34`, `0f62f77`, `d555308`, `a8f68fe`
**AI Role:** AI implemented TCP wiring, webhook verification, exploit fix, and all tests. Human prioritized the backlog, identified the free-bucks exploit severity, and directed the sprint.

### 2026-04-03 — P23 Triple Lens Analysis + Doc Accuracy Pass

**What:** Applied P23 (Triple Lens Research Protocol) Optimist Lens to kova pyramid architecture — assessed what works, what's close, what's genuinely novel. Full doc accuracy pass: PROOF_OF_ARTIFACTS updated with honest status columns and dashed architecture lines for unwired components. TRIPLE_SIMS docs given status update appendices. All Coming Soon features cross-linked to their blocking dependencies ([Pixel Forge](https://github.com/cochranblock/pixel-forge) for games, [Stripe stubs](rogue-repo/src/switch/stripe.rs) for payments). README "What Works" vs "What Doesn't Work Yet" tables verified against code.
**Why:** P23 protocol requires multi-perspective analysis before architecture decisions. Doc accuracy is enforced — claims must match reality (per claim_verifier philosophy from pyramid architecture).
**Commit:** `2fa7b7b` (docs), plus this entry.
**AI Role:** AI performed P23 Optimist Lens analysis and doc audit. Human directed P23 application and approved all changes.

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
**AI Role:** AI implemented all 3 new MTIs, tokenization, and release tightening. Human specified ISO 8583 message type requirements and economy route wiring.

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
