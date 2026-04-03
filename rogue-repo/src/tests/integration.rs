// Copyright (c) 2026 The Cochran Block, LLC (Pending). All rights reserved.
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! f50 = integration: real DB, real ledger f14/f15/f16

use std::time::Instant;

use crate::ledger::t4;
use crate::switch::{f12, f127, f129, t2, t39};
use sqlx::postgres::PgPoolOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use uuid::Uuid;

use crate::tests::t24;

pub async fn f50() -> Vec<t24> {
    let mut out = Vec::new();

    if let Ok(url) = std::env::var("DATABASE_URL") {
        out.push(ledger_add_device(&url).await);
        out.push(ledger_provision_entitlement(&url).await);
        out.push(ledger_credit_bucks(&url).await);
        out.push(ledger_insufficient_rejected(&url).await);
        out.push(ledger_device_exists_rejected(&url).await);
    }

    // ISO 8583 TCP mock tests — no real bank required
    out.push(tcp_mock_send_and_receive().await);
    out.push(tcp_mock_parse_0210_round_trip().await);
    out.push(tcp_mock_connection_refused().await);
    out.push(tcp_mock_invalid_response_length().await);

    out
}

async fn ledger_add_device(url: &str) -> t24 {
    let start = Instant::now();
    let pool = match PgPoolOptions::new().max_connections(2).connect(url).await {
        Ok(p) => p,
        Err(e) => {
            return t24 {
                name: "ledger_add_device".into(),
                passed: false,
                duration_ms: start.elapsed().as_millis() as u64,
                message: Some(format!("pool: {}", e)),
            };
        }
    };
    if let Err(e) = sqlx::migrate!("../migrations").run(&pool).await {
        return t24 {
            name: "ledger_add_device".into(),
            passed: false,
            duration_ms: start.elapsed().as_millis() as u64,
            message: Some(format!("migrate: {}", e)),
        };
    }
    let uid = Uuid::new_v4();
    sqlx::query("INSERT INTO users (id, email, rogue_bucks_balance) VALUES ($1, $2, 500)")
        .bind(uid)
        .bind(format!("test-{}@x.com", uid))
        .execute(&pool)
        .await
        .ok();
    let ledger = t4::new(pool.clone());
    let r = ledger.f14(uid, "fp-test-1").await;
    let Ok(()) = r else {
        return t24 {
            name: "ledger_add_device".into(),
            passed: false,
            duration_ms: start.elapsed().as_millis() as u64,
            message: Some(format!("f14: {:?}", r.err())),
        };
    };
    let row: (i64,) = sqlx::query_as("SELECT rogue_bucks_balance FROM users WHERE id = $1")
        .bind(uid)
        .fetch_one(&pool)
        .await
        .unwrap();
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM devices WHERE user_id = $1 AND hardware_fingerprint = $2",
    )
    .bind(uid)
    .bind("fp-test-1")
    .fetch_one(&pool)
    .await
    .unwrap();
    sqlx::query("DELETE FROM devices WHERE user_id = $1")
        .bind(uid)
        .execute(&pool)
        .await
        .ok();
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(uid)
        .execute(&pool)
        .await
        .ok();
    let ok = row.0 == 80 && count.0 == 1;
    t24 {
        name: "ledger_add_device".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("balance 80, device count 1".into())
        },
    }
}

