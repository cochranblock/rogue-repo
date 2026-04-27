// Unlicense — public domain — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! f31=rogue_repo_test. TRIPLE SIMS via exopack::triple_sims::f61_with_args (cargo test 3x).

fn main() {
    let project = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let (ok, stderr) = exopack::triple_sims::f61_with_args(project, 3, &[]);
    if !ok {
        eprintln!("{}", stderr);
        std::process::exit(1);
    }
}
