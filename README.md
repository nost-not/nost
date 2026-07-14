# Nost

Nost is a markdown-based note generator that adheres to the NOT format.

Whenever you need to take notes, Nost helps you create files following the structure: year/month/week number/day number.

For example, if you add a note on the 6th of June 2025:

```txt
2025/
  06/
    1/
      06.md
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

## Work sessions (WIP)

Begin a work session:

```sh
cargo run start-work
```

Or

```sh
cargo run sw
```

End a work session:

```sh
cargo run end-work
```

Or

```sh
cargo run ew
```

Display work stats:

```sh
cargo run work-stats
```

Or

```sh
cargo run ws
```

### Work plugin configuration

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
