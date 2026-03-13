# dxa-compiler

> **GitHub About:** Future home of the DEXA reference compiler. Developed in private monorepo until 1.0.0.

Reference compiler for **DEXA** — a statically typed, deterministic language for the DXA stack (GPU kernels, native binaries, contract VMs). Written in Rust.

**Status:** This repo is a **placeholder**. The compiler is currently developed in [dxa-dev](https://github.com/dxiv/dxa-dev). When the monorepo is split, this repo will hold the compiler source, CLI, and examples.

---

## What it does

- **Input:** DEXA source (`.dxa` files) — functions, models, contracts, top-level `let` globals.
- **Pipeline:** Lexer → Parser (AST) → Typechecker → Lowering (DX-IR) → IR validation → Interpreter.
- **Output:** Program result (value, prints, execution trace, gas used) or a typed error with line/column and source snippet where available.

The compiler is single-pass: no separate “runtime” binary; the same binary compiles and executes. Future backends (LLVM, GPU, DX-VM) are planned; today only the interpreter runs.

---

## CLI

When this repo is populated from dxa-dev, you’ll get the `dxc` binary:

```bash
dxc <file.dxa>
```

- **Success:** Prints program output, result value, and gas (steps).
- **Failure:** Exit code 1 with lex/parse/typecheck/IR or runtime error (e.g. `require` failure).

---

## Library API (Rust)

Intended public surface (from the current dxa-dev implementation):

| Function | Description |
|----------|-------------|
| `run_source(source: &str)` | Compile and run a string; returns `Result<ProgramResult, DexaError>`. |
| `run_source_with_debug(source: &str)` | Same, plus pretty-printed AST and IR for tooling/debugging. |
| `run_file(path: &Path)` | Read file and run (convenience over `run_source`). |

**Re-exported types:** `DexaError`, `ProgramResult`, `ExecutionTrace`, `GasMeter`, `Value` (runtime value).

---

## Version and dependencies

- **Version:** 0.1.0 (aligned with dxa-dev).
- **Rust:** Edition 2021.
- **Dependencies:** `thiserror` (errors). No runtime deps.

---

## Build and test (after split)

```bash
cargo build --release
cargo test
```

The `dxc` CLI will be built from `src/main.rs`; the library from `src/lib.rs`.

---

## Examples

When split, example `.dxa` files (e.g. `hello.dxa`, `wallet.dxa`, `loops.dxa`, builtins and error-case tests) will live in an `examples/` directory here or in the dxa-dev repo until the split.

---

## License

Apache 2.0 (to match the compiler crate in dxa-dev).

---

## Links

- **Language & site:** [dxa.dev](https://dxa.dev)
- **Monorepo (current development):** [dxa-dev](https://github.com/dxiv/dxa-dev)
- **WASM build (playground):** [dxa-wasm](https://github.com/dxiv/dxa-wasm)
