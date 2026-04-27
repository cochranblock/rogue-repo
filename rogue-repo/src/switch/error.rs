// Unlicense — public domain — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum E4 {
    #[error("pack: {0}")]
    Pack(String),
    #[error("unpack: {0}")]
    Unpack(String),
    #[error("connection: {0}")]
    Connection(String),
    #[error("invalid amount: {0}")]
    Amount(String),
}
