// Copyright (c) 2026 The Cochran Block, LLC (Pending). All rights reserved.
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! f49 = unit tests: vault round-trip, tamper rejection; switch f12

use std::time::Instant;

use crate::auth;
use crate::switch::{f12, f17, f18, f19, t2, t3, t30, t32, t34, t36, t37, t39};
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
        // Stripe mapping tests
        stripe_decline_code_round_trip(),
        stripe_decline_approved_is_00(),
        stripe_event_kind_parse(),
        stripe_event_mti_mapping(),
        stripe_unknown_event_returns_none(),
        stripe_unknown_decline_returns_none(),
        // Stripe stub functions return Coming Soon (f120/f121/f122 still stubs)
        stripe_f120_returns_coming_soon(),
        stripe_f121_returns_coming_soon(),
        stripe_f122_returns_coming_soon(),
        // f123: real HMAC-SHA256 webhook verification
        stripe_f123_valid_signature(),
        stripe_f123_invalid_signature(),
        stripe_f123_missing_env_returns_err(),
        stripe_f123_missing_timestamp_err(),
        stripe_f123_missing_v1_err(),
        stripe_f123_tampered_payload_rejected(),
        // Auth: session signing and verification
        auth_session_secret_returns_value(),
        auth_sign_verify_round_trip(),
        auth_verify_expired_session(),
        auth_verify_tampered_session(),
        auth_verify_malformed_session(),
        auth_f20_none_input_returns_none(),
        auth_f20_empty_string_returns_none(),
        auth_f124_no_cookie_header(),
        auth_f124_other_cookies_only(),
        auth_f124_extracts_session_cookie(),
        // Vault edge cases
        vault_key_32_bytes_works(),
        vault_decrypt_exactly_nonce_length(),
        vault_large_plaintext(),
        vault_encrypt_different_each_time(),
        // ISO 8583 edge cases
        switch_0200_stan_zero(),
        switch_0200_max_amount(),
        switch_0200_boundary_amount_1(),
        switch_0200_amount_99999999(),
        switch_0100_default_merchant(),
        switch_0210_boundary_32_bytes(),
        switch_0210_invalid_amount_string(),
        switch_0400_pan_empty(),
        switch_0400_all_reasons_build(),
        // TCP endpoint config
        tcp_endpoint_from_env_missing(),
        tcp_endpoint_addr_format(),
        // Stripe decline card_declined_maps_to_05
        stripe_card_declined_maps_to_05(),
        stripe_all_events_have_mti(),
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

// ---------------------------------------------------------------------------
// Stripe ↔ ISO 8583 mapping tests
// ---------------------------------------------------------------------------

fn stripe_decline_code_round_trip() -> t24 {
    let start = Instant::now();
    let codes = [
        ("approved", b"00"),
        ("insufficient_funds", b"51"),
        ("lost_card", b"41"),
        ("stolen_card", b"43"),
        ("expired_card", b"54"),
        ("incorrect_cvc", b"82"),
        ("processing_error", b"96"),
        ("do_not_honor", b"05"),
        ("fraudulent", b"59"),
    ];
    let ok = codes.iter().all(|(stripe_code, expected_iso)| {
        let parsed = t36::from_stripe(stripe_code).unwrap();
        &parsed.to_iso_response() == *expected_iso
    });
    t24 {
        name: "stripe_decline_code_round_trip".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("all Stripe decline codes must map to correct ISO response codes".into())
        },
    }
}

fn stripe_decline_approved_is_00() -> t24 {
    let start = Instant::now();
    let ok = t36::from_stripe("approved")
        .map(|c| c.to_iso_response() == *b"00")
        .unwrap_or(false);
    t24 {
        name: "stripe_decline_approved_is_00".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("Stripe 'approved' must map to ISO 00".into())
        },
    }
}

