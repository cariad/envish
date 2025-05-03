#[doc = include_str!("../README.md")]
use regex::{Error, Regex};
use std::{collections::HashMap, ffi::OsString};

/// The result of an `envish::group_by` operation.
///
/// The outer HashMap describes named groups of environment variables.
///
/// The inner HashMaps describe each environment variable's key and value.
type GroupByResult = HashMap<String, HashMap<String, OsString>>;

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

/// Gets a grouped collection of related environment variables.
///
/// The group expression must contain two named groups:
///
/// - `<group>` describes the substring that groups variables together.
/// - `<key>` describes the variable's key in the result.
///
/// ```rust
/// std::env::set_var("FOO_ALLY_AGE", "39");
/// std::env::set_var("FOO_ALLY_JOB", "assassin");
/// std::env::set_var("FOO_CHARLIE_AGE", "42");
/// std::env::set_var("FOO_CHARLIE_JOB", "jester");
///
/// let group_expr = r"FOO_(?<group>.+)_(?<key>.+)";
/// let mut groups = envish::group_by(group_expr).unwrap();
///
/// let ally = &groups["ALLY"];
/// assert_eq!(ally["AGE"], "39");
/// assert_eq!(ally["JOB"], "assassin");
///
/// let charlie = &groups["CHARLIE"];
/// assert_eq!(charlie["AGE"], "42");
/// assert_eq!(charlie["JOB"], "jester");
/// ```
pub fn group_by(expr: &str) -> Result<GroupByResult, Error> {
    match Regex::new(expr) {
        Ok(re) => {
            let mut groups = HashMap::new();

            for p in std::env::vars_os() {
                match re.captures(p.0.to_str().unwrap()) {
                    Some(captures) => {
                        let group_id = &captures["group"];

                        if !groups.contains_key(group_id) {
                            groups.insert(group_id.to_owned(), HashMap::new());
                        }

                        groups
                            .get_mut(group_id)
                            .unwrap()
                            .insert(captures["key"].to_string(), p.1);
                    }
                    None => continue,
                }
            }

            Ok(groups)
        }
        Err(e) => Err(e),
    }
}

/// Gets an iterator over environment variables with names that match a regular
/// expression.
///
/// ```rust
/// std::env::set_var("CAT_CLOWNS_WOO", "woo");
/// std::env::set_var("IMP_JESTER_FOO", "foo");
/// std::env::set_var("DOG_JUGGLE_WAR", "war");
/// std::env::set_var("IMP_JESTER_BAR", "bar");
///
/// let expr = "(.*)_JESTER_(.*)";
/// let mut matches = envish::search(expr).unwrap();
///
/// let m = matches.next().unwrap();
/// assert_eq!(m.0, "IMP_JESTER_FOO");
/// assert_eq!(m.1, "foo");
///
/// let m = matches.next().unwrap();
/// assert_eq!(m.0, "IMP_JESTER_BAR");
/// assert_eq!(m.1, "bar");
///
/// let m = matches.next();
/// assert!(m.is_none());
/// ```
pub fn search(expr: &str) -> Result<Search, Error> {
    Ok(Search {
        expr: Regex::new(expr)?,
        vars: std::env::vars_os(),
    })
}

/// Gets an iterator over environment variables with names that have a given
/// prefix.
///
/// ```rust
/// std::env::set_var("CAT_CLOWNS_WOO", "woo");
/// std::env::set_var("IMP_JESTER_FOO", "foo");
/// std::env::set_var("DOG_JUGGLE_WAR", "war");
/// std::env::set_var("IMP_JESTER_BAR", "bar");
///
/// let prefix = "IMP_JESTER_";
/// let mut matches = envish::with_prefix(prefix).unwrap();
///
/// let m = matches.next().unwrap();
/// assert_eq!(m.0, "IMP_JESTER_FOO");
/// assert_eq!(m.1, "foo");
///
/// let m = matches.next().unwrap();
/// assert_eq!(m.0, "IMP_JESTER_BAR");
/// assert_eq!(m.1, "bar");
///
/// let m = matches.next();
/// assert!(m.is_none());
/// ```
pub fn with_prefix(prefix: &str) -> Result<Search, Error> {
    search(&format!("{}(.*)", prefix))
}
