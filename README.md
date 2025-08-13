# `durstr`

A simple library for parsing human-readable duration strings into `std::time::Duration`.

## Usage

Add `durstr` to `Cargo.toml`:

```toml
[dependencies]
durstr = "0.1.0"
```

Then use the `parse` function:

```rust
use durstr::parse;
use std::time::Duration;

let duration = parse("1h 2min 3s").unwrap();
assert_eq!(duration, Duration::from_secs(3723));
```

## Supported Units

| Unit        | Aliases                             |
|-------------|-------------------------------------|
| Millisecond | `ms`, `msec`, `milliseconds`        |
| Second      | `s`, `sec`, `seconds`               |
| Minute      | `m`, `min`, `minutes`               |
| Hour        | `h`, `hr`, `hours`                  |

## Future Enhancements

-   Floating point support
-   User-defined custom units (e.g., days, weeks)
-   Case sensitivity option
-   Improved documentation