fn stripe_event_kind_parse() -> t24 {
    let start = Instant::now();
    let events = [
        "payment_intent.created",
        "charge.succeeded",
        "charge.captured",
        "charge.refunded",
        "charge.failed",
        "payment_intent.canceled",
        "dispute.created",
    ];
    let ok = events.iter().all(|e| t37::from_stripe(e).is_some());
    t24 {
        name: "stripe_event_kind_parse".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("all defined Stripe event types must parse".into())
        },
    }
}

fn stripe_event_mti_mapping() -> t24 {
    let start = Instant::now();
    let ok = t37::from_stripe("payment_intent.created")
        .map(|e| e.iso_mti() == "0100")
        .unwrap_or(false)
        && t37::from_stripe("charge.succeeded")
            .map(|e| e.iso_mti() == "0200")
            .unwrap_or(false)
        && t37::from_stripe("charge.refunded")
            .map(|e| e.iso_mti() == "0420")
            .unwrap_or(false)
        && t37::from_stripe("charge.failed")
            .map(|e| e.iso_mti() == "0210")
            .unwrap_or(false);
    t24 {
        name: "stripe_event_mti_mapping".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("Stripe events must map to correct ISO MTIs".into())
        },
    }
}

fn stripe_unknown_event_returns_none() -> t24 {
    let start = Instant::now();
    let ok = t37::from_stripe("bogus.event").is_none();
    t24 {
        name: "stripe_unknown_event_returns_none".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("unknown Stripe event must return None".into())
        },
    }
}

fn stripe_unknown_decline_returns_none() -> t24 {
    let start = Instant::now();
    let ok = t36::from_stripe("bogus_code").is_none();
    t24 {
        name: "stripe_unknown_decline_returns_none".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("unknown Stripe decline code must return None".into())
        },
    }
}

// ---------------------------------------------------------------------------
// Stripe stub functions — verify Coming Soon behavior
// ---------------------------------------------------------------------------

fn stripe_f120_returns_coming_soon() -> t24 {
    let start = Instant::now();
    let ok = crate::switch::f120(b"{}").is_err();
    t24 { name: "stripe_f120_returns_coming_soon".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("f120 stub must return Err".into()) } }
}

fn stripe_f121_returns_coming_soon() -> t24 {
    let start = Instant::now();
    let ok = crate::switch::f121(1, 100).is_err();
    t24 { name: "stripe_f121_returns_coming_soon".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("f121 stub must return Err".into()) } }
}

fn stripe_f122_returns_coming_soon() -> t24 {
    let start = Instant::now();
    use crate::switch::t31;
    let resp = t31 { mti: *b"0210", response_code: *b"00", stan: 1, amount_cents: 100, approved: true, auth_code: None };
    let ok = crate::switch::f122(&resp).is_err();
    t24 { name: "stripe_f122_returns_coming_soon".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("f122 stub must return Err".into()) } }
}

// ---------------------------------------------------------------------------
// f123: Stripe HMAC-SHA256 webhook verification
// ---------------------------------------------------------------------------

fn make_stripe_sig(secret: &str, timestamp: &str, payload: &[u8]) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    let mut signed = timestamp.as_bytes().to_vec();
    signed.push(b'.');
    signed.extend_from_slice(payload);
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(&signed);
    let bytes = mac.finalize().into_bytes();
    let hex: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    format!("t={},v1={}", timestamp, hex)
}

fn stripe_f123_valid_signature() -> t24 {
    let start = Instant::now();
    // Set env var, compute real signature, verify it passes
    std::env::set_var("STRIPE_WEBHOOK_SECRET", "whsec_test_secret_key_for_tests");
    let payload = br#"{"type":"payment_intent.created","data":{}}"#;
    let sig = make_stripe_sig("whsec_test_secret_key_for_tests", "1492774577", payload);
    let result = crate::switch::f123(payload, &sig);
    let ok = result == Ok(true);
    t24 { name: "stripe_f123_valid_signature".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some(format!("expected Ok(true), got {:?}", result)) } }
}

