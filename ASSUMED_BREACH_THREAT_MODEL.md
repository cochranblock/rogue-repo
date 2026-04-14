# Assumed Breach Threat Model

> **Operating assumption: every component below is already compromised. Design for damage containment and loud detection, not for prevention.**

This document is the canonical threat model for every project in the `cochranblock/*` portfolio. Each project adapts the Threat Surface section for its own context but shares the same first principles, mitigations, and verification protocol.

---

## First Principles

1. **Every record that matters has an external witness.** Hashes published to public git (or equivalent neutral timestamp authority) so tampering requires simultaneously corrupting your system AND the public chain.
2. **No single point of compromise.** Signing keys in hardware (YubiKey / TPM / Secure Enclave). Never in software. Never in env vars. Never in config files.
3. **Default air-gap.** No network dependency for correctness. Network is for backup + publishing hashes, both signed, both verifiable post-hoc.
4. **Append-only everything.** No delete path in any storage layer. Corrections are reversing entries referencing the original. Standard accounting discipline, enforced in code.
5. **Cryptographic audit chain.** Every day's state derives from the previous day's hash. Tampering with any day invalidates every subsequent day.
6. **Disclosure of methodology is a security feature.** If an auditor can independently verify the algorithm, they can independently verify the outputs. No "trust us" layers.
7. **Separation of duties enforced in software.** Entry, approval, and audit live in different trust zones. Compromise of one does not compromise the others.
8. **Redundancy across trust zones.** Local + different-cloud + different-format + offline. Attacker must compromise all to hide damage.
9. **Test breach scenarios regularly.** Triple Sims applied to tamper detection. If the chain does not detect a simulated tamper, the chain is broken.

---

## Threat Surface (project-specific — adapt below)

rogue-repo is a sovereign software distribution PWA plus an ISO 8583 payment switch. It emits records that are simultaneously **financial** (money moves), **legal** (entitlement = license to a game binary), and **operational** (binary integrity = what the user actually runs). Every category of record must survive the assumed-breach posture independently.

**Records of consequence emitted by this project:**

- **Ledger entries** — `entitlements(user_id, game_id)` rows in PostgreSQL + sled mirror. Every row is a property right (user owns a game) and a settled debit of Rogue Bucks.
- **Rogue Bucks balance mutations** — every credit (USD→bucks conversion, bonuses) and debit (game purchase 42 RB, device registration 420 RB, entry buy-in 420 RB). 100 RB = $1.00 USD, so forged entries map 1:1 to dollars stolen.
- **ISO 8583 transaction logs** — auth (0100), reversal (0400), settlement (0200) messages through the switch (`src/switch/iso8583.rs`, `src/switch/tcp.rs`). Each is a financial instruction from/to an upstream acquirer.
- **Stripe webhook receipts** — HMAC-SHA256 verified events (`src/switch/stripe.rs`) that trigger Rogue Bucks issuance. A forged or replayed webhook fabricates currency.
- **Game download artifacts** — Windows MSI/EXE, Android APK, macOS, Linux binaries served from `/downloads/rogue-runner`. What the user installs and executes.
- **Device registration records** — 420 RB fee links a device to an account. Forged registrations grant paid entitlements to attacker-controlled devices.

**Assumed-breach scenarios specific to this surface:**

