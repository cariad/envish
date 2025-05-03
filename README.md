# envish: Search for environment variables in Rust

## Installation

```bash
cargo add envish
```

## Examples

### Find environment variables with a pattern

`envish::with_prefix()` returns an iterator over all environment variables with the given prefix.

```rust
std::env::set_var("CAT_CLOWNS_WOO", "woo");
std::env::set_var("IMP_JESTER_FOO", "foo");
std::env::set_var("DOG_JUGGLE_WAR", "war");
std::env::set_var("IMP_JESTER_BAR", "bar");

let mut matches = envish::by_pattern("(.*)_JESTER_(.*)").unwrap();

let m = matches.next().unwrap();

assert_eq!(m.0, "IMP_JESTER_FOO");
assert_eq!(m.1, "foo");

let m = matches.next().unwrap();

assert_eq!(m.0, "IMP_JESTER_BAR");
assert_eq!(m.1, "bar");

let m = matches.next();

assert!(m.is_none());
```

### Find environment variables with a prefix

`envish::with_prefix()` returns an iterator over all environment variables with the given prefix.

```rust
std::env::set_var("CAT_CLOWNS_WOO", "woo");
std::env::set_var("IMP_JESTER_FOO", "foo");
std::env::set_var("DOG_JUGGLE_WAR", "war");
std::env::set_var("IMP_JESTER_BAR", "bar");

let mut matches = envish::with_prefix("IMP_JESTER_").unwrap();

let m = matches.next().unwrap();

assert_eq!(m.0, "IMP_JESTER_FOO");
assert_eq!(m.1, "foo");

let m = matches.next().unwrap();

assert_eq!(m.0, "IMP_JESTER_BAR");
assert_eq!(m.1, "bar");

let m = matches.next();

assert!(m.is_none());
```

## Author

Hello! 👋 I'm Cariad Eccleston. You can find me at [cariad.earth](https://www.cariad.earth), [github.com/cariad](https://github.com/cariad), [linkedin.com/in/cariad](https://linkedin.com/in/cariad) and [@cariad.earth](https://bsky.app/profile/cariad.earth) on Bluesky.