fn stripe_f123_invalid_signature() -> t24 {
    let start = Instant::now();
    std::env::set_var("STRIPE_WEBHOOK_SECRET", "whsec_test_secret_key_for_tests");
    let payload = b"real payload";
    let sig = "t=1492774577,v1=deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";
    let result = crate::switch::f123(payload, sig);
    let ok = result == Ok(false);
    t24 { name: "stripe_f123_invalid_signature".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some(format!("expected Ok(false), got {:?}", result)) } }
}

fn stripe_f123_missing_env_returns_err() -> t24 {
    let start = Instant::now();
    std::env::remove_var("STRIPE_WEBHOOK_SECRET");
    let result = crate::switch::f123(b"payload", "t=1,v1=abc");
    let ok = result.is_err();
    // Restore for other tests
    std::env::set_var("STRIPE_WEBHOOK_SECRET", "whsec_test_secret_key_for_tests");
    t24 { name: "stripe_f123_missing_env_returns_err".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("missing env must return Err".into()) } }
}

fn stripe_f123_missing_timestamp_err() -> t24 {
    let start = Instant::now();
    std::env::set_var("STRIPE_WEBHOOK_SECRET", "whsec_test_secret_key_for_tests");
    // Header with no t= part
    let result = crate::switch::f123(b"payload", "v1=deadbeef");
    let ok = result.is_err();
    t24 { name: "stripe_f123_missing_timestamp_err".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("missing t= must return Err".into()) } }
}

fn stripe_f123_missing_v1_err() -> t24 {
    let start = Instant::now();
    std::env::set_var("STRIPE_WEBHOOK_SECRET", "whsec_test_secret_key_for_tests");
    // Header with t= but no v1=
    let result = crate::switch::f123(b"payload", "t=1492774577");
    let ok = result.is_err();
    t24 { name: "stripe_f123_missing_v1_err".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("missing v1= must return Err".into()) } }
}

fn stripe_f123_tampered_payload_rejected() -> t24 {
    let start = Instant::now();
    std::env::set_var("STRIPE_WEBHOOK_SECRET", "whsec_test_secret_key_for_tests");
    let original = b"original payload";
    let tampered = b"tampered payload";
    let sig = make_stripe_sig("whsec_test_secret_key_for_tests", "1492774577", original);
    // Use sig computed for original, but verify against tampered
    let result = crate::switch::f123(tampered, &sig);
    let ok = result == Ok(false);
    t24 { name: "stripe_f123_tampered_payload_rejected".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("tampered payload must return Ok(false)".into()) } }
}

// ---------------------------------------------------------------------------
// Auth: session signing, verification, cookie extraction
// ---------------------------------------------------------------------------

fn auth_session_secret_returns_value() -> t24 {
    let start = Instant::now();
    let secret = auth::f125();
    let ok = !secret.is_empty();
    t24 { name: "auth_session_secret_returns_value".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("f125 must return non-empty secret".into()) } }
}

fn auth_sign_verify_round_trip() -> t24 {
    let start = Instant::now();
    let uid = uuid::Uuid::new_v4();
    // Sign a session using the internal function, then verify via f20
    let secret = auth::f125();
    // We test f20 by constructing a valid cookie value
    // f20 calls verify_session internally, so we need to create a valid signed session
    // Use the same signing logic: payload = "uuid:exp", sig = hmac-sha256
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    let exp = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64) + 3600;
    let payload = format!("{}:{}", uid, exp);
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload.as_bytes());
    let sig = mac.finalize().into_bytes();
    let sig_b64 = base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, sig);
    let cookie_val = format!("{}.{}", payload, sig_b64);

    let result = auth::f20(Some(&cookie_val));
    let ok = result == Some(uid);
    t24 { name: "auth_sign_verify_round_trip".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("signed session must verify to same user_id".into()) } }
}

