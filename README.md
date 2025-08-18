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

## (WIP) Use not for working

Begin a work session:

```
cargo run start-work
```

End a work session:

```
cargo run stop-work
```

For now, the data is only annotated, but these annotations are not yet used or processed.

## Test

Unit tests:

```
cargo test
```

Style:

```
cargo clippy --verbose
```

Linter:

```
cargo fmt -- --check
```
