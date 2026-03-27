// Copyright (c) 2026 The Cochran Block, LLC (Pending). All rights reserved.
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! ISO 8583 message types.
//! f12 = build_0200 (purchase request)
//! f17 = build_0100 (authorization request)
//! f18 = parse_0210 (purchase response)
//! f19 = build_0400 (reversal)

use bitvec::prelude::*;
use bytes::BytesMut;
use std::io::Write;

use super::E4;

const MTI_0100: &[u8] = b"0100";
const MTI_0200: &[u8] = b"0200";
const MTI_0210: &[u8] = b"0210";
const MTI_0400: &[u8] = b"0400";

/// t2 = PurchaseRequest (used by MTI 0200)
#[derive(Debug, Clone)]
pub struct t2 {
    pub pan_encrypted: Vec<u8>,
    pub amount_cents: u64,
    pub stan: u32,
}

impl Default for t2 {
    fn default() -> Self {
        Self {
            pan_encrypted: Vec::new(),
            amount_cents: 420,
            stan: 0,
        }
    }
}

/// t3 = Iso8583Message (raw wire bytes)
#[derive(Debug)]
pub struct t3 {
    pub raw: Vec<u8>,
}

/// t30 = AuthRequest (MTI 0100 — authorization, hold funds without capture)
#[derive(Debug, Clone)]
pub struct t30 {
    pub pan_encrypted: Vec<u8>,
    pub amount_cents: u64,
    pub stan: u32,
    pub merchant_id: [u8; 15],
}

impl Default for t30 {
    fn default() -> Self {
        Self {
            pan_encrypted: Vec::new(),
            amount_cents: 420,
            stan: 0,
            merchant_id: *b"ROGUEREPO000000",
        }
    }
}

/// t31 = PurchaseResponse (parsed MTI 0210)
#[derive(Debug, Clone)]
pub struct t31 {
    pub mti: [u8; 4],
    pub response_code: [u8; 2],
    pub stan: u32,
    pub amount_cents: u64,
    pub approved: bool,
    pub auth_code: Option<[u8; 6]>,
}

/// t32 = ReversalRequest (MTI 0400 — undo a previous 0200 or 0100)
#[derive(Debug, Clone)]
pub struct t32 {
    pub pan_encrypted: Vec<u8>,
    pub amount_cents: u64,
    pub original_stan: u32,
    pub reversal_stan: u32,
    pub reason: t34,
}

/// t34 = ReversalReason
#[derive(Debug, Clone, Copy)]
pub enum t34 {
    Timeout,
    CustomerCancel,
    SystemError,
}

fn validate_amount(cents: u64) -> Result<(), E4> {
    if cents == 0 || cents > 99_999_999 {
        return Err(E4::Amount("1-99999999 cents".into()));
    }
    Ok(())
}

fn validate_pan(pan: &[u8]) -> Result<(), E4> {
    if pan.len() > 99 {
        return Err(E4::Pack("PAN too long".into()));
    }
    Ok(())
}

fn write_pan(out: &mut Vec<u8>, pan: &[u8]) -> Result<(), E4> {
    let mut pan_buf = BytesMut::new();
    pan_buf.extend_from_slice(format!("{:02}", pan.len()).as_bytes());
    pan_buf.extend_from_slice(pan);
    out.write_all(&pan_buf)
        .map_err(|e| E4::Pack(e.to_string()))
}

fn build_bitmap(fields: &[usize]) -> Result<Vec<u8>, E4> {
    let mut bitmap = bitvec![u8, Msb0; 0u8; 64];
    for &f in fields {
        bitmap.set(f, true);
    }
    let bytes: Vec<u8> = bitmap.as_raw_slice().to_vec();
    if bytes.len() != 8 {
        return Err(E4::Pack("bitmap 8 bytes".into()));
    }
    Ok(bytes)
}

fn now_fields() -> (String, String) {
    let now = chrono::Utc::now();
    (
        now.format("%H%M%S").to_string(),
        now.format("%m%d").to_string(),
    )
}

// ---------------------------------------------------------------------------
// MTI 0200 — Purchase Request (financial transaction)
// ---------------------------------------------------------------------------

/// f12 = build_0200: purchase request
/// Fields: 2(PAN), 3(proc code), 4(amount), 11(STAN), 12(time), 13(date), 49(currency)
pub fn f12(req: &t2) -> Result<t3, E4> {
    validate_amount(req.amount_cents)?;
    validate_pan(&req.pan_encrypted)?;

    let (time_str, date_str) = now_fields();
    let amount_str = format!("{:012}", req.amount_cents);

    let bitmap_bytes = build_bitmap(&[1, 2, 3, 10, 11, 12, 48])?;

    let mut out = Vec::new();
    out.write_all(MTI_0200)
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(&bitmap_bytes)
        .map_err(|e| E4::Pack(e.to_string()))?;
    write_pan(&mut out, &req.pan_encrypted)?;
    out.write_all(b"000000")
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(amount_str.as_bytes())
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(format!("{:06}", req.stan).as_bytes())
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(time_str.as_bytes())
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(date_str.as_bytes())
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(b"840")
        .map_err(|e| E4::Pack(e.to_string()))?;

    Ok(t3 { raw: out })
}

// ---------------------------------------------------------------------------
// MTI 0100 — Authorization Request (hold funds, no capture)
// ---------------------------------------------------------------------------

