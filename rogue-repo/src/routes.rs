// Copyright (c) 2026 The Cochran Block, LLC (Pending). All rights reserved.
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! f87=serve_buy_bucks f88=serve_provision_app f89=serve_add_device. t83=BuyBucksReq t86=AddDeviceReq.

use axum::http::{HeaderMap, StatusCode};
use axum::{extract::State, response::Html, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth;
use crate::ledger::t4;
use crate::pwa;
use crate::switch::{f12, f129, t2, t39};

/// f126 = require_session — extract and validate session from headers.
/// Returns authenticated user_id or 401 error.
pub fn f126(headers: &HeaderMap) -> Result<Uuid, (StatusCode, Json<t84>)> {
    let cookie_val = auth::f124(headers);
    auth::f20(cookie_val.as_deref()).ok_or((
        StatusCode::UNAUTHORIZED,
        Json(t84 {
            s85: false,
            s84: "Authentication required".into(),
        }),
    ))
}

#[derive(Clone)]
pub struct t0 {
    pub s0: Option<sqlx::PgPool>,
}

/// t83 = BuyBucksReq
#[derive(Deserialize)]
pub struct t83 {
    #[serde(rename = "user_id")]
    pub s87: Uuid,
    #[serde(rename = "pan_encrypted")]
    pub pan_encrypted: Vec<u8>,
}

#[derive(Deserialize)]
pub struct t6 {
    pub user_id: Uuid,
    pub game_id: String,
}

/// t86 = AddDeviceReq
#[derive(Deserialize)]
pub struct t86 {
    #[serde(rename = "user_id")]
    pub s87: Uuid,
    #[serde(rename = "hardware_fingerprint")]
    pub s88: String,
}

/// t84 = BuyBucksRes (s85=ok s84=message)
#[derive(Serialize)]
pub struct t84 {
    #[serde(rename = "ok")]
    pub s85: bool,
    #[serde(rename = "message")]
    pub s84: String,
}

#[derive(Serialize)]
pub struct t14 {
    pub error: String,
}

/// f5 = health, GET /health
pub async fn f5() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({"ok": true}))
}

/// f4 = serve_index, GET / — native Rust PWA HTML
pub async fn f4(headers: HeaderMap) -> Html<String> {
    let cookie_val = auth::f124(&headers);
    let session_user = auth::f20(cookie_val.as_deref());
    Html(pwa::f90(session_user))
}

/// f87 = serve_buy_bucks, POST /buy-bucks
/// Flow: build ISO 8583 MTI 0200 → (bank send TBD) → credit bucks via ledger
pub async fn f87(
    headers: HeaderMap,
    State(p0): State<t0>,
    Json(p2): Json<t83>,
) -> (StatusCode, Json<t84>) {
    let session_uid = match f126(&headers) {
        Ok(uid) => uid,
        Err(e) => return e,
    };
    if session_uid != p2.s87 {
        return (
            StatusCode::FORBIDDEN,
            Json(t84 {
                s85: false,
                s84: "Session user does not match request".into(),
            }),
        );
    }

    let pool = match &p0.s0 {
        Some(p) => p,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(t84 {
                    s85: false,
                    s84: "Database not configured".into(),
                }),
            );
        }
    };

    // Build ISO 8583 MTI 0200 purchase request
    let iso_req = t2 {
        pan_encrypted: p2.pan_encrypted,
        amount_cents: 420,
        stan: rand::random::<u32>() % 999999,
    };
    let iso_msg = match f12(&iso_req) {
        Ok(m) => m,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(t84 {
                    s85: false,
                    s84: format!("ISO 8583 build failed: {}", e),
                }),
            );
        }
    };

    tracing::info!("ISO 8583 MTI 0200 built: {} bytes", iso_msg.raw.len());

    // Send to bank via TCP — SWITCH_HOST is required. No fallback credit.
    let iso_len = iso_msg.raw.len();
    match t39::from_env() {
        Some(endpoint) => {
            match f129(&endpoint, &iso_msg).await {
                Ok(resp) => {
                    if resp.approved {
                        tracing::info!(
                            "Bank approved: STAN={}, auth_code={:?}",
                            resp.stan,
                            resp.auth_code.map(|c| String::from_utf8_lossy(&c).to_string())
                        );
                    } else {
                        tracing::warn!(
                            "Bank declined: response_code={}",
                            String::from_utf8_lossy(&resp.response_code)
                        );
                        return (
                            StatusCode::PAYMENT_REQUIRED,
                            Json(t84 {
                                s85: false,
                                s84: format!(
                                    "Payment declined (ISO response code: {})",
                                    String::from_utf8_lossy(&resp.response_code)
                                ),
                            }),
                        );
                    }
                }
                Err(e) => {
                    tracing::error!("Bank TCP error: {}", e);
                    return (
                        StatusCode::BAD_GATEWAY,
                        Json(t84 {
                            s85: false,
                            s84: format!("Payment processor error: {}", e),
                        }),
                    );
                }
            }
        }
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(t84 {
                    s85: false,
                    s84: "Payment processor not configured".into(),
                }),
            );
        }
    };

    // Credit 420 Rogue Bucks — only reached after bank approval
    let ledger = t4::new(pool.clone());
    match ledger.f16(p2.s87, 420).await {
        Ok(()) => {
            let msg = format!(
                "420 Rogue Bucks credited. Bank approved ({} bytes sent).",
                iso_len
            );
            (StatusCode::OK, Json(t84 { s85: true, s84: msg }))
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(t84 {
                s85: false,
                s84: format!("Ledger error: {}", e),
            }),
        ),
    }
}