async fn ledger_provision_entitlement(url: &str) -> t24 {
    let start = Instant::now();
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(url)
        .await
        .unwrap();
    let uid = Uuid::new_v4();
    sqlx::query("INSERT INTO users (id, email, rogue_bucks_balance) VALUES ($1, $2, 100)")
        .bind(uid)
        .bind(format!("test-{}@x.com", uid))
        .execute(&pool)
        .await
        .unwrap();
    let ledger = t4::new(pool.clone());
    let r = ledger.f15(uid, "game-1").await;
    let Ok(()) = r else {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(uid)
            .execute(&pool)
            .await
            .ok();
        return t24 {
            name: "ledger_provision_entitlement".into(),
            passed: false,
            duration_ms: start.elapsed().as_millis() as u64,
            message: Some(format!("f15: {:?}", r.err())),
        };
    };
    let row: (i64,) = sqlx::query_as("SELECT rogue_bucks_balance FROM users WHERE id = $1")
        .bind(uid)
        .fetch_one(&pool)
        .await
        .unwrap();
    let has: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM entitlements WHERE user_id = $1 AND game_id = $2")
            .bind(uid)
            .bind("game-1")
            .fetch_one(&pool)
            .await
            .unwrap();
    sqlx::query("DELETE FROM entitlements WHERE user_id = $1")
        .bind(uid)
        .execute(&pool)
        .await
        .ok();
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(uid)
        .execute(&pool)
        .await
        .ok();
    let ok = row.0 == 58 && has.0 == 1;
    t24 {
        name: "ledger_provision_entitlement".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("balance 58, entitlement exists".into())
        },
    }
}

async fn ledger_credit_bucks(url: &str) -> t24 {
    let start = Instant::now();
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(url)
        .await
        .unwrap();
    let uid = Uuid::new_v4();
    sqlx::query("INSERT INTO users (id, email, rogue_bucks_balance) VALUES ($1, $2, 0)")
        .bind(uid)
        .bind(format!("test-{}@x.com", uid))
        .execute(&pool)
        .await
        .unwrap();
    let ledger = t4::new(pool.clone());
    ledger.f16(uid, 420).await.unwrap();
    let row: (i64,) = sqlx::query_as("SELECT rogue_bucks_balance FROM users WHERE id = $1")
        .bind(uid)
        .fetch_one(&pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(uid)
        .execute(&pool)
        .await
        .ok();
    let ok = row.0 == 420;
    t24 {
        name: "ledger_credit_bucks".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("balance 420 after credit".into())
        },
    }
}

async fn ledger_insufficient_rejected(url: &str) -> t24 {
    let start = Instant::now();
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(url)
        .await
        .unwrap();
    let uid = Uuid::new_v4();
    sqlx::query("INSERT INTO users (id, email, rogue_bucks_balance) VALUES ($1, $2, 10)")
        .bind(uid)
        .bind(format!("test-{}@x.com", uid))
        .execute(&pool)
        .await
        .unwrap();
    let ledger = t4::new(pool.clone());
    let r = ledger.f14(uid, "fp-x").await;
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(uid)
        .execute(&pool)
        .await
        .ok();
    let ok = r.is_err();
    t24 {
        name: "ledger_insufficient_rejected".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("f14 with 10 bucks must fail".into())
        },
    }
}

// ---------------------------------------------------------------------------
// ISO 8583 TCP mock integration tests
// ---------------------------------------------------------------------------

/// Spin up a mock TCP server that responds with a valid 0210 message.
fn build_mock_0210(amount_cents: u64, stan: u32, approved: bool) -> Vec<u8> {
    let rc = if approved { b"00" } else { b"05" };
    let mut msg = Vec::new();
    msg.extend_from_slice(b"0210");
    msg.extend_from_slice(&[0u8; 8]); // bitmap
    msg.extend_from_slice(rc);
    msg.extend_from_slice(format!("{:012}", amount_cents).as_bytes());
    msg.extend_from_slice(format!("{:06}", stan).as_bytes());
    msg
}

async fn spawn_mock_switch(response: Vec<u8>) -> t39 {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        if let Ok((mut stream, _)) = listener.accept().await {
            // Read length-prefixed request
            let mut len_buf = [0u8; 2];
            let _ = stream.read_exact(&mut len_buf).await;
            let len = u16::from_be_bytes(len_buf) as usize;
            let mut _req = vec![0u8; len];
            let _ = stream.read_exact(&mut _req).await;
            // Send length-prefixed response
            let resp_len = response.len() as u16;
            let _ = stream.write_all(&resp_len.to_be_bytes()).await;
            let _ = stream.write_all(&response).await;
        }
    });
    t39 { host: addr.ip().to_string(), port: addr.port(), timeout_ms: 2000 }
}

