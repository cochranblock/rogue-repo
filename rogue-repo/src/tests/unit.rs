// Copyright (c) 2026 The Cochran Block, LLC (Pending). All rights reserved.
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! f49 = unit tests: vault round-trip, tamper rejection; switch f12

use std::time::Instant;

use crate::switch::{f12, f17, f18, f19, t2, t30, t32, t34};
use crate::vault::{f10, f11, t1};

use crate::tests::t24;

pub fn f49() -> Vec<t24> {
    vec![
        vault_round_trip(),
        vault_tamper_rejected(),
        vault_wrong_key(),
        vault_nonce_uniqueness(),
        vault_decrypt_short_fails(),
        vault_empty_plaintext(),
        switch_build_0200(),
        switch_invalid_amount(),
        switch_amount_in_output(),
        switch_currency_840_in_output(),
        switch_amount_out_of_range_fails(),
        switch_pan_too_long_fails(),
        // MTI 0100 — authorization request
        switch_build_0100(),
        switch_0100_has_merchant_id(),
        switch_0100_invalid_amount(),
        // MTI 0210 — purchase response parsing
        switch_parse_0210_approved(),
        switch_parse_0210_declined(),
        switch_parse_0210_with_auth_code(),
        switch_parse_0210_too_short(),
        switch_parse_0210_wrong_mti(),
        // MTI 0400 — reversal
        switch_build_0400(),
        switch_0400_has_original_stan(),
        switch_0400_reason_codes(),
        switch_0400_invalid_amount(),
    ]
}

fn vault_round_trip() -> t24 {
    let start = Instant::now();
    let key = [0u8; 32];
    let v = match t1::new(&key) {
        Ok(x) => x,
        Err(e) => {
            return t24 {
                name: "vault_round_trip".into(),
                passed: false,
                duration_ms: start.elapsed().as_millis() as u64,
                message: Some(format!("vault new: {}", e)),
            };
        }
    };
    let plain = b"4111111111111111";
    let enc = match f10(&v, plain) {
        Ok(x) => x,
        Err(e) => {
            return t24 {
                name: "vault_round_trip".into(),
                passed: false,
                duration_ms: start.elapsed().as_millis() as u64,
                message: Some(format!("encrypt: {}", e)),
            };
        }
    };
    let dec = match f11(&v, &enc) {
        Ok(x) => x,
        Err(e) => {
            return t24 {
                name: "vault_round_trip".into(),
                passed: false,
                duration_ms: start.elapsed().as_millis() as u64,
                message: Some(format!("decrypt: {}", e)),
            };
        }
    };
    let ok = dec == plain;
    t24 {
        name: "vault_round_trip".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("decrypted != plain".into())
        },
    }
}

fn vault_tamper_rejected() -> t24 {
    let start = Instant::now();
    let key = [0u8; 32];
    let v = t1::new(&key).unwrap();
    let plain = b"4111111111111111";
    let mut enc = f10(&v, plain).unwrap();
    if enc.len() > 20 {
        enc[20] = enc[20].wrapping_add(1);
    }
    let dec = f11(&v, &enc);
    let ok = dec.is_err();
    t24 {
        name: "vault_tamper_rejected".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("tampered ciphertext should fail".into())
        },
    }
}

fn vault_wrong_key() -> t24 {
    let start = Instant::now();
    let key1 = [0u8; 32];
    let key2 = [1u8; 32];
    let v1 = t1::new(&key1).unwrap();
    let v2 = t1::new(&key2).unwrap();
    let plain = b"4111111111111111";
    let enc = f10(&v1, plain).unwrap();
    let dec = f11(&v2, &enc);
    let ok = dec.is_err();
    t24 {
        name: "vault_wrong_key".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("wrong key should fail".into())
        },
    }
}

