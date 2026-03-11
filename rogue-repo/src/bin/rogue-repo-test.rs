// Copyright (c) 2026 The Cochran Block, LLC (Pending). All rights reserved.
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! f31=rogue_repo_test. TRIPLE SIMS via exopack::triple_sims::f60. f30=run_tests.

use rogue_repo::tests;

#[tokio::main]
async fn main() {
    let ok = exopack::triple_sims::f60(|| async { tests::f30().await }).await;
    std::process::exit(if ok { 0 } else { 1 });
}
