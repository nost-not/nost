# Nost

Nost is a markdown-based note generator that adheres to the NOT format.

Whenever you need to take notes, Nost helps you create files following the structure: year/month/week number/day number.

For example, if you add a note on the 6th of June 2025:

```txt
  2025/
    06/
      1/
      03.md
```

## Create a not

```
cargo run not
```

Or

```
cargo run n
```

## (WIP) Use not for working

Begin a work session:

```
cargo run start-work
```

Or

```
cargo run sw
```

End a work session:

```
cargo run end-work
```

Or

```
cargo run ew
```

For now, the data is only annotated, but these annotations are not yet used or processed.

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

## Build the app

Build the app with cargo

```sh
cargo build --release
```

In your shell config, add an environment variable with the location of the folder where you want to store your notes.

```sh
export NOST_NOT_PATH=/home/gaetan/not

```

Optional: add an alias

```sh
alias nost="/home/gaetan/dev/nost-not/nost/target/release/nost"
```

## Configure the app

Optional: set an env var for the language

For now, only French (fr) and English (default) are supported

```sh
NOST_LANGUAGE=fr
```

For computing work stats, add some env vars:

```sh
export NOST_WORK_SALARY=0
export NOST_WORK_CURRENCY=EUR
```
