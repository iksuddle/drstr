# `durstr`

[<img alt="Crates.io Version" src="https://img.shields.io/crates/v/durstr?style=flat-square">](https://crates.io/crates/durstr)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/durstr?style=flat-square">](https://docs.rs/durstr)

A simple library for parsing human-readable duration strings into `std::time::Duration`.

## Usage

Add `durstr` to `Cargo.toml`:

```toml
[dependencies]
durstr = "0.2.0"
```

Then use the `parse` function:

```rust
use durstr::parse;
use std::time::Duration;

let dur = parse("12 minutes, 21 seconds");
assert_eq!(dur, Ok(Duration::from_secs(741)));

let dur = parse("1hr 2min 3sec");
assert_eq!(dur, Ok(Duration::from_secs(3723)));
```

## Supported Units

| Unit        | Aliases                               |
|-------------|---------------------------------------|
| Millisecond | `ms`, `msec`/`msecs`, `milliseconds`  |
| Second      | `s`, `sec`/`secs`, `seconds`          |
| Minute      | `m`, `min`/`mins`, `minutes`          |
| Hour        | `h`, `hr`/`hrs`, `hours`              |

## Future Enhancements

-   Floating point support
-   User-defined custom units
-   Case sensitivity option