fn switch_build_0200() -> t24 {
    let start = Instant::now();
    let req = t2 {
        pan_encrypted: b"encrypted_pan_data".to_vec(),
        amount_cents: 420,
        stan: 12345,
    };
    let msg = match f12(&req) {
        Ok(m) => m,
        Err(e) => {
            return t24 {
                name: "switch_build_0200".into(),
                passed: false,
                duration_ms: start.elapsed().as_millis() as u64,
                message: Some(format!("f12: {}", e)),
            };
        }
    };
    let raw = &msg.raw;
    let has_mti = raw.starts_with(b"0200");
    let has_bitmap = raw.len() >= 12 && raw[4..12].iter().any(|&b| b != 0);
    let has_amount = raw.windows(12).any(|w| w == b"000000000420");
    let ok = has_mti && has_bitmap && has_amount;
    t24 {
        name: "switch_build_0200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("MTI+bitmap+amount in 0200".into())
        },
    }
}

fn vault_nonce_uniqueness() -> t24 {
    let start = Instant::now();
    let key = [0u8; 32];
    let v = t1::new(&key).unwrap();
    let plain = b"4111111111111111";
    let enc1 = f10(&v, plain).unwrap();
    let enc2 = f10(&v, plain).unwrap();
    let ok = enc1 != enc2;
    t24 {
        name: "vault_nonce_uniqueness".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("same plaintext must produce different ciphertext".into())
        },
    }
}

fn vault_decrypt_short_fails() -> t24 {
    let start = Instant::now();
    let key = [0u8; 32];
    let v = t1::new(&key).unwrap();
    let short = [0u8; 8];
    let r = f11(&v, &short);
    let ok = r.is_err();
    t24 {
        name: "vault_decrypt_short_fails".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("ciphertext < nonce len must fail".into())
        },
    }
}

fn vault_empty_plaintext() -> t24 {
    let start = Instant::now();
    let key = [0u8; 32];
    let v = t1::new(&key).unwrap();
    let enc = f10(&v, &[]).unwrap();
    let dec = f11(&v, &enc).unwrap();
    let ok = dec.is_empty();
    t24 {
        name: "vault_empty_plaintext".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("encrypt/decrypt empty round-trip".into())
        },
    }
}

fn switch_invalid_amount() -> t24 {
    let start = Instant::now();
    let req = t2 {
        pan_encrypted: vec![],
        amount_cents: 0,
        stan: 0,
    };
    let r = f12(&req);
    let ok = r.is_err();
    t24 {
        name: "switch_invalid_amount".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("amount 0 should fail".into())
        },
    }
}

fn switch_amount_in_output() -> t24 {
    let start = Instant::now();
    let req = t2 {
        pan_encrypted: vec![1, 2, 3],
        amount_cents: 420,
        stan: 999999,
    };
    let msg = match f12(&req) {
        Ok(m) => m,
        Err(e) => {
            return t24 {
                name: "switch_amount_in_output".into(),
                passed: false,
                duration_ms: start.elapsed().as_millis() as u64,
                message: Some(format!("f12: {}", e)),
            };
        }
    };
    let raw = String::from_utf8_lossy(&msg.raw);
    let ok = raw.contains("000000000420");
    t24 {
        name: "switch_amount_in_output".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("amount 420 must appear as 12-digit field".into())
        },
    }
}

fn switch_currency_840_in_output() -> t24 {
    let start = Instant::now();
    let req = t2 {
        pan_encrypted: vec![],
        amount_cents: 420,
        stan: 0,
    };
    let msg = f12(&req).unwrap();
    let ok = msg.raw.ends_with(b"840");
    t24 {
        name: "switch_currency_840_in_output".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("currency 840 USD must be last field".into())
        },
    }
}

fn switch_amount_out_of_range_fails() -> t24 {
    let start = Instant::now();
    let req = t2 {
        pan_encrypted: vec![],
        amount_cents: 999_999_999,
        stan: 0,
    };
    let r = f12(&req);
    let ok = r.is_err();
    t24 {
        name: "switch_amount_out_of_range_fails".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("amount > 99999999 must fail".into())
        },
    }
}

fn switch_pan_too_long_fails() -> t24 {
    let start = Instant::now();
    let req = t2 {
        pan_encrypted: vec![0; 100],
        amount_cents: 420,
        stan: 0,
    };
    let r = f12(&req);
    let ok = r.is_err();
    t24 {
        name: "switch_pan_too_long_fails".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("PAN > 99 bytes must fail".into())
        },
    }
}

