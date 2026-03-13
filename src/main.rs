//! Minimal dxc CLI stub. Full implementation in dxa-dev.

use std::env;
use std::fs;
use std::process;

use dexa_compiler::{run_source, VERSION};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("dxc — DEXA reference compiler (placeholder)");
        eprintln!("Usage: dxc <file.dxa>");
        eprintln!("Version: {}", VERSION);
        eprintln!("\nFull implementation: https://github.com/dxiv/dxa-dev");
        process::exit(1);
    }

    let path = &args[1];
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    match run_source(&source) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
