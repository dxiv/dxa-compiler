//! Placeholder crate for the DEXA reference compiler.
//! Full implementation lives in dxa-dev until 1.0.0.

/// Compiler version (matches placeholder state).
pub const VERSION: &str = "0.1.0";

/// Run DEXA source. Placeholder: returns an error directing to dxa-dev.
pub fn run_source(source: &str) -> Result<String, String> {
    let _ = source;
    Err("Not implemented. Full compiler in dxa-dev until 1.0.0.".into())
}
