// Copyright (c) 2026 The Cochran Block, LLC (Pending). All rights reserved.
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports, clippy::module_inception)]
//! vault: Encryption/decryption core. "Radioactive Data" — PAN never plaintext.
//! t1 = Vault, f10 = encrypt_pan, f11 = decrypt_pan

mod error;
mod vault;

pub use error::E3;
pub use vault::{f10, f11, t1};
