// Unlicense — public domain — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! switch: ISO 8583 message types + TCP transport.
//! f12 = build_0200 (purchase), f17 = build_0100 (auth), f18 = parse_0210 (response), f19 = build_0400 (reversal)
//! f127 = send_and_receive (TCP), f128 = connect, f129 = send_and_parse

mod error;
mod iso8583;
pub mod stripe;
pub mod tcp;

pub use error::E4;
pub use iso8583::{f12, f17, f18, f19, t2, t3, t30, t31, t32, t34};
pub use stripe::{f120, f121, f122, f123, t36, t37, StripeOutcome};
pub use tcp::{f127, f128, f129, t39};
