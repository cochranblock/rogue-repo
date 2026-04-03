// Copyright (c) 2026 The Cochran Block, LLC (Pending). All rights reserved.
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! f51 = HTTP tests: real server, real requests

use std::time::Instant;

use axum::{routing::get, Router};
use tokio::net::TcpListener;

use crate::pwa;
use crate::routes;
use crate::tests::t24;

pub async fn f51() -> Vec<t24> {
    let mut out = Vec::new();

    let listener = match TcpListener::bind("127.0.0.1:0").await {
        Ok(l) => l,
        Err(e) => {
            return vec![t24 {
                name: "http_bind".into(),
                passed: false,
                duration_ms: 0,
                message: Some(format!("bind: {}", e)),
            }];
        }
    };
    let addr = listener.local_addr().unwrap();
    let p0 = routes::t0 { s0: None };
    let app = Router::new()
        .route("/", get(routes::f4))
        .route("/login", get(pwa::f102))
        .route("/register", get(pwa::f103))
        .route("/manifest.json", get(pwa::f92))
        .route("/sw.js", get(pwa::f93))
        .route("/assets/*path", get(pwa::f91))
        .route("/apps/rogue-runner", get(pwa::f94))
        .route("/apps/rogue-runner-wasm", get(pwa::f95))
        .route("/apps/null-terminal", get(pwa::f117))
        .route("/health", get(health))
        .route("/buy-bucks", axum::routing::post(routes::f87))
        .route("/provision-app", axum::routing::post(routes::f88))
        .route("/add-device", axum::routing::post(routes::f89))
        .with_state(p0);

    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let base = format!("http://{}", addr);
    let client = reqwest::Client::new();

    out.push(get_index_200(&client, &base).await);
    out.push(get_health_200(&client, &base).await);
    out.push(get_index_contains_economy(&client, &base).await);
    out.push(get_index_shows_login_not_logout(&client, &base).await);
    out.push(get_manifest_200(&client, &base).await);
    out.push(get_manifest_valid_json(&client, &base).await);
    out.push(get_sw_200(&client, &base).await);
    out.push(get_icon_200(&client, &base).await);
    out.push(get_app_image_200(&client, &base).await);
    out.push(get_rogue_runner_200(&client, &base).await);
    out.push(get_rogue_runner_wasm_200(&client, &base).await);
    out.push(get_null_terminal_200(&client, &base).await);
    out.push(get_not_found_404(&client, &base).await);
    out.push(get_health_json_ok(&client, &base).await);
    out.push(get_asset_404(&client, &base).await);
    out.push(post_buy_bucks_401_no_session(&client, &base).await);
    out.push(post_provision_app_401_no_session(&client, &base).await);
    out.push(post_add_device_401_no_session(&client, &base).await);
    out.push(post_invalid_json_returns_4xx(&client, &base).await);
    out.push(post_buy_bucks_no_session_401(&client, &base).await);
    // New: expanded coverage
    out.push(get_login_page_200(&client, &base).await);
    out.push(get_register_page_200(&client, &base).await);
    out.push(get_asset_path_traversal_blocked(&client, &base).await);
    out.push(get_index_contains_pixel_forge(&client, &base).await);
    out.push(get_index_has_coming_soon(&client, &base).await);
    out.push(get_health_content_type_json(&client, &base).await);
    out.push(get_sw_content_type_js(&client, &base).await);
    out.push(get_icon_svg_content_type(&client, &base).await);
    out.push(post_empty_body_returns_4xx(&client, &base).await);
    out.push(get_index_has_iso_8583_reference(&client, &base).await);

    out
}

async fn health() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({"ok": true}))
}

async fn get_index_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/", base)).send().await;
    let ok = match r {
        Ok(res) => {
            res.status() == 200
                && res
                    .text()
                    .await
                    .map(|t| t.contains("Rogue Repo"))
                    .unwrap_or(false)
        }
        Err(_) => false,
    };
    t24 {
        name: "get_index_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET / 200 + Rogue Repo".into())
        },
    }
}

async fn get_health_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/health", base)).send().await;
    let ok = match r {
        Ok(res) => res.status() == 200,
        Err(_) => false,
    };
    t24 {
        name: "get_health_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET /health 200".into())
        },
    }
}

