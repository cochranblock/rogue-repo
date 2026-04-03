// Copyright (c) 2026 The Cochran Block, LLC (Pending). All rights reserved.
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! ISO 8583 TCP client — send messages to bank/processor, receive responses.
//! f127 = send_and_receive (length-prefixed TCP round-trip)
//! f128 = connect (establish TCP connection to switch endpoint)
//!
//! Wire format: 2-byte big-endian length prefix + ISO 8583 message bytes.
//! Standard for most ISO 8583 TCP implementations (Postilion, Base24, etc).

use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

use super::error::E4;
use super::iso8583::{f18, t3, t31};

/// Default timeout for TCP operations (10 seconds)
const TCP_TIMEOUT_MS: u64 = 10_000;

/// Maximum response size (64KB — ISO 8583 messages are typically <1KB)
const MAX_RESPONSE_SIZE: usize = 65_536;

/// t39 = SwitchEndpoint — configurable bank/processor connection
#[derive(Debug, Clone)]
pub struct t39 {
    pub host: String,
    pub port: u16,
    pub timeout_ms: u64,
}

impl t39 {
    /// Read from env: SWITCH_HOST, SWITCH_PORT, SWITCH_TIMEOUT_MS
    pub fn from_env() -> Option<Self> {
        let host = std::env::var("SWITCH_HOST").ok()?;
        let port: u16 = std::env::var("SWITCH_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8583);
        let timeout_ms: u64 = std::env::var("SWITCH_TIMEOUT_MS")
            .ok()
            .and_then(|t| t.parse().ok())
            .unwrap_or(TCP_TIMEOUT_MS);
        Some(t39 {
            host,
            port,
            timeout_ms,
        })
    }

    fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// f128 = connect — establish TCP connection to switch endpoint.
pub async fn f128(endpoint: &t39) -> Result<TcpStream, E4> {
    let addr = endpoint.addr();
    let dur = Duration::from_millis(endpoint.timeout_ms);

    timeout(dur, TcpStream::connect(&addr))
        .await
        .map_err(|_| E4::Connection(format!("timeout connecting to {}", addr)))?
        .map_err(|e| E4::Connection(format!("{}: {}", addr, e)))
}

/// f127 = send_and_receive — send ISO 8583 message, receive response.
/// Wire format: 2-byte big-endian length prefix + message bytes.
/// Returns raw response bytes (caller parses via f18 or similar).
pub async fn f127(endpoint: &t39, msg: &t3) -> Result<Vec<u8>, E4> {
    let mut stream = f128(endpoint).await?;
    let dur = Duration::from_millis(endpoint.timeout_ms);

    // Send: 2-byte length prefix (big-endian) + message
    let len = msg.raw.len() as u16;
    let mut send_buf = Vec::with_capacity(2 + msg.raw.len());
    send_buf.extend_from_slice(&len.to_be_bytes());
    send_buf.extend_from_slice(&msg.raw);

    timeout(dur, stream.write_all(&send_buf))
        .await
        .map_err(|_| E4::Connection("timeout sending message".into()))?
        .map_err(|e| E4::Connection(format!("send: {}", e)))?;

    // Receive: 2-byte length prefix + response
    let mut len_buf = [0u8; 2];
    timeout(dur, stream.read_exact(&mut len_buf))
        .await
        .map_err(|_| E4::Connection("timeout reading response length".into()))?
        .map_err(|e| E4::Connection(format!("recv length: {}", e)))?;

    let resp_len = u16::from_be_bytes(len_buf) as usize;
    if resp_len == 0 || resp_len > MAX_RESPONSE_SIZE {
        return Err(E4::Connection(format!(
            "invalid response length: {}",
            resp_len
        )));
    }

    let mut resp_buf = vec![0u8; resp_len];
    timeout(dur, stream.read_exact(&mut resp_buf))
        .await
        .map_err(|_| E4::Connection("timeout reading response body".into()))?
        .map_err(|e| E4::Connection(format!("recv body: {}", e)))?;

    Ok(resp_buf)
}

/// f129 = send_and_parse — send ISO 8583 purchase, parse 0210 response.
/// Convenience wrapper: f127 + f18 in one call.
pub async fn f129(endpoint: &t39, msg: &t3) -> Result<t31, E4> {
    let raw = f127(endpoint, msg).await?;
    f18(&raw)
}
