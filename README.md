# koption_macros

Some macros that are useful for working with `Option`s.

- `or!`: `or!(optA => optB => optC)` will select the first non-`None` value, like a `COALESCE` in SQL.
- `and!`: `and!(optA => optB => optC)` will produce a tuple `(A, B, C)` iff all values are `Some`.
- `try_!`: This is an `Option` focused version of the try block which seems to work better with type inference than the `try` blocks in nightly.

There is at least one more thing planned as soon as I can figure out how to use proc_macros.

## Examples

Straight from the unit tests.

```rust
#[test]
fn or_works() {
    assert_eq!(Some(1), or!(Some(1) => Some(2) => Some(3)));
    assert_eq!(Some(2), or!(None => Some(2) => Some(3)));
    assert_eq!(Some(3), or!(None => None => Some(3)));
    assert_eq!(None::<()>, or!(None => None => None));
}

#[test]
fn and_works() {
    assert_eq!(Some((1, 2, 3)), and!(Some(1) => Some(2) => Some(3)));
    assert_eq!(None, and!(None => Some(2) => Some(3)));
    assert_eq!(None, and!(None => None => Some(3)));
    assert_eq!(None, and!(None => None => None));
}

struct Config {
    log: Option<LogConfig>,
}

struct LogConfig {
    level: Option<String>,
}

#[test]
fn try_works() {
    assert_eq!(
        Some(6),
        try_! {
            let x = Some(3);
            let y = Some(2);
            x? * y?
        }
    );

    let config = Config {
        log: Some(LogConfig {
            level: Some("debug".to_owned()),
        }),
    };

    let x = try_! { config.log?.level? }.unwrap_or("foo".to_owned());
    assert_eq!(x, "debug");
}
```

## Why `k`option_macros?

It's my version of namespacing my crates with my last initial, `k`.