// ---------------------------------------------------------------------------
// MTI 0100 — Authorization Request tests
// ---------------------------------------------------------------------------

fn switch_build_0100() -> t24 {
    let start = Instant::now();
    let req = t30 {
        pan_encrypted: b"encrypted_pan".to_vec(),
        amount_cents: 1000,
        stan: 54321,
        ..Default::default()
    };
    let msg = match f17(&req) {
        Ok(m) => m,
        Err(e) => {
            return t24 {
                name: "switch_build_0100".into(),
                passed: false,
                duration_ms: start.elapsed().as_millis() as u64,
                message: Some(format!("f17: {}", e)),
            };
        }
    };
    let raw = &msg.raw;
    let has_mti = raw.starts_with(b"0100");
    let has_bitmap = raw.len() >= 12 && raw[4..12].iter().any(|&b| b != 0);
    let has_amount = raw.windows(12).any(|w| w == b"000000001000");
    let ok = has_mti && has_bitmap && has_amount;
    t24 {
        name: "switch_build_0100".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("MTI 0100 + bitmap + amount".into())
        },
    }
}

fn switch_0100_has_merchant_id() -> t24 {
    let start = Instant::now();
    let req = t30 {
        pan_encrypted: vec![1, 2, 3],
        amount_cents: 420,
        stan: 1,
        merchant_id: *b"ROGUEREPO000000",
    };
    let msg = f17(&req).unwrap();
    let ok = msg
        .raw
        .windows(15)
        .any(|w| w == b"ROGUEREPO000000");
    t24 {
        name: "switch_0100_has_merchant_id".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("0100 must contain merchant ID".into())
        },
    }
}

fn switch_0100_invalid_amount() -> t24 {
    let start = Instant::now();
    let req = t30 {
        amount_cents: 0,
        ..Default::default()
    };
    let ok = f17(&req).is_err();
    t24 {
        name: "switch_0100_invalid_amount".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("0100 amount 0 must fail".into())
        },
    }
}

// ---------------------------------------------------------------------------
// MTI 0210 — Purchase Response parsing tests
// ---------------------------------------------------------------------------

fn build_0210_raw(response_code: &[u8; 2], amount_cents: u64, stan: u32, auth_code: Option<&[u8; 6]>) -> Vec<u8> {
    let mut raw = Vec::new();
    raw.extend_from_slice(b"0210");
    raw.extend_from_slice(&[0u8; 8]); // bitmap placeholder
    raw.extend_from_slice(response_code);
    raw.extend_from_slice(format!("{:012}", amount_cents).as_bytes());
    raw.extend_from_slice(format!("{:06}", stan).as_bytes());
    if let Some(code) = auth_code {
        raw.extend_from_slice(code);
    }
    raw
}

fn switch_parse_0210_approved() -> t24 {
    let start = Instant::now();
    let raw = build_0210_raw(b"00", 420, 12345, None);
    let res = match f18(&raw) {
        Ok(r) => r,
        Err(e) => {
            return t24 {
                name: "switch_parse_0210_approved".into(),
                passed: false,
                duration_ms: start.elapsed().as_millis() as u64,
                message: Some(format!("f18: {}", e)),
            };
        }
    };
    let ok = res.approved && res.amount_cents == 420 && res.stan == 12345 && res.response_code == *b"00";
    t24 {
        name: "switch_parse_0210_approved".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("0210 response code 00 = approved".into())
        },
    }
}

fn switch_parse_0210_declined() -> t24 {
    let start = Instant::now();
    let raw = build_0210_raw(b"05", 420, 99999, None);
    let res = f18(&raw).unwrap();
    let ok = !res.approved && res.response_code == *b"05" && res.stan == 99999;
    t24 {
        name: "switch_parse_0210_declined".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("0210 response code 05 = declined".into())
        },
    }
}

fn switch_parse_0210_with_auth_code() -> t24 {
    let start = Instant::now();
    let raw = build_0210_raw(b"00", 1000, 55555, Some(b"A12345"));
    let res = f18(&raw).unwrap();
    let ok = res.approved && res.auth_code == Some(*b"A12345") && res.amount_cents == 1000;
    t24 {
        name: "switch_parse_0210_with_auth_code".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("0210 with auth code must parse it".into())
        },
    }
}

