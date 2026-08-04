# Nost

Nost is a markdown-based note generator that adheres to the NOT format.

Whenever you need to take notes, Nost helps you create files following the structure: year/month/week number/day number/default-file-not.

For example, if you add a note on the 6th of June 2025:

```txt
2025/
  06/
    1/
      06/
        06/default.md
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (includes `cargo`)

## Build the app

```sh
cargo build --release
```

Optional: add an alias

```sh
alias nost="RUST_LOG=warn /path/to/nost/target/release/nost"
```

## Configure the app

Copy `config.toml.dist` into `config.toml` and update the values. For example:

```toml
not_path="/path/to/your/notes"
language="fr"
```

## Create a note

```sh
cargo run new
```

Or

```sh
cargo run n
```

### Create a note for a specific date (WIP)

By default a note is created for today. You can also pass an optional date
argument in `YYYY-MM-DD` format to create (or open) the note for that day:

```sh
cargo run new 2026-07-31
```

Or with the short alias:

```sh
cargo run n 2026-07-31
```

The date must be strictly formatted as `YYYY-MM-DD` (zero-padded month and day).
An invalid or malformed date exits with an error.

## Work sessions (WIP)

Begin a work session:

```sh
cargo run work
```

Or

```sh
cargo run w
```

End a work session:

```sh
cargo run work
```

Or

```sh
cargo run w
```

Display work stats:

```sh
cargo run stats
```

Or

```sh
cargo run s
```

### Work plugin configuration (WIP)

For computing work stats, add some env vars:

```sh
export NOST_WORK_SALARY=0
export NOST_WORK_CURRENCY=EUR
```

## Development

Unit tests:

```sh
cargo test
```

Style:

```sh
cargo clippy --verbose -- -D warnings
```

Linter:

```sh
cargo fmt -- --check
```