/// f88 = serve_provision_app, POST /provision-app (42 bucks)
pub async fn f88(
    headers: HeaderMap,
    State(p0): State<t0>,
    Json(p2): Json<t6>,
) -> (StatusCode, Json<t84>) {
    let session_uid = match f126(&headers) {
        Ok(uid) => uid,
        Err(e) => return e,
    };
    if session_uid != p2.user_id {
        return (
            StatusCode::FORBIDDEN,
            Json(t84 {
                s85: false,
                s84: "Session user does not match request".into(),
            }),
        );
    }

    let pool = match &p0.s0 {
        Some(p) => p,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(t84 {
                    s85: false,
                    s84: "Database not configured".into(),
                }),
            );
        }
    };

    let ledger = t4::new(pool.clone());
    match ledger.f15(p2.user_id, &p2.game_id).await {
        Ok(()) => (
            StatusCode::OK,
            Json(t84 {
                s85: true,
                s84: format!("Provisioned '{}'. 42 Rogue Bucks deducted.", p2.game_id),
            }),
        ),
        Err(e) => {
            let (status, msg) = match &e {
                crate::ledger::E5::Insufficient(bal) => (
                    StatusCode::PAYMENT_REQUIRED,
                    format!("Insufficient Rogue Bucks: {} (need 42)", bal),
                ),
                crate::ledger::E5::NotFound(_) => {
                    (StatusCode::NOT_FOUND, "User not found".to_string())
                }
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Ledger error: {}", e),
                ),
            };
            (
                status,
                Json(t84 {
                    s85: false,
                    s84: msg,
                }),
            )
        }
    }
}

/// f89 = serve_add_device, POST /add-device (420 bucks)
pub async fn f89(
    headers: HeaderMap,
    State(p0): State<t0>,
    Json(p2): Json<t86>,
) -> (StatusCode, Json<t84>) {
    let session_uid = match f126(&headers) {
        Ok(uid) => uid,
        Err(e) => return e,
    };
    if session_uid != p2.s87 {
        return (
            StatusCode::FORBIDDEN,
            Json(t84 {
                s85: false,
                s84: "Session user does not match request".into(),
            }),
        );
    }

    let pool = match &p0.s0 {
        Some(p) => p,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(t84 {
                    s85: false,
                    s84: "Database not configured".into(),
                }),
            );
        }
    };

    let ledger = t4::new(pool.clone());
    match ledger.f14(p2.s87, &p2.s88).await {
        Ok(()) => (
            StatusCode::OK,
            Json(t84 {
                s85: true,
                s84: "Device registered. 420 Rogue Bucks deducted.".into(),
            }),
        ),
        Err(e) => {
            let (status, msg) = match &e {
                crate::ledger::E5::Insufficient(bal) => (
                    StatusCode::PAYMENT_REQUIRED,
                    format!("Insufficient Rogue Bucks: {} (need 420)", bal),
                ),
                crate::ledger::E5::DeviceExists => (
                    StatusCode::CONFLICT,
                    "Device already registered".into(),
                ),
                crate::ledger::E5::NotFound(_) => {
                    (StatusCode::NOT_FOUND, "User not found".into())
                }
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Ledger error: {}", e),
                ),
            };
            (
                status,
                Json(t84 {
                    s85: false,
                    s84: msg,
                }),
            )
        }
    }
}
