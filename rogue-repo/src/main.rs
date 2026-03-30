// Copyright (c) 2026 The Cochran Block, LLC (Pending). All rights reserved.
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! rogue-repo: f0=main. Zero-downtime hot reload via SO_REUSEPORT + PID lockfile.

use approuter::{f116, RegisterConfig};
use axum::{
    routing::{get, post},
    Router,
};
use rogue_repo::{auth, downloads, pwa, routes, vault};
use std::net::SocketAddr;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// PID lockfile directory
fn pid_dir() -> std::path::PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("rogue-repo")
}

/// Read old PID from lockfile
fn read_old_pid() -> Option<u32> {
    let path = pid_dir().join("pid");
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
}

/// Write current PID to lockfile
fn write_pid() {
    let dir = pid_dir();
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(dir.join("pid"), std::process::id().to_string());
}

/// SIGTERM old process, wait up to 5s, SIGKILL if still alive
fn retire_old(pid: u32) {
    use std::time::{Duration, Instant};

    // Don't kill ourselves
    if pid == std::process::id() {
        return;
    }

    // Check if process exists
    let alive = unsafe { libc::kill(pid as i32, 0) } == 0;
    if !alive {
        tracing::info!("Old PID {} already gone", pid);
        return;
    }

    tracing::info!("SIGTERM → old PID {}", pid);
    unsafe {
        libc::kill(pid as i32, libc::SIGTERM);
    }

    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        std::thread::sleep(Duration::from_millis(100));
        let still_alive = unsafe { libc::kill(pid as i32, 0) } == 0;
        if !still_alive {
            tracing::info!("Old PID {} exited cleanly", pid);
            return;
        }
        if Instant::now() >= deadline {
            break;
        }
    }

    tracing::warn!("SIGKILL → old PID {} (did not exit in 5s)", pid);
    unsafe {
        libc::kill(pid as i32, libc::SIGKILL);
    }
}

/// Bind with SO_REUSEPORT for overlapping listen during hot reload
fn bind_reuseport(addr: SocketAddr) -> std::io::Result<std::net::TcpListener> {
    use socket2::{Domain, Protocol, Socket, Type};

    let domain = if addr.is_ipv4() {
        Domain::IPV4
    } else {
        Domain::IPV6
    };
    let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;
    socket.set_reuse_address(true)?;
    socket.set_reuse_port(true)?;
    socket.set_nonblocking(true)?;
    socket.bind(&addr.into())?;
    socket.listen(1024)?;
    Ok(std::net::TcpListener::from(socket))
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // --- Hot reload: read old PID before binding ---
    let old_pid = read_old_pid();

    let pool = std::env::var("DATABASE_URL").ok().and_then(|url| {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                sqlx::postgres::PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&url)
                    .await
                    .ok()
            })
        })
    });

    if pool.is_none() {
        tracing::warn!("DATABASE_URL not set or invalid; auth disabled");
    }

    let p0 = routes::t0 { s0: pool };
    let app = Router::new()
        .route("/", get(routes::f4))
        .route("/login", get(pwa::f102).post(auth::f98))
        .route("/register", get(pwa::f103).post(auth::f97))
        .route("/verify-email", get(auth::f100))
        .route("/logout", axum::routing::post(auth::f101).get(auth::f101))
        .route("/manifest.json", get(pwa::f92))
        .route("/sw.js", get(pwa::f93))
        .route("/assets/*path", get(pwa::f91))
        .route("/apps/rogue-runner", get(pwa::f94))
        .route("/apps/rogue-runner-wasm", get(pwa::f95))
        .route("/apps/null-terminal", get(pwa::f117))
        .route("/health", get(routes::f5))
        .route("/buy-bucks", post(routes::f87))
        .route("/provision-app", post(routes::f88))
        .route("/add-device", post(routes::f89))
        .route("/downloads/rogue-runner", get(downloads::f118))
        .with_state(p0)
        .layer(CompressionLayer::new())
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any));

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3001);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // --- Bind with SO_REUSEPORT (overlaps with old instance) ---
    let std_listener = bind_reuseport(addr).unwrap_or_else(|e| {
        tracing::error!("Failed to bind {}: {}. Falling back to standard bind.", addr, e);
        let l = std::net::TcpListener::bind(addr).expect("bind failed");
        l.set_nonblocking(true).expect("set_nonblocking");
        l
    });
    let listener = tokio::net::TcpListener::from_std(std_listener).expect("tokio listener");

    // --- Register with approuter ---
    f116(RegisterConfig {
        app_id: "roguerepo",
        hostnames: std::env::var("REPO_HOSTNAMES")
            .unwrap_or_else(|_| "roguerepo.io,www.roguerepo.io".into())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        backend_url: std::env::var("REPO_BACKEND_URL")
            .unwrap_or_else(|_| format!("http://127.0.0.1:{}", port)),
    })
    .await;

    // --- Write PID, then retire old instance ---
    write_pid();
    if let Some(pid) = old_pid {
        retire_old(pid);
    }

    tracing::info!("Rogue Repo API on {} (PID {})", addr, std::process::id());
    axum::serve(listener, app).await.unwrap();
}