async fn post_buy_bucks_401_no_session(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let body = serde_json::json!({
        "user_id": "00000000-0000-0000-0000-000000000001",
        "pan_encrypted": []
    });
    let r = client
        .post(format!("{}/buy-bucks", base))
        .json(&body)
        .send()
        .await;
    let ok = match r {
        Ok(res) => {
            let status = res.status().as_u16() == 401;
            let text = res.text().await.unwrap_or_default();
            status && text.contains("Authentication required")
        }
        Err(_) => false,
    };
    t24 {
        name: "post_buy_bucks_401_no_session".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("POST /buy-bucks without session must return 401".into())
        },
    }
}

async fn post_provision_app_401_no_session(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let body = serde_json::json!({
        "user_id": "00000000-0000-0000-0000-000000000001",
        "game_id": "game-1"
    });
    let r = client
        .post(format!("{}/provision-app", base))
        .json(&body)
        .send()
        .await;
    let ok = match r {
        Ok(res) => {
            let status = res.status().as_u16() == 401;
            let text = res.text().await.unwrap_or_default();
            status && text.contains("Authentication required")
        }
        Err(_) => false,
    };
    t24 {
        name: "post_provision_app_401_no_session".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("POST /provision-app without session must return 401".into())
        },
    }
}

async fn post_add_device_401_no_session(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let body = serde_json::json!({
        "user_id": "00000000-0000-0000-0000-000000000001",
        "hardware_fingerprint": "fp-abc123"
    });
    let r = client
        .post(format!("{}/add-device", base))
        .json(&body)
        .send()
        .await;
    let ok = match r {
        Ok(res) => {
            let status = res.status().as_u16() == 401;
            let text = res.text().await.unwrap_or_default();
            status && text.contains("Authentication required")
        }
        Err(_) => false,
    };
    t24 {
        name: "post_add_device_401_no_session".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("POST /add-device without session must return 401".into())
        },
    }
}

async fn get_manifest_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/manifest.json", base)).send().await;
    let ok = match r {
        Ok(res) => {
            let status = res.status() == 200;
            let ct = res
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");
            status && (ct.contains("manifest") || ct.contains("json"))
        }
        Err(_) => false,
    };
    t24 {
        name: "get_manifest_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET /manifest.json 200 + manifest content-type".into())
        },
    }
}

async fn get_manifest_valid_json(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/manifest.json", base)).send().await;
    let ok = match r {
        Ok(res) => {
            let text = res.text().await.unwrap_or_default();
            let parsed: Result<serde_json::Value, _> = serde_json::from_str(&text);
            let has_name = parsed
                .as_ref()
                .ok()
                .and_then(|v| v.get("name"))
                .map(|v| v.is_string())
                .unwrap_or(false);
            let has_icons = parsed
                .as_ref()
                .ok()
                .and_then(|v| v.get("icons"))
                .map(|v| v.is_array())
                .unwrap_or(false);
            parsed.is_ok() && has_name && has_icons
        }
        Err(_) => false,
    };
    t24 {
        name: "get_manifest_valid_json".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("manifest must be valid JSON with name and icons".into())
        },
    }
}

async fn get_not_found_404(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client
        .get(format!("{}/nonexistent-path-xyz", base))
        .send()
        .await;
    let ok = match r {
        Ok(res) => res.status().as_u16() == 404,
        Err(_) => false,
    };
    t24 {
        name: "get_not_found_404".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("unknown path must return 404".into())
        },
    }
}

async fn get_health_json_ok(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/health", base)).send().await;
    let ok = match r {
        Ok(res) => {
            let text = res.text().await.unwrap_or_default();
            let parsed: Result<serde_json::Value, _> = serde_json::from_str(&text);
            let ok_field = parsed
                .as_ref()
                .ok()
                .and_then(|v| v.get("ok"))
                .and_then(|v| v.as_bool());
            parsed.is_ok() && ok_field == Some(true)
        }
        Err(_) => false,
    };
    t24 {
        name: "get_health_json_ok".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("health must return JSON with ok:true".into())
        },
    }
}

async fn get_asset_404(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client
        .get(format!("{}/assets/nonexistent-file.xyz", base))
        .send()
        .await;
    let ok = match r {
        Ok(res) => res.status().as_u16() == 404,
        Err(_) => false,
    };
    t24 {
        name: "get_asset_404".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("nonexistent asset must return 404".into())
        },
    }
}

async fn get_sw_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/sw.js", base)).send().await;
    let ok = match r {
        Ok(res) => res.status() == 200,
        Err(_) => false,
    };
    t24 {
        name: "get_sw_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET /sw.js 200".into())
        },
    }
}

async fn get_icon_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client
        .get(format!("{}/assets/icon-192.svg", base))
        .send()
        .await;
    let ok = match r {
        Ok(res) => res.status() == 200,
        Err(_) => false,
    };
    t24 {
        name: "get_icon_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET /assets/icon-192.svg 200".into())
        },
    }
}