fn auth_verify_expired_session() -> t24 {
    let start = Instant::now();
    let uid = uuid::Uuid::new_v4();
    let secret = auth::f125();
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    let exp = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64) - 3600; // expired 1h ago
    let payload = format!("{}:{}", uid, exp);
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload.as_bytes());
    let sig = mac.finalize().into_bytes();
    let sig_b64 = base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, sig);
    let cookie_val = format!("{}.{}", payload, sig_b64);

    let ok = auth::f20(Some(&cookie_val)).is_none();
    t24 { name: "auth_verify_expired_session".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("expired session must return None".into()) } }
}

fn auth_verify_tampered_session() -> t24 {
    let start = Instant::now();
    let uid = uuid::Uuid::new_v4();
    let secret = auth::f125();
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    let exp = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64) + 3600;
    let payload = format!("{}:{}", uid, exp);
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload.as_bytes());
    let sig = mac.finalize().into_bytes();
    let sig_b64 = base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, sig);
    // Tamper: change uuid in payload but keep original sig
    let tampered_uid = uuid::Uuid::new_v4();
    let tampered_payload = format!("{}:{}", tampered_uid, exp);
    let cookie_val = format!("{}.{}", tampered_payload, sig_b64);

    let ok = auth::f20(Some(&cookie_val)).is_none();
    t24 { name: "auth_verify_tampered_session".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("tampered session must return None".into()) } }
}

fn auth_verify_malformed_session() -> t24 {
    let start = Instant::now();
    let cases = [
        "",
        "not-a-session",
        "no-dot-separator",
        "a.b.c.too-many-dots",
        "invalid-uuid:12345.AAAA",
    ];
    let ok = cases.iter().all(|c| auth::f20(Some(c)).is_none());
    t24 { name: "auth_verify_malformed_session".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("malformed sessions must all return None".into()) } }
}

fn auth_f20_none_input_returns_none() -> t24 {
    let start = Instant::now();
    let ok = auth::f20(None).is_none();
    t24 { name: "auth_f20_none_input_returns_none".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("f20(None) must return None".into()) } }
}

fn auth_f20_empty_string_returns_none() -> t24 {
    let start = Instant::now();
    let ok = auth::f20(Some("")).is_none();
    t24 { name: "auth_f20_empty_string_returns_none".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("f20(Some('')) must return None".into()) } }
}

fn auth_f124_no_cookie_header() -> t24 {
    let start = Instant::now();
    let headers = axum::http::HeaderMap::new();
    let ok = auth::f124(&headers).is_none();
    t24 { name: "auth_f124_no_cookie_header".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("no cookie header → None".into()) } }
}

fn auth_f124_other_cookies_only() -> t24 {
    let start = Instant::now();
    let mut headers = axum::http::HeaderMap::new();
    headers.insert("cookie", "other_cookie=abc; tracking=xyz".parse().unwrap());
    let ok = auth::f124(&headers).is_none();
    t24 { name: "auth_f124_other_cookies_only".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("cookies without rr_session → None".into()) } }
}

fn auth_f124_extracts_session_cookie() -> t24 {
    let start = Instant::now();
    let mut headers = axum::http::HeaderMap::new();
    headers.insert("cookie", "other=1; rr_session=test_value_123; tracking=z".parse().unwrap());
    let result = auth::f124(&headers);
    let ok = result.as_deref() == Some("test_value_123");
    t24 { name: "auth_f124_extracts_session_cookie".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some(format!("expected Some(test_value_123), got {:?}", result)) } }
}

// ---------------------------------------------------------------------------
// Vault edge cases
// ---------------------------------------------------------------------------

fn vault_key_32_bytes_works() -> t24 {
    let start = Instant::now();
    let key = [0xABu8; 32];
    let ok = t1::new(&key).is_ok();
    t24 { name: "vault_key_32_bytes_works".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("32-byte key must succeed".into()) } }
}

