// Copyright (c) 2026 The Cochran Block, LLC (Pending). All rights reserved.
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! switch: ISO 8583 message types.
//! f12 = build_0200 (purchase), f17 = build_0100 (auth), f18 = parse_0210 (response), f19 = build_0400 (reversal)

mod error;
mod iso8583;

pub use error::E4;
pub use iso8583::{f12, f17, f18, f19, t2, t3, t30, t31, t32, t34};