fn switch_parse_0210_too_short() -> t24 {
    let start = Instant::now();
    let ok = f18(b"0210short").is_err();
    t24 {
        name: "switch_parse_0210_too_short".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("0210 < 32 bytes must fail".into())
        },
    }
}

fn switch_parse_0210_wrong_mti() -> t24 {
    let start = Instant::now();
    let mut raw = build_0210_raw(b"00", 420, 1, None);
    raw[0..4].copy_from_slice(b"0200");
    let ok = f18(&raw).is_err();
    t24 {
        name: "switch_parse_0210_wrong_mti".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("non-0210 MTI must fail".into())
        },
    }
}

// ---------------------------------------------------------------------------
// MTI 0400 — Reversal tests
// ---------------------------------------------------------------------------

fn switch_build_0400() -> t24 {
    let start = Instant::now();
    let req = t32 {
        pan_encrypted: b"encrypted_pan".to_vec(),
        amount_cents: 420,
        original_stan: 12345,
        reversal_stan: 12346,
        reason: t34::Timeout,
    };
    let msg = match f19(&req) {
        Ok(m) => m,
        Err(e) => {
            return t24 {
                name: "switch_build_0400".into(),
                passed: false,
                duration_ms: start.elapsed().as_millis() as u64,
                message: Some(format!("f19: {}", e)),
            };
        }
    };
    let raw = &msg.raw;
    let has_mti = raw.starts_with(b"0400");
    let has_bitmap = raw.len() >= 12 && raw[4..12].iter().any(|&b| b != 0);
    let has_amount = raw.windows(12).any(|w| w == b"000000000420");
    let ok = has_mti && has_bitmap && has_amount;
    t24 {
        name: "switch_build_0400".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("MTI 0400 + bitmap + amount".into())
        },
    }
}

fn switch_0400_has_original_stan() -> t24 {
    let start = Instant::now();
    let req = t32 {
        pan_encrypted: vec![],
        amount_cents: 420,
        original_stan: 54321,
        reversal_stan: 54322,
        reason: t34::CustomerCancel,
    };
    let msg = f19(&req).unwrap();
    let raw_str = String::from_utf8_lossy(&msg.raw);
    // Original STAN encoded as 12-digit retrieval reference
    let ok = raw_str.contains("000000054321");
    t24 {
        name: "switch_0400_has_original_stan".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("0400 must contain original STAN as retrieval ref".into())
        },
    }
}

fn switch_0400_reason_codes() -> t24 {
    let start = Instant::now();
    let base = t32 {
        pan_encrypted: vec![],
        amount_cents: 420,
        original_stan: 1,
        reversal_stan: 2,
        reason: t34::Timeout,
    };

    let msg_timeout = f19(&base).unwrap();
    let has_timeout = msg_timeout.raw.windows(4).any(|w| w == b"4021");

    let mut cancel = base.clone();
    cancel.reason = t34::CustomerCancel;
    let msg_cancel = f19(&cancel).unwrap();
    let has_cancel = msg_cancel.raw.windows(4).any(|w| w == b"4000");

    let mut syserr = base.clone();
    syserr.reason = t34::SystemError;
    let msg_syserr = f19(&syserr).unwrap();
    let has_syserr = msg_syserr.raw.windows(4).any(|w| w == b"4005");

    let ok = has_timeout && has_cancel && has_syserr;
    t24 {
        name: "switch_0400_reason_codes".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("0400 must encode reason: 4021/4000/4005".into())
        },
    }
}

fn switch_0400_invalid_amount() -> t24 {
    let start = Instant::now();
    let req = t32 {
        pan_encrypted: vec![],
        amount_cents: 0,
        original_stan: 1,
        reversal_stan: 2,
        reason: t34::Timeout,
    };
    let ok = f19(&req).is_err();
    t24 {
        name: "switch_0400_invalid_amount".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("0400 amount 0 must fail".into())
        },
    }
}
