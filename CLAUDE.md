# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What Vif is

Vif is an experimental, Python-looking programming language implemented in Rust. Source files use the `.vif` extension. The language is dynamic but the compiler does as much static work as possible (notably type inference and mutability checking). Variables are declared with `var`, made mutable with `mut`, and are always passed by reference. Indentation is significant (Python-style).

The project recently migrated from a custom stack-based bytecode VM to **LLVM** (via the `inkwell` crate). Several docs and the README still describe the old VM/bytecode compiler — treat the actual crate graph and pipeline below as ground truth, not the prose docs.

## Build / run / test

```bash
cargo build --release                          # build (default member is vif-cli)
./target/release/vif-cli run <file.vif>        # JIT-compile and execute a file
./target/release/vif-cli build <file.vif>      # emit an object file (currently "here.o")
./target/release/vif-cli print --ast <file>    # print the AST tree
./target/release/vif-cli print --assembly <f>  # print generated LLVM IR (default for `print`)
./target/release/vif-cli compile               # read LLVM IR from stdin and JIT-run it
```

Logging is controlled by env vars read in `vif-loader`: `DEBUG=1` enables trace logging; `VIF_LOG_LEVEL=<level>` sets the `log` level filter explicitly.

### LLVM toolchain requirement

This needs **LLVM 22** at build time, not the LLVM 16 the README mentions. `inkwell` is pinned to the `llvm22-1` feature, and the build expects `LLVM_SYS_221_PREFIX` to point at an LLVM 22 install (see `.envrc`, which sets it to a local `./llvm/...` directory). On NixOS, `shell.nix` provides the dependencies (note: `shell.nix` is also stale at LLVM 16 / `LLVM_SYS_160_PREFIX` and may need updating).

### Tests — important caveat

`cargo test --workspace` does **not** currently pass. The integration tests under `crates/vif-cli/tests/` (`lib.rs`, `test_snippets.rs`) still `use vif_compiler` and `vif_vm`, crates that were deleted during the LLVM migration. They assert on bytecode `OpCode` sequences that no longer exist. These tests need rewriting against the LLVM pipeline before they compile. The `.vif` fixtures they were meant to exercise live in `tests/` (and more samples in `snippets/`).

Per-crate unit tests (e.g. in `vif-typing`) are the parts of the suite that are still meaningful.

## Compilation pipeline

The flow is orchestrated in `crates/vif-cli/src/application.rs` (`Vif::run`), dispatching on the `Action`/`Print` enums from `vif-loader`:

```
source text
  → vif-scanner   (Scanner: text → tokens)
  → vif-ast       (Parser: tokens → AST; build_ast)            -> Function AST root
  → vif-typing    (run_typing_checks: in-place type + mutability passes)
  → vif-llvm      (Compiler over inkwell: AST → LLVM IR → JIT exec / object file / IR string)
```

- **`vif-scanner`** — lexer. `Scanner`, `token`.
- **`vif-ast`** — Pratt-style parser producing the AST. Entry point `build_ast`. `print_ast_tree` renders the tree (uses `treeline`). The AST node types themselves live in `vif-objects::ast`.
- **`vif-typing`** — mutates the AST in place. `run_typing_checks` runs two `BottomUpTyper` passes: a `SoftTypeMerger` pass then a `HardTypeMerger` pass (so function parameter types are known on the second pass), followed by `check_mutability`. This is where the "compiler does its best statically" promise is implemented.
- **`vif-llvm`** — the current backend. `Compiler` wraps an `inkwell` `Context`/module. `compile_and_execute` JITs and runs; `compile_and_build_binary` writes an object file via a target machine; `get_llvm_ir` returns the IR as a string; `execute_llvm_from_stdin` JITs IR piped in. Builtins are registered via `add_builtin_functions`. Several constructs are still `unimplemented!()` (e.g. `assert`, some operators) — expect gaps.
- **`vif-objects`** — shared data structures. Contains the `ast` module (consumed by ast/typing/llvm) plus leftover VM-era types (`chunk`, `op_code`, `stack`, `global_store`, etc.) that are now mostly dead and only still referenced by the broken tests.
- **`vif-native`** — native/builtin runtime functions (`io`, `time`).
- **`vif-loader`** — CLI definition (`clap` in `cli.rs`), global `CONFIG` (`lazy_static`, in `config.rs`), and logging setup. Subcommands: `run`, `build`, `compile`, `print`.

## Workspace notes

- Cargo workspace; `default-members = ["crates/vif-cli"]`, so bare `cargo run`/`cargo build` target the CLI.
- `crates/vif-cli-prof` is a separate profiling harness (uses `pprof` for flamegraphs) and is **not** a workspace member — build/test it explicitly from its own directory.
- Release profile is tuned for performance benchmarking: `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, `debug = true`. Benchmarking workflow (copy a baseline binary, rebuild, compare with `hyperfine`) is documented in `README.md` and `benchmark.md`.

## Source control

This repo is managed with **jj (Jujutsu)**, not plain git — there is a `.jj/` directory. Per user instruction, use `jj` rather than `git` for version-control operations.