async fn get_app_image_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client
        .get(format!("{}/assets/apps/rogue-runner.webp", base))
        .send()
        .await;
    let ok = match r {
        Ok(res) => {
            let status = res.status() == 200;
            let ct = res
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");
            status && (ct.contains("image") || ct.contains("webp"))
        }
        Err(_) => false,
    };
    t24 {
        name: "get_app_image_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET /assets/apps/rogue-runner.webp 200 + image content-type".into())
        },
    }
}

async fn get_rogue_runner_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client
        .get(format!("{}/apps/rogue-runner", base))
        .send()
        .await;
    let ok = match r {
        Ok(res) => {
            let status = res.status() == 200;
            let ct: String = res
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .into();
            let body = res.text().await.unwrap_or_default();
            status
                && (ct.contains("text/html") || ct.contains("html"))
                && body.contains("Rogue Runner")
        }
        Err(_) => false,
    };
    t24 {
        name: "get_rogue_runner_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET /apps/rogue-runner 200 + HTML content-type".into())
        },
    }
}

async fn get_rogue_runner_wasm_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client
        .get(format!("{}/apps/rogue-runner-wasm", base))
        .send()
        .await;
    let ok = match r {
        Ok(res) => {
            let status = res.status() == 200;
            let ct = res
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_string();
            let body = res.text().await.unwrap_or_default();
            status && (ct.contains("text/html") || ct.contains("html")) && body.contains("glcanvas")
        }
        Err(_) => false,
    };
    t24 {
        name: "get_rogue_runner_wasm_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET /apps/rogue-runner-wasm 200 + HTML".into())
        },
    }
}

async fn get_index_contains_economy(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/", base)).send().await;
    let ok = match r {
        Ok(res) => {
            let t = res.text().await.unwrap_or_default();
            t.contains("100 Rogue Bucks") && t.contains("420")
        }
        Err(_) => false,
    };
    t24 {
        name: "get_index_contains_economy".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET / must show economy table".into())
        },
    }
}

async fn get_null_terminal_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client
        .get(format!("{}/apps/null-terminal", base))
        .send()
        .await;
    let ok = match r {
        Ok(res) => {
            let status = res.status() == 200;
            let ct = res
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_string();
            let body = res.text().await.unwrap_or_default();
            status && ct.contains("text/html") && body.contains("Null Terminal")
        }
        Err(_) => false,
    };
    t24 {
        name: "get_null_terminal_200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET /apps/null-terminal 200 + HTML + Null Terminal".into())
        },
    }
}

async fn get_index_shows_login_not_logout(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/", base)).send().await;
    let ok = match r {
        Ok(res) => {
            let t = res.text().await.unwrap_or_default();
            t.contains("/login") && !t.contains("/logout")
        }
        Err(_) => false,
    };
    t24 {
        name: "get_index_shows_login_not_logout".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("GET / without session must show Login, not Logout".into())
        },
    }
}

async fn post_invalid_json_returns_4xx(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client
        .post(format!("{}/buy-bucks", base))
        .header("Content-Type", "application/json")
        .body("{ invalid json }")
        .send()
        .await;
    let ok = match r {
        Ok(res) => {
            let s = res.status().as_u16();
            // 401 (no session) or 400/422 (bad json) — both are correct rejections
            s == 400 || s == 401 || s == 422
        }
        Err(_) => false,
    };
    t24 {
        name: "post_invalid_json_returns_4xx".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("invalid JSON must return 4xx".into())
        },
    }
}

async fn post_buy_bucks_no_session_401(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let body = serde_json::json!({ "pan_encrypted": [] });
    let r = client
        .post(format!("{}/buy-bucks", base))
        .json(&body)
        .send()
        .await;
    let ok = match r {
        Ok(res) => {
            let s = res.status().as_u16();
            // 401 (no session) is the expected first rejection
            s == 401 || s == 422
        }
        Err(_) => false,
    };
    t24 {
        name: "post_buy_bucks_no_session_or_missing_field".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("missing session or field must return 401 or 422".into())
        },
    }
}

// ---------------------------------------------------------------------------
// New: expanded HTTP test coverage
// ---------------------------------------------------------------------------

