# Assembly Policy

## Policy

Assembly (`unsafe` inline assembly or `.s` files) is **forbidden** in Sprints 1-3. Starting from Sprint 4, assembly is allowed only when ALL of the following conditions are met:

1. **Real profiler output proves a bottleneck.** Use `perf` (Linux), `xctrace` (macOS), or `flamegraph-rs` to produce a flamegraph showing the hot path. The profile must be taken on a realistic workload, not a microbenchmark.
2. **A pure-Rust fallback exists.** The fallback must be the default implementation. The assembly path is selected at compile time via `cfg`.
3. **Isolated in its own module.** Each assembly-using module lives in a separate file. The `unsafe` keyword appears only at the call site, not inside the assembly block.
4. **Gated behind `cfg`.** The assembly code must be behind `cfg(any(target_arch = "x86_64", target_arch = "aarch64"))`. Other architectures use the pure-Rust fallback.
5. **Maintainable.** The assembly must be commented with the exact semantics of each instruction block. A corresponding Rust reference implementation must be kept in the same file as a doc-test.

## Current assembly usage

**None.** As of Sprint 1, no assembly code exists anywhere in the workspace. All performance-critical paths (file I/O, project traversal) use standard library functions.

## How to add assembly safely

Follow this checklist:

- [ ] Run a profiler on the current pure-Rust implementation. Save the flamegraph.
- [ ] Write the assembly version in a new file, e.g. `crates/editor/src/sha1_fast.rs`.
- [ ] Gate the module: `#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]`.
- [ ] Keep the pure-Rust version as `mod sha1_fallback`.
- [ ] At the call site: `#[cfg(feature = "asm")] pub use sha1_fast::hash;` / `pub use sha1_fallback::hash;`.
- [ ] Run the same benchmark and compare results.
- [ ] If the speedup is less than 2x on the profiled workload, revert to the pure-Rust version.
- [ ] Add a `#[deny(unsafe)]` lint to the fallback module to prevent accidental unsafe code.

## How to profile before adding assembly

```sh
# Linux: perf record
cargo build --release -p app
perf record --call-graph dwarf ./target/release/app --some-benchmark-arg
perf report

# Universal: flamegraph-rs
cargo install flamegraph
cargo flamegraph -p app -- --some-benchmark-arg
```

Do NOT add assembly based on intuition or microbenchmarks. Only real-world profiling data justifies unsafe code.