async fn tcp_mock_send_and_receive() -> t24 {
    let start = Instant::now();
    let mock_resp = build_mock_0210(420, 1, true);
    let endpoint = spawn_mock_switch(mock_resp.clone()).await;
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    let req = t2 { pan_encrypted: vec![1, 2, 3], amount_cents: 420, stan: 1 };
    let msg = f12(&req).unwrap();
    let result = f127(&endpoint, &msg).await;
    let ok = match result {
        Ok(raw) => raw == mock_resp,
        Err(_) => false,
    };
    t24 { name: "tcp_mock_send_and_receive".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("f127 round-trip to mock switch must return exact response bytes".into()) } }
}

async fn tcp_mock_parse_0210_round_trip() -> t24 {
    let start = Instant::now();
    let mock_resp = build_mock_0210(1000, 42, false); // declined
    let endpoint = spawn_mock_switch(mock_resp).await;
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    let req = t2 { pan_encrypted: vec![9, 8, 7], amount_cents: 1000, stan: 42 };
    let msg = f12(&req).unwrap();
    let result = f129(&endpoint, &msg).await;
    let ok = match result {
        Ok(r) => !r.approved && r.stan == 42 && r.amount_cents == 1000,
        Err(_) => false,
    };
    t24 { name: "tcp_mock_parse_0210_round_trip".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("f129 declined response: approved=false, correct stan+amount".into()) } }
}

async fn tcp_mock_connection_refused() -> t24 {
    let start = Instant::now();
    // Bind to get a free port, then drop the listener so it's closed
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);

    let endpoint = t39 { host: "127.0.0.1".into(), port, timeout_ms: 500 };
    let req = t2 { pan_encrypted: vec![1], amount_cents: 100, stan: 1 };
    let msg = f12(&req).unwrap();
    let result = f127(&endpoint, &msg).await;
    let ok = result.is_err();
    t24 { name: "tcp_mock_connection_refused".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("connection to closed port must return Err".into()) } }
}

async fn tcp_mock_invalid_response_length() -> t24 {
    let start = Instant::now();
    // Server sends length=0 (invalid per our check)
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        if let Ok((mut stream, _)) = listener.accept().await {
            let mut len_buf = [0u8; 2];
            let _ = stream.read_exact(&mut len_buf).await;
            let len = u16::from_be_bytes(len_buf) as usize;
            let mut _req = vec![0u8; len];
            let _ = stream.read_exact(&mut _req).await;
            // Send length=0 (invalid)
            let _ = stream.write_all(&0u16.to_be_bytes()).await;
        }
    });
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    let endpoint = t39 { host: addr.ip().to_string(), port: addr.port(), timeout_ms: 2000 };
    let req = t2 { pan_encrypted: vec![1], amount_cents: 100, stan: 1 };
    let msg = f12(&req).unwrap();
    let result = f127(&endpoint, &msg).await;
    let ok = result.is_err();
    t24 { name: "tcp_mock_invalid_response_length".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("length=0 response must return Err".into()) } }
}

async fn ledger_device_exists_rejected(url: &str) -> t24 {
    let start = Instant::now();
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(url)
        .await
        .unwrap();
    let uid = Uuid::new_v4();
    sqlx::query("INSERT INTO users (id, email, rogue_bucks_balance) VALUES ($1, $2, 1000)")
        .bind(uid)
        .bind(format!("test-{}@x.com", uid))
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO devices (id, user_id, hardware_fingerprint, is_authorized) VALUES (gen_random_uuid(), $1, $2, true)",
    )
        .bind(uid)
        .bind("fp-dupe")
        .execute(&pool)
        .await
        .unwrap();
    let ledger = t4::new(pool.clone());
    let r = ledger.f14(uid, "fp-dupe").await;
    sqlx::query("DELETE FROM devices WHERE user_id = $1")
        .bind(uid)
        .execute(&pool)
        .await
        .ok();
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(uid)
        .execute(&pool)
        .await
        .ok();
    let ok = r.is_err();
    t24 {
        name: "ledger_device_exists_rejected".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("f14 duplicate device must fail".into())
        },
    }
}
