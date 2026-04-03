<!-- Copyright (c) 2026 The Cochran Block, LLC (Pending). All rights reserved. -->

# Backlog

Prioritized stack. Most important at top. Max 20 items.
Tags: `[build]` `[test]` `[docs]` `[feature]` `[fix]` `[research]`

> This backlog self-reorganizes based on recency and relevance. Items completed or obsoleted get removed. New items enter at their priority position, not the bottom. Cross-project dependencies noted inline.

---

1. ~~`[fix]` **Wire ISO 8583 bank TCP endpoint**~~ DONE (tcp.rs: f127/f128/f129, wired into f87. Set SWITCH_HOST/SWITCH_PORT to activate. Graceful fallback when not configured.)
2. `[feature]` **Implement Stripe webhook verification (f123)** — HMAC-SHA256 over payload vs `Stripe-Signature` header. Requires `STRIPE_SECRET_KEY` env var. Unblocks f120-f122.
3. `[feature]` **Implement Stripe→ISO 8583 translation (f120)** — parse webhook JSON, build corresponding ISO message. Depends on f123 (verify first, translate second).
4. `[feature]` **Implement ISO→Stripe response translation (f122)** — map ISO response codes back to Stripe confirm/cancel calls via reqwest. Depends on f120.
5. `[feature]` **Build MTI 0220 completion message (f121)** — capture after auth hold. Last missing ISO message type for full Stripe flow.
6. `[fix]` **Add CSRF tokens to login/register forms** — POST endpoints accept form submissions with no CSRF protection. Guest analysis flagged this.
7. `[fix]` **Add rate limiting to auth endpoints** — no brute-force protection on `/login`, `/register`. Consider tower-governor or manual token bucket.
8. `[feature]` **User dashboard** — `/dashboard` showing balance, devices, entitlements. No "my account" page exists. Session infrastructure (f125/f126) is ready.
9. ~~`[test]` **Add rogue-runner unit tests**~~ DONE (17 tests: PRNG determinism/range/divergence, zone mapping boundaries, level gen determinism/scaling/dimensions, GameState transitions, gravity, jump guard)
10. `[fix]` **Move hardcoded prices to config** — 420 bucks (entry/device) and 42 bucks (game) are baked into ledger functions f14/f15. Should be DB-driven or env config.
11. `[feature]` **API spec / OpenAPI** — no machine-readable API documentation. Game developers integrating with f87/f88/f89 have no schema reference.
12. ~~`[test]` **Add Stripe mapping unit tests**~~ DONE (6 tests: decline code round-trip, approved=00, event kind parsing, MTI mapping, unknown event/decline return None)
13. `[feature]` **Admin panel** — no user lookup, ledger audit, dispute resolution, or entitlement management. Required before real money flows.
14. `[research]` **Evaluate payment processor options** — Stripe is staged but alternatives exist (Adyen, direct bank integration via ISO 8583 TCP). P23 Triple Lens analysis needed. Depends on: [kova](https://github.com/cochranblock/kova) P23 protocol (f393).
15. `[build]` **Game asset pipeline** — blocked on [pixel-forge](https://github.com/cochranblock/pixel-forge). When pixel-forge ships, generate 96x96 WebP icons + in-game sprites for all 8 Coming Soon titles. Depends on: [pixel-forge](https://github.com/cochranblock/pixel-forge).
16. `[feature]` **Secure cookie attributes audit** — verify Secure flag is set in production (HTTPS only). Current cookies are HttpOnly + SameSite=Lax but Secure flag unconfirmed for prod.
17. `[docs]` **Add getting-started walkthrough** — README has build commands but no step-by-step "first run" guide for new contributors. Include Postgres setup, migration, .env config.
18. `[feature]` **Deploy rogue-repo to gd node** — production deploy target is n1/gd via [approuter](https://github.com/cochranblock/approuter). Hot reload (SO_REUSEPORT) is ready. Depends on: [approuter](https://github.com/cochranblock/approuter) routing config.
19. `[research]` **P23 Pessimist + Paranoia lenses on payment architecture** — Optimist lens completed 2026-04-03. Need pessimist (what fails, gaps) and paranoia (attack vectors, PCI compliance gaps) before shipping real payments. Depends on: [kova](https://github.com/cochranblock/kova) C2 fleet for parallel dispatch.
20. ~~`[test]` **HTTP test for Null Terminal**~~ DONE (get_null_terminal_200: verifies 200 + text/html + "Null Terminal" in body)

---

## Cross-Project Dependencies

| Blocker | Blocked Items | Status |
|---------|--------------|--------|
| [pixel-forge](https://github.com/cochranblock/pixel-forge) | #15 (game assets) | In development — pixel art AI model |
| [kova](https://github.com/cochranblock/kova) | #14, #19 (P23 research) | P23 protocol defined, f393 not yet wired |
| [approuter](https://github.com/cochranblock/approuter) | #18 (deploy to gd) | Routing config ready, deploy not triggered |