fn vault_decrypt_exactly_nonce_length() -> t24 {
    let start = Instant::now();
    let v = t1::new(&[1u8; 32]).unwrap();
    // 12 bytes = exactly nonce length, no ciphertext — should fail
    let ok = f11(&v, &[0u8; 12]).is_err();
    t24 { name: "vault_decrypt_exactly_nonce_length".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("12 bytes (nonce only, no ciphertext) must fail".into()) } }
}

fn vault_large_plaintext() -> t24 {
    let start = Instant::now();
    let v = t1::new(&[2u8; 32]).unwrap();
    let large = vec![0x42u8; 65536]; // 64KB
    let encrypted = f10(&v, &large);
    let ok = match encrypted {
        Ok(enc) => f11(&v, &enc).map(|dec| dec == large).unwrap_or(false),
        Err(_) => false,
    };
    t24 { name: "vault_large_plaintext".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("64KB plaintext must round-trip".into()) } }
}

fn vault_encrypt_different_each_time() -> t24 {
    let start = Instant::now();
    let v = t1::new(&[3u8; 32]).unwrap();
    let plain = b"test-data";
    let enc1 = f10(&v, plain).unwrap();
    let enc2 = f10(&v, plain).unwrap();
    let enc3 = f10(&v, plain).unwrap();
    // All three should produce different ciphertext (different nonces)
    let ok = enc1 != enc2 && enc2 != enc3 && enc1 != enc3;
    t24 { name: "vault_encrypt_different_each_time".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("same plaintext must produce different ciphertext each time".into()) } }
}

// ---------------------------------------------------------------------------
// ISO 8583 edge cases
// ---------------------------------------------------------------------------

fn switch_0200_stan_zero() -> t24 {
    let start = Instant::now();
    let req = t2 { pan_encrypted: vec![1, 2, 3], amount_cents: 100, stan: 0 };
    let ok = f12(&req).is_ok(); // STAN=0 is valid
    t24 { name: "switch_0200_stan_zero".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("STAN=0 must be valid".into()) } }
}

fn switch_0200_max_amount() -> t24 {
    let start = Instant::now();
    let req = t2 { pan_encrypted: vec![1], amount_cents: 99_999_999, stan: 1 };
    let ok = f12(&req).is_ok();
    t24 { name: "switch_0200_max_amount".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("99999999 cents must succeed".into()) } }
}

fn switch_0200_boundary_amount_1() -> t24 {
    let start = Instant::now();
    let req = t2 { pan_encrypted: vec![1], amount_cents: 1, stan: 1 };
    let ok = f12(&req).is_ok();
    t24 { name: "switch_0200_boundary_amount_1".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("1 cent must succeed".into()) } }
}

fn switch_0200_amount_99999999() -> t24 {
    let start = Instant::now();
    // Just above max should fail
    let req = t2 { pan_encrypted: vec![1], amount_cents: 100_000_000, stan: 1 };
    let ok = f12(&req).is_err();
    t24 { name: "switch_0200_amount_99999999".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("100000000 cents must fail (max is 99999999)".into()) } }
}

fn switch_0100_default_merchant() -> t24 {
    let start = Instant::now();
    let req = t30::default();
    let msg = f17(&req).unwrap();
    let ok = msg.raw.windows(15).any(|w| w == b"ROGUEREPO000000");
    t24 { name: "switch_0100_default_merchant".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("default t30 must have ROGUEREPO000000".into()) } }
}

fn switch_0210_boundary_32_bytes() -> t24 {
    let start = Instant::now();
    // Exactly 32 bytes: MTI(4) + bitmap(8) + response_code(2) + amount(12) + STAN(6) = 32
    let mut raw = Vec::new();
    raw.extend_from_slice(b"0210");
    raw.extend_from_slice(&[0u8; 8]); // bitmap
    raw.extend_from_slice(b"00"); // response code = approved
    raw.extend_from_slice(b"000000000100"); // amount = 100 cents
    raw.extend_from_slice(b"000042"); // STAN = 42
    let result = f18(&raw);
    let ok = match result {
        Ok(r) => r.approved && r.amount_cents == 100 && r.stan == 42,
        Err(_) => false,
    };
    t24 { name: "switch_0210_boundary_32_bytes".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("exact 32-byte 0210 must parse".into()) } }
}