async fn get_login_page_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/login", base)).send().await;
    let ok = match r {
        Ok(res) => {
            let status = res.status() == 200;
            let ct = res.headers().get("content-type").and_then(|v| v.to_str().ok()).unwrap_or("").to_string();
            let body = res.text().await.unwrap_or_default();
            status && ct.contains("text/html") && (body.contains("login") || body.contains("Login") || body.contains("email"))
        }
        Err(_) => false,
    };
    t24 { name: "get_login_page_200".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("GET /login → 200 + HTML".into()) } }
}

async fn get_register_page_200(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/register", base)).send().await;
    let ok = match r {
        Ok(res) => {
            let status = res.status() == 200;
            let ct = res.headers().get("content-type").and_then(|v| v.to_str().ok()).unwrap_or("").to_string();
            let body = res.text().await.unwrap_or_default();
            status && ct.contains("text/html") && (body.contains("register") || body.contains("Register") || body.contains("email"))
        }
        Err(_) => false,
    };
    t24 { name: "get_register_page_200".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("GET /register → 200 + HTML".into()) } }
}

async fn get_asset_path_traversal_blocked(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let attacks = [
        "/assets/../../Cargo.toml",
        "/assets/../src/main.rs",
        "/assets/..%2F..%2FCargo.toml",
    ];
    let mut ok = true;
    for path in &attacks {
        let r = client.get(format!("{}{}", base, path)).send().await;
        match r {
            Ok(res) => {
                let status = res.status().as_u16();
                if status == 200 {
                    let body = res.text().await.unwrap_or_default();
                    // Must not return Cargo.toml or Rust source
                    if body.contains("[package]") || body.contains("fn main") {
                        ok = false;
                    }
                }
            }
            Err(_) => {} // connection error is fine
        }
    }
    t24 { name: "get_asset_path_traversal_blocked".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("path traversal must not leak source files".into()) } }
}

async fn get_index_contains_pixel_forge(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/", base)).send().await;
    let ok = match r {
        Ok(res) => {
            let t = res.text().await.unwrap_or_default();
            t.contains("pixel-forge") || t.contains("Pixel Forge")
        }
        Err(_) => false,
    };
    t24 { name: "get_index_contains_pixel_forge".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("index must reference Pixel Forge dependency".into()) } }
}

async fn get_index_has_coming_soon(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/", base)).send().await;
    let ok = match r {
        Ok(res) => res.text().await.unwrap_or_default().contains("Coming Soon"),
        Err(_) => false,
    };
    t24 { name: "get_index_has_coming_soon".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("index must show Coming Soon for blocked features".into()) } }
}

async fn get_health_content_type_json(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/health", base)).send().await;
    let ok = match r {
        Ok(res) => {
            let ct = res.headers().get("content-type").and_then(|v| v.to_str().ok()).unwrap_or("");
            ct.contains("json")
        }
        Err(_) => false,
    };
    t24 { name: "get_health_content_type_json".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("/health must return JSON content-type".into()) } }
}

async fn get_sw_content_type_js(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/sw.js", base)).send().await;
    let ok = match r {
        Ok(res) => {
            let ct = res.headers().get("content-type").and_then(|v| v.to_str().ok()).unwrap_or("");
            ct.contains("javascript")
        }
        Err(_) => false,
    };
    t24 { name: "get_sw_content_type_js".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("/sw.js must return javascript content-type".into()) } }
}

async fn get_icon_svg_content_type(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/assets/icon-192.svg", base)).send().await;
    let ok = match r {
        Ok(res) => {
            let ct = res.headers().get("content-type").and_then(|v| v.to_str().ok()).unwrap_or("");
            ct.contains("svg") || ct.contains("xml")
        }
        Err(_) => false,
    };
    t24 { name: "get_icon_svg_content_type".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("icon-192.svg must return SVG content-type".into()) } }
}

async fn post_empty_body_returns_4xx(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client
        .post(format!("{}/buy-bucks", base))
        .header("Content-Type", "application/json")
        .body("")
        .send()
        .await;
    let ok = match r {
        Ok(res) => {
            let s = res.status().as_u16();
            s >= 400 && s < 500
        }
        Err(_) => false,
    };
    t24 { name: "post_empty_body_returns_4xx".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("empty POST body must return 4xx".into()) } }
}

async fn get_index_has_iso_8583_reference(client: &reqwest::Client, base: &str) -> t24 {
    let start = Instant::now();
    let r = client.get(format!("{}/", base)).send().await;
    let ok = match r {
        Ok(res) => res.text().await.unwrap_or_default().contains("ISO 8583"),
        Err(_) => false,
    };
    t24 { name: "get_index_has_iso_8583_reference".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("index must reference ISO 8583".into()) } }
}