| Assume compromised | Attacker-visible outcome |
|--------------------|--------------------------|
| ISO 8583 switch process | Attacker forges auth responses or replays settlements — can mint fraudulent approvals to upstream acquirers |
| Stripe webhook endpoint | Attacker replays captured webhooks or bypasses HMAC check — fabricates Rogue Bucks credits without real USD flow |
| Ledger PostgreSQL | Attacker rewrites entitlement/balance history — free games, inflated balances, deleted audit traces |
| sled storage (hot path) | Attacker desynchronizes sled from text-of-truth — dual-layer divergence hides tampering if only one side is checked |
| Signing key for downloads | Attacker produces MSI/APK/EXE signed as rogue-repo — supply-chain compromise of every player's machine |
| Vault (`src/vault/`) | Secrets (Stripe keys, ISO 8583 HSM creds, download signing keys) leak — downstream compromise of every integration |
| Auth / session store | Attacker acts as any user or merchant — purchases games, drains balances, registers devices |
| Game provisioning endpoint (`POST /provision-app`, `f88`) | Attacker issues entitlements without debiting Rogue Bucks — infinite free games |
| `SWITCH_HOST` env or similar config | Attacker redirects ISO 8583 traffic to attacker-controlled acquirer — MITM on live money flow (see BACKLOG #3, fixed via 503 on missing config) |
| Download artifact storage (`assets/downloads/`) | Attacker swaps binary after signing step — users get malicious build under valid metadata if hash isn't re-verified at serve time |
| Webhook replay window | Same Stripe `event.id` processed twice — double-credits Rogue Bucks unless idempotency keyed on `event.id` in append-only dedup log |
| Clock manipulation on switch host | Attacker rewinds to replay settlements or forward-skips reversal windows — breaks ISO 8583 timing invariants |
| Host filesystem seizure | Full access to sled trees, ledger dumps, cached webhook payloads, any in-memory secrets swapped to disk |
| Supply chain (Rust crates) | Backdoored `hmac`, `sha2`, `reqwest`, or payment-adjacent crate compromises signature verification silently |

**N/A for this project:**

- **PAN storage / PCI-DSS cardholder data at rest** — the switch is a routing layer; card data is tokenized upstream by Stripe and never persisted by rogue-repo. If this changes, PCI-DSS SAQ scope activates and this line moves to the threat table above.
- **Source code confidentiality** — codebase is Unlicensed/public domain per portfolio standard. Disclosure is a feature, not a risk.

---

## Mitigations

| Assume | Mitigation | Verification |
|--------|-----------|--------------|
| Binary compromised | Hardware-key signatures for every output of consequence | Anyone can verify the public key matches expected fingerprint |
| Storage compromised | Append-only sled trees. Delete is not a function, not a policy. | Hash chain breaks on any rewrite. External witness detects. |
| Network MITM | Air-gap capable. Network used only for signed backups + hash publishing. | NTP + GitHub timestamp + hardware counter cross-checked. |
| Signing key stolen | Daily hash committed to public git. Stolen key cannot retroactively change committed days. | Any day older than the public commit is immutable in evidence. |
| Audit log tampered | Separate sled tree, write-only from main app. Auditor tool reads both + cross-checks. | Compromise of main app leaves audit log intact. |
| Backup tampered | 3 different targets with 3 different credentials (local USB + off-site cloud + paper). | Attacker needs all three to hide damage. |
| Insider / self-tampering | No admin role. No delete. Reversing entries only. | Legal record immune to author second-thoughts. |
| Clock manipulation | Multiple time sources: local clock, NTP, git commit timestamp, hardware-key counter. | Divergence flags exception requiring supervisor approval. |
| Supply chain (deps) | `cargo audit` in CI. Pinned SBOM. Reproducible builds where possible. | Anyone can reproduce the binary from source + lockfile. |
| Physical device seizure | Full-disk encryption. Hardware key physically separate from device. | Stolen laptop without key is useless for forgery. |

---

## Public-Chain Deployment

This project publishes tamper-evident hashes to a public companion repo: `cochranblock/<project>-chain` (where `<project>` is the project name).

- **Daily cycle:** at 23:59 local, compute BLAKE3 of all records-of-consequence from the day. Sign with hardware key. Commit to chain repo. Push.
- **GitHub timestamp** on the commit = neutral third-party witness. Anyone can cold-verify records were not rewritten after commit time.
- **Verification:** `<project> verify` reads the chain and re-derives hashes. Any divergence = tampering detected.

This pattern is a private Certificate Transparency log for project state. Same primitive Google uses for TLS certs, applied to whatever the project tracks.

---

## Triple Sims for Tamper Detection

Standard Triple Sims gate (run 3x identically) extended with a tamper-scenario sim:

1. Normal run → produce canonical output
2. Simulated tampering (flip one bit in storage) → `verify` must flag it
3. Simulated clock rewind → `verify` must flag it

If any sim fails to detect, the chain is broken. Fix before merge.

---

## Scope of this Document

- Covers: any artifact this project emits that has legal, financial, or audit consequence.
- Does NOT cover: source code itself (public under Unlicense, not sensitive), build outputs (reproducible), marketing content (public by design).
- If your project emits no records of consequence, the relevant sections are zero-length and the public-chain deployment is skipped. Document that explicitly.

---

## Relation to Other Docs

- **TIMELINE_OF_INVENTION.md** — establishes priority dates for contributions. Feeds into the chain's initial state.
- **PROOF_OF_ARTIFACTS.md** — cryptographic signatures on release artifacts. Adjacent pattern, same first principles.
- **DCAA_COMPLIANCE.md** (where applicable) — how this threat model satisfies FAR/DFARS audit requirements.

---

## Status

- [ ] Threat Surface section adapted for this project
- [ ] Hardware-key signing integrated or N/A documented
- [ ] Public-chain repo created and connected or N/A documented
- [ ] Triple Sims tamper-detection test present or N/A documented
- [ ] External verification procedure documented

---

*Unlicensed. Public domain. Fork, strip attribution, adapt, ship.*

*Canonical source: cochranblock.org/threat-model — last revision 2026-04-14*