/// f17 = build_0100: authorization request
/// Fields: 2(PAN), 3(proc code), 4(amount), 11(STAN), 12(time), 13(date), 42(merchant), 49(currency)
pub fn f17(req: &t30) -> Result<t3, E4> {
    validate_amount(req.amount_cents)?;
    validate_pan(&req.pan_encrypted)?;

    let (time_str, date_str) = now_fields();
    let amount_str = format!("{:012}", req.amount_cents);

    // Bitmap fields: 2,3,4,11,12,13,42,49 (0-indexed bit positions: 1,2,3,10,11,12,41,48)
    let bitmap_bytes = build_bitmap(&[1, 2, 3, 10, 11, 12, 41, 48])?;

    let mut out = Vec::new();
    out.write_all(MTI_0100)
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(&bitmap_bytes)
        .map_err(|e| E4::Pack(e.to_string()))?;
    write_pan(&mut out, &req.pan_encrypted)?;
    out.write_all(b"000000")
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(amount_str.as_bytes())
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(format!("{:06}", req.stan).as_bytes())
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(time_str.as_bytes())
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(date_str.as_bytes())
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(&req.merchant_id)
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(b"840")
        .map_err(|e| E4::Pack(e.to_string()))?;

    Ok(t3 { raw: out })
}

// ---------------------------------------------------------------------------
// MTI 0210 — Purchase Response (parse incoming response from bank)
// ---------------------------------------------------------------------------

/// f18 = parse_0210: parse purchase response from bank
/// Expects: MTI(4) + bitmap(8) + response_code(2) + amount(12) + STAN(6) + auth_code(6, optional)
pub fn f18(raw: &[u8]) -> Result<t31, E4> {
    // Minimum: MTI(4) + bitmap(8) + response_code(2) + amount(12) + STAN(6) = 32
    if raw.len() < 32 {
        return Err(E4::Unpack(format!(
            "0210 too short: {} bytes, need >= 32",
            raw.len()
        )));
    }

    let mti = &raw[0..4];
    if mti != MTI_0210 {
        return Err(E4::Unpack(format!(
            "expected MTI 0210, got {}",
            String::from_utf8_lossy(mti)
        )));
    }

    let _bitmap = &raw[4..12];

    let mut pos = 12;

    // Field 39: response code (2 bytes)
    if pos + 2 > raw.len() {
        return Err(E4::Unpack("missing response code".into()));
    }
    let response_code: [u8; 2] = [raw[pos], raw[pos + 1]];
    pos += 2;

    // Field 4: amount (12 bytes)
    if pos + 12 > raw.len() {
        return Err(E4::Unpack("missing amount".into()));
    }
    let amount_str = String::from_utf8_lossy(&raw[pos..pos + 12]);
    let amount_cents: u64 = amount_str
        .parse()
        .map_err(|_| E4::Unpack(format!("invalid amount: {}", amount_str)))?;
    pos += 12;

    // Field 11: STAN (6 bytes)
    if pos + 6 > raw.len() {
        return Err(E4::Unpack("missing STAN".into()));
    }
    let stan_str = String::from_utf8_lossy(&raw[pos..pos + 6]);
    let stan: u32 = stan_str
        .parse()
        .map_err(|_| E4::Unpack(format!("invalid STAN: {}", stan_str)))?;
    pos += 6;

    // Field 38: auth code (6 bytes, optional — present on approval)
    let auth_code = if pos + 6 <= raw.len() {
        let mut code = [0u8; 6];
        code.copy_from_slice(&raw[pos..pos + 6]);
        Some(code)
    } else {
        None
    };

    let approved = &response_code == b"00";

    Ok(t31 {
        mti: [raw[0], raw[1], raw[2], raw[3]],
        response_code,
        stan,
        amount_cents,
        approved,
        auth_code,
    })
}

// ---------------------------------------------------------------------------
// MTI 0400 — Reversal Request (undo a previous transaction)
// ---------------------------------------------------------------------------

/// f19 = build_0400: reversal request
/// Fields: 2(PAN), 3(proc code), 4(amount), 11(STAN), 12(time), 13(date), 37(original STAN), 49(currency)
pub fn f19(req: &t32) -> Result<t3, E4> {
    validate_amount(req.amount_cents)?;
    validate_pan(&req.pan_encrypted)?;

    let (time_str, date_str) = now_fields();
    let amount_str = format!("{:012}", req.amount_cents);

    let reason_code = match req.reason {
        t34::Timeout => b"4021",
        t34::CustomerCancel => b"4000",
        t34::SystemError => b"4005",
    };

    // Bitmap fields: 2,3,4,11,12,13,37,49,56 (0-indexed: 1,2,3,10,11,12,36,48,55)
    let bitmap_bytes = build_bitmap(&[1, 2, 3, 10, 11, 12, 36, 48, 55])?;

    let mut out = Vec::new();
    out.write_all(MTI_0400)
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(&bitmap_bytes)
        .map_err(|e| E4::Pack(e.to_string()))?;
    write_pan(&mut out, &req.pan_encrypted)?;
    out.write_all(b"000000")
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(amount_str.as_bytes())
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(format!("{:06}", req.reversal_stan).as_bytes())
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(time_str.as_bytes())
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(date_str.as_bytes())
        .map_err(|e| E4::Pack(e.to_string()))?;
    // Field 37: original data — encode original STAN as retrieval reference
    out.write_all(format!("{:012}", req.original_stan).as_bytes())
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(b"840")
        .map_err(|e| E4::Pack(e.to_string()))?;
    // Field 56: reason code
    out.write_all(reason_code)
        .map_err(|e| E4::Pack(e.to_string()))?;

    Ok(t3 { raw: out })
}
