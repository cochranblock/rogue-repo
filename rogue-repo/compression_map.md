<!-- Copyright (c) 2026 The Cochran Block, LLC (Pending). All rights reserved. -->
<!-- Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->
# Rogue Repo Compression Map
# Token-Optimized Code Representation
# Sovereign ISO 8583 payment engine + Rogue Bucks economy

---

## L1: IDENTIFIER MAP

### Functions (f)

```
f0   = main (rogue-repo binary entry)
f4   = serve_index (GET /)
f5   = health (GET /health)
f10  = encrypt_pan (vault)
f11  = decrypt_pan (vault)
f12  = build_0200 (ISO 8583 MTI 0200 purchase request)
f14  = add_device_ledger (deduct 420, insert fingerprint)
f15  = provision_entitlement (42 bucks, insert entitlement)
f16  = credit_bucks (420 for entry buy-in)
f17  = build_0100 (ISO 8583 MTI 0100 authorization request)
f18  = parse_0210 (ISO 8583 MTI 0210 purchase response)
f19  = build_0400 (ISO 8583 MTI 0400 reversal)
f20  = session_user_id (extract user UUID from signed session cookie)
f30  = run_tests (test orchestrator)
f31  = rogue_repo_test (binary, TRIPLE SIMS via exopack f61)
f87  = serve_buy_bucks (POST /buy-bucks)
f88  = serve_provision_app (POST /provision-app)
f89  = serve_add_device (POST /add-device)
f90  = pwa_html (app store PWA index)
f91  = serve_asset (GET /assets/*)
f92  = serve_manifest (GET /manifest.json)
f93  = serve_sw (GET /sw.js)
f94  = serve_rogue_runner (GET /apps/rogue-runner)
f95  = serve_rogue_runner_wasm (GET /apps/rogue-runner-wasm)
f97  = register (POST /register)
f98  = login (POST /login)
f100 = verify_email (GET /verify-email)
f101 = logout (POST /logout, GET /logout)
f102 = serve_login (GET /login)
f103 = serve_register (GET /register)
f117 = serve_null_terminal (GET /apps/null-terminal)
f118 = serve_rogue_runner_download (GET /downloads/rogue-runner)
f124 = extract_session_cookie (cookie value from HeaderMap)
f125 = session_secret (startup enforcer, OnceLock, panics in release if not set)
f126 = require_session (extract + validate session from headers, 401 on fail)
f120 = translate_event (Stripe webhook → ISO 8583, COMING SOON stub)
f121 = build_0220 (ISO 8583 MTI 0220 completion/capture, COMING SOON stub)
f122 = translate_response (ISO 8583 response → Stripe outcome, COMING SOON stub)
f123 = verify_webhook (Stripe webhook signature verify, COMING SOON stub)
```

### Types/Structs (t)

```
t0   = AppState (s0=db_pool)
t1   = Vault (AES-256-GCM)
t2   = PurchaseRequest (ISO 8583 MTI 0200)
t3   = Iso8583Message (raw wire bytes)
t4   = Ledger (wraps PgPool)
t6   = ProvisionAppReq (user_id, game_id)
t14  = ErrorResponse (error string)
t24  = TestResult (name, passed, duration_ms, message)
t30  = AuthRequest (ISO 8583 MTI 0100)
t31  = PurchaseResponse (parsed MTI 0210)
t32  = ReversalRequest (ISO 8583 MTI 0400)
t33  = Assets (embedded static files via rust-embed)
t34  = ReversalReason (Timeout, CustomerCancel, SystemError)
t83  = BuyBucksReq (s87=user_id, pan_encrypted)
t84  = BuyBucksRes (s85=ok, s84=message)
t36  = StripeDeclineCode (Stripe decline → ISO 8583 response code)
t37  = StripeEventKind (Stripe webhook event → ISO 8583 MTI)
t86  = AddDeviceReq (s87=user_id, s88=hardware_fingerprint)
t97  = RegisterForm (email, password)
t98  = LoginForm (email, password)
t99  = AuthRes (ok, message, user_id)
t118 = DownloadQuery (platform)
```

### Parameters (p)

```
p0 = state (AppState)
p2 = body / payload
p6 = user_id
p7 = hardware_fingerprint
p8 = game_id
```

### Struct Fields (s)

```
s0  = db_pool (Option<PgPool> in AppState)
s84 = message (in BuyBucksRes)
s85 = ok (in BuyBucksRes)
s87 = user_id (in BuyBucksReq, AddDeviceReq)
s88 = hardware_fingerprint (in AddDeviceReq)
```

### Error Enums

```
E3 = VaultError (Encrypt, Decrypt, Key, Ciphertext)
E4 = SwitchError (Pack, Unpack, Connection, Amount)
E5 = LedgerError (Insufficient, DeviceExists, Db, NotFound)
```

### Constants (c)

```
c10 = SESSION_COOKIE ("rr_session")
```

### Rogue Runner (rogue-runner binary + lib)

```
f95  = mulberry32 (lib PRNG)
f96  = generate_level (lib)
f105 = load_progress
f106 = save_progress
f107 = start_game
f108 = jump
f109 = game_over
f110 = level_complete
f111 = update
f112 = draw
f115 = rogue_runner_test (binary, TRIPLE SIMS via exopack f61)
f117 = zone_for_level (lib)
t35  = Action (None, Jump, Start)
t88  = GameState
t95  = Obstacle (x, h, w)
t96  = LevelData (speed, obstacles)
s88  = state (GameState)
s89  = level
s90  = score
s91  = player_y
s92  = vy
s93  = level_data
s94  = obstacle_idx
s95  = saved_level
s96  = run_frame (animation)
s97  = is_jumping (animation)
c90  = MAX_LEVEL (1000)
c91  = GRAVITY
c92  = JUMP
c93  = PLAYER_H
c94  = PLAYER_W
c95  = GROUND
```

---

## PRESERVED (not compressed)

Rust std, tokio, axum, sqlx, aes_gcm, bitvec, bytes, serde, argon2, hmac, sha2, reqwest, chrono, macroquad.
