# Advanced Cargo & Crates.io

## Development and Release Builds

For development,

```
cargo build --dev
```

And likewise for release,

```
cargo build --release
```

Where the options for both are specified in `cargo.toml`,

```toml
...

[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

## Autodocumentation

Running `cargo doc --open` to build docs (from comments), and then open up the HTML ingex page.
