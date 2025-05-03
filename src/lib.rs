#[doc = include_str!("../README.md")]
use regex::{Error, Regex};
use std::ffi::OsString;

pub struct Search {
    expr: Regex,
    vars: std::env::VarsOs,
}

impl Iterator for Search {
    type Item = (OsString, OsString);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.vars.next() {
                Some(pair) => {
                    if self.expr.is_match(pair.0.to_str().unwrap()) {
                        return Some(pair);
                    }
                }
                None => {
                    return None;
                }
            }
        }
    }
}

/// Gets an iterator over environment variables with names that match the given
/// pattern.
///
/// ```rust
/// std::env::set_var("CAT_CLOWNS_WOO", "woo");
/// std::env::set_var("IMP_JESTER_FOO", "foo");
/// std::env::set_var("DOG_JUGGLE_WAR", "war");
/// std::env::set_var("IMP_JESTER_BAR", "bar");
///
/// let mut matches = envish::by_pattern("(.*)_JESTER_(.*)").unwrap();
///
/// let m = matches.next().unwrap();
///
/// assert_eq!(m.0, "IMP_JESTER_FOO");
/// assert_eq!(m.1, "foo");
///
/// let m = matches.next().unwrap();
///
/// assert_eq!(m.0, "IMP_JESTER_BAR");
/// assert_eq!(m.1, "bar");
///
/// let m = matches.next();
///
/// assert!(m.is_none());
/// ```
pub fn by_pattern(pattern: &str) -> Result<Search, Error> {
    Ok(Search {
        vars: std::env::vars_os(),
        expr: Regex::new(pattern)?,
    })
}

/// Gets an iterator over environment variables with names that have the given
/// prefix.
///
/// ```rust
/// std::env::set_var("CAT_CLOWNS_WOO", "woo");
/// std::env::set_var("IMP_JESTER_FOO", "foo");
/// std::env::set_var("DOG_JUGGLE_WAR", "war");
/// std::env::set_var("IMP_JESTER_BAR", "bar");
///
/// let mut matches = envish::with_prefix("IMP_JESTER_").unwrap();
///
/// let m = matches.next().unwrap();
///
/// assert_eq!(m.0, "IMP_JESTER_FOO");
/// assert_eq!(m.1, "foo");
///
/// let m = matches.next().unwrap();
///
/// assert_eq!(m.0, "IMP_JESTER_BAR");
/// assert_eq!(m.1, "bar");
///
/// let m = matches.next();
///
/// assert!(m.is_none());
/// ```
pub fn with_prefix(pattern: &str) -> Result<Search, Error> {
    by_pattern(&format!("{}(.*)", pattern))
}
