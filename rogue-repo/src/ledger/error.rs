// Copyright (c) 2026 The Cochran Block, LLC (Pending). All rights reserved.
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum E5 {
    #[error("insufficient bucks: {0}")]
    Insufficient(i64),
    #[error("device already authorized")]
    DeviceExists,
    #[error("db: {0}")]
    Db(String),
    #[error("not found: {0}")]
    NotFound(String),
}