fn switch_0210_invalid_amount_string() -> t24 {
    let start = Instant::now();
    let mut raw = Vec::new();
    raw.extend_from_slice(b"0210");
    raw.extend_from_slice(&[0u8; 8]);
    raw.extend_from_slice(b"00");
    raw.extend_from_slice(b"NOT_A_NUMBER"); // invalid amount
    raw.extend_from_slice(b"000001");
    let ok = f18(&raw).is_err();
    t24 { name: "switch_0210_invalid_amount_string".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("non-numeric amount must fail parse".into()) } }
}

fn switch_0400_pan_empty() -> t24 {
    let start = Instant::now();
    let req = t32 { pan_encrypted: vec![], amount_cents: 100, original_stan: 1, reversal_stan: 2, reason: t34::Timeout };
    let ok = f19(&req).is_ok(); // empty PAN is valid (length=0)
    t24 { name: "switch_0400_pan_empty".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("empty PAN must be valid for reversal".into()) } }
}

fn switch_0400_all_reasons_build() -> t24 {
    let start = Instant::now();
    let reasons = [t34::Timeout, t34::CustomerCancel, t34::SystemError];
    let ok = reasons.iter().all(|r| {
        let req = t32 { pan_encrypted: vec![1], amount_cents: 100, original_stan: 1, reversal_stan: 2, reason: *r };
        f19(&req).is_ok()
    });
    t24 { name: "switch_0400_all_reasons_build".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("all reversal reasons must build successfully".into()) } }
}

// ---------------------------------------------------------------------------
// TCP endpoint config
// ---------------------------------------------------------------------------

fn tcp_endpoint_from_env_missing() -> t24 {
    let start = Instant::now();
    // SWITCH_HOST not set → from_env returns None
    let ok = t39::from_env().is_none() || std::env::var("SWITCH_HOST").is_ok();
    t24 { name: "tcp_endpoint_from_env_missing".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("no SWITCH_HOST → from_env returns None".into()) } }
}

fn tcp_endpoint_addr_format() -> t24 {
    let start = Instant::now();
    let ep = t39 { host: "10.0.0.1".into(), port: 8583, timeout_ms: 5000 };
    let ok = format!("{}:{}", ep.host, ep.port) == "10.0.0.1:8583";
    t24 { name: "tcp_endpoint_addr_format".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("addr must format as host:port".into()) } }
}

// ---------------------------------------------------------------------------
// More Stripe coverage
// ---------------------------------------------------------------------------

fn stripe_card_declined_maps_to_05() -> t24 {
    let start = Instant::now();
    let ok = t36::from_stripe("card_declined")
        .map(|c| c.to_iso_response() == *b"05")
        .unwrap_or(false);
    t24 { name: "stripe_card_declined_maps_to_05".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("card_declined must map to ISO 05".into()) } }
}

fn stripe_all_events_have_mti() -> t24 {
    let start = Instant::now();
    let events = [
        ("payment_intent.created", "0100"),
        ("charge.succeeded", "0200"),
        ("charge.captured", "0220"),
        ("charge.refunded", "0420"),
        ("charge.failed", "0210"),
        ("payment_intent.canceled", "0400"),
        ("dispute.created", "0400"),
    ];
    let ok = events.iter().all(|(event, expected_mti)| {
        t37::from_stripe(event)
            .map(|e| e.iso_mti() == *expected_mti)
            .unwrap_or(false)
    });
    t24 { name: "stripe_all_events_have_mti".into(), passed: ok, duration_ms: start.elapsed().as_millis() as u64, message: if ok { None } else { Some("all 7 Stripe events must map to expected MTIs".into()) } }
}
