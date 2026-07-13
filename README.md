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

## Create a note

```sh
cargo run new
```

Or

```sh
cargo run n
```

## (WIP) Use not for working

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

For now, the data is only annotated, but these annotations are not yet used or processed.

## Create a new folder

```sh
cargo run new-folder
```

Or

```sh
cargo run nf
```

## Test

Unit tests:

```
cargo test
```

Style:

```
cargo clippy --verbose -- -D warnings
```

Linter:

```
cargo fmt -- --check
```

## Configure the app

Copy `config.toml.dist` into `config.toml` and update the values. For example:

```toml
not_path="/path/to/your/notes"
language="fr"
```

## Build the app

Build the app with cargo

```sh
cargo build --release
```

Optional: add an alias

```sh
alias nost="RUST_LOG=warn /path/to/nost/target/release/nost"
```

## Work plugin

For computing work stats, add some env vars:

```sh
export NOST_WORK_SALARY=0
export NOST_WORK_CURRENCY=EUR
```
