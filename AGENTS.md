# AGENTS.md — php_shakmaty

## What this is
A PHP extension (namespace `shakmaty\`) that wraps the Rust crate [shakmaty](https://crates.io/crates/shakmaty) v0.30 via ext-php-rs 0.15.15. This is NOT a Composer package: there is no `composer.json`, and the PHP API is generated from Rust. The build produces `target/release/php_shakmaty.dll` (Linux: `.so`); the loadable module name is `shakmaty`.

## Build
- `cargo build --release` (Rust edition 2024; on Windows — MSVC).
- `.cargo/config.toml` sets the mandatory env vars for the build (path to PHP, `LIBCLANG_PATH`, MSVC `INCLUDE`/`LIB`). Without it the build fails on Windows. The file is in `.gitignore`.

## Tests (the main trap)
`php.ini` (`D:\Workspace\OpenServer\modules\PHP-8.4\php.ini`) already loads a **shadow copy** of the extension from `D:\Workspace\OpenServer\modules\PHP-8.4\ext`. So running with `-d extension=target\release\php_shakmaty.dll` without `-n` yields `Module "shakmaty" is already loaded` and tests the stale DLL. Always run the freshly built DLL like this:

```bash
php -n -d extension=target\release\php_shakmaty.dll tests\integration.php
php -n -d zend.assertions=1 -d extension=target\release\php_shakmaty.dll tests\step25_zobrist_position.php
```

- `-n` disables php.ini; the extension is loaded via an explicit `-d extension=...`.
- `enable_dl=Off` in php.ini, so `dl()` in tests will not work — the extension must be passed via `-d`.
- Skipping `-n` = testing the shadow DLL (a frequent source of false "PASS").

- There is no PHPUnit/composer/test runner. A full run = `tests/integration.php` (one large script: prints `Passed`/`Failed`, `exit(1)` on failure, ends with `All integration tests passed!`) plus the per-step `tests/stepN_*.php`. After each phase `integration.php` is extended with new checks.
- Some step tests use `assert()` — this requires `-d zend.assertions=1`.

## Code conventions
- A Rust struct is named `Php<Name>`; the class is annotated with `#[php_class]` + `#[php(name = "shakmaty\\Name")]`; methods live in `#[php_impl]`, constants are `pub const`.
- ext-php-rs itself maps Rust `snake_case` → PHP `camelCase`. Tests, docs, and stubs use camelCase (`playSan`, `legalMovesCount`).
- shakmaty enum types are PHP classes with an integer field `inner` and `int` constants (not PHP `enum`).
- Complex types keep `pub inner: <shakmaty-type>` for access from other modules.
- Errors: Rust `Result<T, &'static str>` → PHP exception; for FEN, custom exception classes are used (see `src/fen_errors.rs`).
- The helpers `*_from_int` and `moves_to_php` are intentionally duplicated in `chess_pos.rs` and `variant_position.rs` (refactoring is out of phase scope — do not "fix" without a request).

## Class registration
Each module is added to `src/lib.rs` (`mod ...;` + `.class::<...>()` inside `get_module()`), and **order matters**: the parent is registered before the child (`PhpPosition` before `PhpChess`/`PhpVariantPosition`). Update the class count in the doc comment and in the comment next to `get_module()` (currently 42).

`Position` is an abstract class. ext-php-rs does NOT dispatch inherited methods declared only on the Rust parent, so the entire trait contract is re-declared in `position.rs` as `#[php(abstract)]`, and the implementations delegate to `chess_pos.rs` and `variant_position.rs`. A new `Position` contract method must be added to all three files.

## Repository structure
- `src/*.rs` — one module per group of classes; `src/pgn/` — the `shakmaty\pgn\*` classes.
- `library/` — vendored shakmaty/pgn-reader sources for reference only (gitignored; do not edit — Cargo takes the crates from crates.io).
- `stubs/` — hand-written PHP stubs for the IDE; they must match the real API, verify with `php -l <file>`.
- `docs/api/*.md`, `docs/examples/`, `docs/index.md`, `README.md` — hand-written documentation (English); sync the class table and counters in `README.md` manually.
- `.kilo/docs/plan.md` — phase history and retrospectives; append an entry after each phase is completed.

## Versioning (mandatory after every phase)
The single source of the version is the `version` field in `Cargo.toml` (there is no Composer manifest; `Cargo.lock` is updated automatically by `cargo build`).

The version bump is performed **at the very last step of a phase** — after changes to code, tests, stubs, and docs, but before the final commit: this way `Cargo.toml` and the updated `Cargo.lock` land in the phase's final commit. Do not bump the version at the start or in the middle of a phase.

After each phase is completed, increase the version:
- **patch** (`0.x.Y+1`) — if the phase contains only fixes/changes that do not affect the public PHP API (signatures, class/method names, behavior is compatible);
- **minor** (`0.X+1.0`) — if the public PHP API is added, changed, or removed (new classes/methods, signature changes, removal of a constructor, etc.).

When in doubt between patch and minor, choose **minor**. Record the fact of the bump in the phase entry in `.kilo/docs/plan.md` (was → became).

## Checklist for adding/changing a class
1. `src/*.rs` + registration and counter in `src/lib.rs`.
2. `tests/stepN_*.php` + new checks in `tests/integration.php`.
3. `stubs/<Class>.php` (+ `php -l`).
4. `docs/api/<...>.md`, `docs/index.md`, `README.md`.
5. A phase entry in `.kilo/docs/plan.md`.
6. `cargo build --release` + test run with `-n` (see above).
7. **Last step of the phase:** bump the version in `Cargo.toml` per the rule below (plus the updated `Cargo.lock`), then the final commit.

## Non-obvious behavior
- `Move::toLong()` — long notation without a hyphen, with a piece prefix: `"e2e4"`, `"Ng1f3"`; drops: `"N@f3"`, `"@e4"`.
- `Zobrist64::value()` — a signed PHP int (hashes ≥ 2^63 are negative); the exact value is obtained via `toHex()`.
- `Position::play()` returns a new position (does not mutate); `playUnchecked()` mutates.
