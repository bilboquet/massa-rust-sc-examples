Examples of how to use massa-rust-sc-sdk

# Build

## candidate 1
/ ! \\ does not seem to make it, see candidate 2 (more raw but also more promising)
Kept as a source of information
`wasm-pack build --target web`

MAYBE useless if project is build with wasm-pack

## candidate 2
/ ! \\ this is fine but it breaks testing feature in vscode, so better set the target at build time.
to force the target to wasm add this file to the Rust project.

```toml
File: .cargo/config.toml
────────────────────────
[build]
target = "wasm32-unknown-unknown"
```

## build command
We need a post build hook to prepend the `header` (0x01) to the produced `.wasm` file.
AFAIK `cargo` does not have this feature. `cargo make` solves this problem in a simple way.
(maybe cargo xtask should be investigated)

to install `cargo make`
```shell
cargo install cargo-make
```

Classical `cargo` commands are still working, in order to fully build the projet in one command use:

to build the project
```shell
cargo make wasm
```

## test command
If the default target set in `.cargo/config.toml` is `wasm32-unknown-unknown`,
one need to set a different one for test to work, example:

```shell
cargo test -p massa-rust-sc-examples --lib --target x86_64-unknown-linux-gnu
```

# various
Experimental Rust is required, at least very useful to have `cargo expand` out of the box,
hence the `rust-toolchain.toml`.
```toml
File: rust-toolchain.toml
─────────────────────────
[toolchain]
channel = "nightly-2023-02-27"
```