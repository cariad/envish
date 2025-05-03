# envish: Search for environment variables in Rust

`envish` is a Rust library for discovering, iterating and grouping environment variables.

## Installation

```bash
cargo add envish
```

## Examples

### Grouping environment variables

`envish::group_by` gets a grouped collection of related environment variables.

The group expression must contain two named groups:

- `<group>` describes the substring that groups variables together.
- `<key>` describes the variable's key in the result.

```rust
std::env::set_var("FOO_ALLY_AGE", "39");
std::env::set_var("FOO_ALLY_JOB", "assassin");
std::env::set_var("FOO_CHARLIE_AGE", "42");
std::env::set_var("FOO_CHARLIE_JOB", "jester");

let group_expr = r"FOO_(?<group>.+)_(?<key>.+)";
let mut groups = envish::group_by(group_expr).unwrap();

let ally = &groups["ALLY"];
assert_eq!(ally["AGE"], "39");
assert_eq!(ally["JOB"], "assassin");

let charlie = &groups["CHARLIE"];
assert_eq!(charlie["AGE"], "42");
assert_eq!(charlie["JOB"], "jester");
```

### Search environment variables by regular expression

`envish::search()` returns an iterator over all environment variables with names that match a given expression.

```rust
std::env::set_var("CAT_CLOWNS_WOO", "woo");
std::env::set_var("IMP_JESTER_FOO", "foo");
std::env::set_var("DOG_JUGGLE_WAR", "war");
std::env::set_var("IMP_JESTER_BAR", "bar");

let expr = "(.*)_JESTER_(.*)";
let mut matches = envish::search(expr).unwrap();

let m = matches.next().unwrap();
assert_eq!(m.0, "IMP_JESTER_FOO");
assert_eq!(m.1, "foo");

let m = matches.next().unwrap();
assert_eq!(m.0, "IMP_JESTER_BAR");
assert_eq!(m.1, "bar");

let m = matches.next();
assert!(m.is_none());
```

### Search environment variables by prefix

`envish::with_prefix()` returns an iterator over all environment variables with the given prefix.

```rust
std::env::set_var("CAT_CLOWNS_WOO", "woo");
std::env::set_var("IMP_JESTER_FOO", "foo");
std::env::set_var("DOG_JUGGLE_WAR", "war");
std::env::set_var("IMP_JESTER_BAR", "bar");

let prefix = "IMP_JESTER_";
let mut matches = envish::with_prefix(prefix).unwrap();

let m = matches.next().unwrap();
assert_eq!(m.0, "IMP_JESTER_FOO");
assert_eq!(m.1, "foo");

let m = matches.next().unwrap();
assert_eq!(m.0, "IMP_JESTER_BAR");
assert_eq!(m.1, "bar");

let m = matches.next();
assert!(m.is_none());
```

## Support

Please submit all your questions, feature requests and bug reports at [github.com/cariad/envish/issues](https://github.com/cariad/envish/issues). Thank you!

## License

The library is [open-source](https://github.com/cariad/envish) and published under the [MIT License](https://github.com/cariad/envish/blob/main/LICENSE).

## Author

Hello! 👋 I'm Cariad Eccleston. You can find me at [cariad.earth](https://www.cariad.earth), [github.com/cariad](https://github.com/cariad), [linkedin.com/in/cariad](https://linkedin.com/in/cariad) and [@cariad.earth](https://bsky.app/profile/cariad.earth) on Bluesky.
