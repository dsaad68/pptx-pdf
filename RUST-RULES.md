# Rust Development Rules

Anything a tool can check lives in `Cargo.toml`, `rustfmt.toml`, or CI — see
[Enforcement](#enforcement) at the bottom. This file covers only what tooling cannot express.

## Toolchain

- Edition 2024, toolchain pinned in `rust-toolchain.toml`. Do not use idioms or APIs newer
  than the pinned version.
- MSRV is declared as `rust-version` in `Cargo.toml`. If a change raises it, say so explicitly
  rather than silently bumping it.

## Verifying APIs (read this first)

- Before using an unfamiliar item from a dependency, check the version in `Cargo.lock` and read
  that version's docs on docs.rs. Do not reconstruct signatures from memory — crate APIs churn
  across minor versions and a plausible-looking wrong signature costs more than a lookup.
- If a needed API doesn't exist in the pinned version, say so and propose either a version bump
  or a different approach. Do not invent a shim that pretends the API exists.
- `cargo check` after every non-trivial edit. Don't batch up unverified code.

## Scope discipline

- Change what was asked for. Do not reformat, rename, or "improve" adjacent code in the same
  edit — it buries the actual diff.
- Never silence a lint to make a build pass. Fix the cause, or surface the tradeoff and ask.
  If a suppression is genuinely correct, use `#[expect(lint, reason = "...")]`, never bare
  `#[allow]` — `expect` warns once the suppression stops being needed.
- Do not edit generated files, `Cargo.lock` by hand, or anything under `target/`.
- No commented-out code and no `dbg!`/`println!` debugging left behind.

## Error handling

- `Result<T, E>` for anything caused by input, I/O, or the environment.
- Panic for broken internal invariants and violated API contracts — that is what `panic!`,
  `assert!`, and slice indexing are for. The rule is not "never panic", it is "never panic on
  runtime input". Every panicking path in a public function gets a `# Panics` doc section.
- `thiserror` for library error types, `anyhow` at application boundaries. Do not leak `anyhow`
  into a library's public API.
- `?` for propagation. Add context at the layer that knows what the operation meant, not at every
  frame — a stack of `.context("failed to read file")` on the same failure is noise.
- `unwrap`/`expect` are denied outside `#[cfg(test)]`. If a case is genuinely infallible, restructure
  so the type system shows it; if that's impossible, use `expect` with a message stating the
  invariant and justify it in review.

## Ownership and API design

- Never take `&String`, `&Vec<T>`, or `&Box<T>` as a parameter. Use `&str`, `&[T]`, `&T`.
- Let the body decide ownership: if the function stores or consumes the value, take `String`/`Vec<T>`
  by value. Taking `&str` and calling `.to_owned()` inside is worse than taking `String` — it forces
  a clone on callers who already own one.
- `impl Into<String>` / `AsRef<Path>` only where callers realistically pass several types. It is
  monomorphization cost and worse error messages otherwise.
- Every `.clone()` in non-test code carries a comment saying why, or gets removed. `Cow<'_, T>` when
  ownership is genuinely conditional — not as a default.
- Prefer accepting `impl Iterator` / `&[T]` over `Vec<T>` for read-only sequence parameters.
- Newtypes over bare primitives for domain values (`UserId(u64)`, not `u64`). Two `u64` parameters
  in a row are a bug waiting to happen.
- Conversion naming follows the standard convention: `as_*` is a cheap borrow, `to_*` is expensive
  and returns owned, `into_*` consumes `self`. Do not name an allocating method `as_`.
- Derive eagerly on public types: `Debug` always; `Clone`, `PartialEq`, `Eq`, `Hash`, `Default`,
  `Copy` where they make sense. Implement `Display` for anything a user sees, `From` rather than
  ad-hoc `new_from_x` constructors.
- `#[must_use]` on functions whose return value is the entire point (builders, pure transforms).

## API design and SemVer (public crates)

- Mark public structs, enums, and enum variants `#[non_exhaustive]` at introduction if you may add
  fields or variants later. Adding a variant to an exhaustive enum is a breaking change, and adding
  `#[non_exhaustive]` afterwards is itself breaking — so the decision has to be made up front.
- Do not match your own enums with a catch-all `_` arm. Exhaustive matching means adding a variant
  becomes a compile error at every site that must handle it, which is the point.
- Public error enums are `#[non_exhaustive]`. New error cases should not be a major version.
- Keep the public surface minimal: `pub(crate)` by default, `pub` deliberately. Re-export the
  intended API from the crate root rather than making callers navigate the module tree.
- `cargo semver-checks` in CI for anything published.

## Async

- `tokio`, multi-threaded runtime unless there's a stated reason otherwise.
- Never hold a `std::sync::Mutex` or `RefCell` guard across an `.await`. Use `tokio::sync::Mutex`
  only when the lock must be held across a suspension point; otherwise the std lock plus a tight
  scope is faster. Do not reach for `RwLock` reflexively — for short critical sections it is
  usually slower than `Mutex` and adds writer-starvation risk.
- Anything that blocks for more than a few microseconds (filesystem, CPU work, sync DB drivers)
  goes in `spawn_blocking`. Blocking the runtime is a silent latency bug.
- `tokio::select!` drops the losing futures at the point they were suspended. Before putting a future
  in a `select!`, state whether it is cancel-safe; if it isn't, hold it in a variable across loop
  iterations rather than recreating it.
- Spawned tasks need a documented owner and shutdown path. No detached `tokio::spawn` whose
  `JoinHandle` is discarded.
- Public async APIs should be `Send` unless there's a reason not to; a non-`Send` future forces
  every caller onto a single-threaded runtime.

## Unsafe

- Default is no `unsafe`. If it's unavoidable, every `unsafe` block gets a `// SAFETY:` comment
  naming the invariant being upheld and why it holds here.
- `unsafe_op_in_unsafe_fn` is denied — an `unsafe fn` body still needs explicit `unsafe` blocks.
- Anything with `unsafe` gets a Miri run in CI.

## Testing

- Unit tests in a `#[cfg(test)]` module in the same file; integration tests in `tests/`.
- Test names describe the behavior, not the function: `rejects_empty_payload`, not
  `test_parse_when_empty_then_error`. The runner already prints the module path.
- Cover the error paths and boundaries, not just the happy path. A test that only asserts
  `result.is_ok()` is not a test.
- Tests must be deterministic and independent of wall-clock time, network, and execution order.
  Inject clocks and I/O through traits rather than mocking globals. Prefer a real dependency behind
  a trait seam over a heavily mocked one — over-mocking tests the mock, not the code.
- `hegeltest` for parsers, encoders, and anything with an invertibility or ordering property.
  State the invariant and let it find the case nobody thought of; the example-based test names
  only the cases somebody did. `insta` for snapshotting complex output.
- hegel is in beta and says it may make breaking changes. It is pinned with a caret like every
  other dependency, and a breaking change is a dev-dependency problem, never a shipped one.

## Documentation

- `///` on all public items, with an example that compiles as a doctest for anything non-obvious.
- Document `# Errors`, `# Panics`, and `# Safety` where they apply. `missing_docs` is warned on.
- Module-level `//!` explaining what the module owns and where its boundaries are.
- Link to related items with intra-doc links (`[`Foo::bar`]`) instead of naming them in prose.

## Dependencies

- Caret requirements in `Cargo.toml` (`serde = "1.0"`), never `=1.2.3`. Exact pins in a manifest
  make transitive conflicts unresolvable rather than safe.
- `Cargo.lock` is committed for every crate in this repo, library or binary. It is what pins
  versions; CI and `cargo install` use `--locked`.
- Do not add a dependency without saying so. For anything that is a few dozen lines of std,
  propose writing it instead.
- Enable only the features actually used; `default-features = false` where practical.
- Cargo features must be additive. Never make two features mutually exclusive or have one
  feature change behavior rather than add it.
- In workspaces, versions live in `[workspace.dependencies]` and members use
  `dep = { workspace = true }`.
- `cargo deny check` gates licenses and advisories; `cargo machete` catches unused deps.

## Security

- Secrets never appear in source, tests, or fixtures. Load from the environment (`std::env` or
  `dotenvy`); `.env` is gitignored and a `.env.example` documents the keys.
- Never log credentials, tokens, or PII. Implement `Debug` manually (or use a redacting wrapper)
  for any type holding a secret — a derived `Debug` will happily print it.
- Validate and bound external input before allocating from it. No `Vec::with_capacity(n)` where
  `n` comes from the network.
- Use constant-time comparison for secrets and MACs, not `==`.

## Observability

- `tracing` for all diagnostics; `println!`/`eprintln!` only for a CLI's actual user-facing output.
- Instrument at boundaries with `#[instrument]`, with `skip` on large or sensitive arguments.
- Errors are logged once, at the layer that decides what to do about them — not at every frame
  on the way up.

## Project structure

- Modules as directories with the module file alongside (`foo.rs` + `foo/`), not `mod.rs`.
  Consistent across the repo.
- Separate domain logic from I/O and from error types. Domain logic should be testable without
  a runtime, a filesystem, or a network.
- No wildcard imports except a crate's documented prelude and `use super::*` inside test modules.
- Feature flags for optional dependencies, with a CI job that builds `--no-default-features`.

## CLI

- `clap` with derive macros. `--help` and `--version` always work.
- Non-zero exit on failure; errors to stderr, data to stdout.
- Handle `ErrorKind::BrokenPipe` as a clean exit, not an error — otherwise piping into `head`
  produces spurious failures.
- No emoji or emoji-like Unicode (✓, ✗, ➜) in output, logs, or source. Plain ASCII markers.
- Long-running operations report progress (`indicatif`) and respect `--quiet` and non-TTY stdout.

## Data processing

- Stream with iterators; do not materialize an input into a `Vec` unless the algorithm needs
  random access.
- `rayon` for CPU-bound data parallelism, and only after measuring that the serial version is
  the bottleneck. The same applies to SIMD and any other optimization: measure first, and do not
  add complexity to code that isn't hot.
- `Vec::with_capacity` when the final size is known and trusted.
- Profile before optimizing. `cargo bench` (criterion) with a committed baseline for anything
  claimed to be performance-sensitive.

## Enforcement

These belong in the repo, not in prose. The agent should treat a failing lint as a failing build.

```toml
# Cargo.toml
[lints.rust]
unsafe_op_in_unsafe_fn = "deny"
missing_docs = "warn"
missing_debug_implementations = "warn"
unused_must_use = "deny"
unreachable_pub = "warn"

[lints.clippy]
# correctness / panics
unwrap_used = "deny"
expect_used = "warn"
panic = "warn"
todo = "deny"
dbg_macro = "deny"
indexing_slicing = "warn"
# API shape
ptr_arg = "deny"                      # catches &String / &Vec<T> params
needless_pass_by_value = "warn"
wildcard_enum_match_arm = "warn"
wildcard_imports = "warn"
# async
await_holding_lock = "deny"
await_holding_refcell_ref = "deny"
# hygiene
allow_attributes = "warn"             # prefer #[expect] over #[allow]
allow_attributes_without_reason = "warn"
inefficient_to_string = "warn"
# cherry-picked from pedantic — do not enable the whole group
doc_markdown = "warn"
must_use_candidate = "warn"
missing_errors_doc = "warn"
missing_panics_doc = "warn"
```

```toml
# rust-toolchain.toml
[toolchain]
channel = "1.XX.0"            # pin explicitly
components = ["rustfmt", "clippy"]
```

CI, and the definition of "done" for any change:

```
cargo fmt --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo deny check
cargo semver-checks          # published crates only
```