use std::vec::IntoIter;

use crate::{Input, StringInput};

fn input(s: &'static str) -> IntoIter<String> {
    let v: Vec<String> = s.split(' ').map(ToString::to_string).collect();
    v.into_iter()
}

#[test]
fn test_no_dash_1() {
    let mut input = StringInput::new(input("ab c def"));
    assert_eq!(input.eat_no_dash("ab"), Some("ab"));
    assert_eq!(input.eat_no_dash("cd"), None);
    assert_eq!(input.eat_no_dash("c"), Some("c"));
    assert_eq!(input.eat_no_dash("de"), None);
    assert_eq!(input.eat_no_dash("def"), Some("def"));
    assert_eq!(input.eat_no_dash(""), None);
    assert!(input.is_empty());
}

#[test]
fn test_no_dash_2() {
    let mut input = StringInput::new(input("ab c-d=e -fg"));
    assert_eq!(input.eat_no_dash("ab"), Some("ab"));
    assert_eq!(input.eat_no_dash("c-d=e"), Some("c-d=e"));
    assert_eq!(input.eat_no_dash("fg"), None);
    assert_eq!(input.eat_no_dash("-fg"), None);
}

#[test]
fn test_no_dash_3() {
    let mut input = StringInput::new(input("ab --cd=e -fg"));
    input.bump(1);
    assert_eq!(input.eat_no_dash("b"), Some("b"));
    assert_eq!(input.eat_two_dashes("cd"), Some("cd"));
    assert_eq!(input.eat_no_dash("e"), None);
    assert_eq!(input.eat_value("e"), Some("e"));
    assert_eq!(input.eat_one_dash("f"), Some("f"));
    assert_eq!(input.eat_no_dash("g"), None);
}

#[test]
fn test_one_dash_1() {
    let mut input = StringInput::new(input("-cde=f -gh= - --"));
    assert_eq!(input.eat_one_dash("c"), Some("c"));
    assert_eq!(input.eat_one_dash("de"), Some("de"));
    assert_eq!(input.eat_value("f"), Some("f"));
    assert_eq!(input.eat_one_dash("gh"), Some("gh"));
    assert_eq!(input.eat_one_dash(""), None);
    assert_eq!(input.eat_value(""), Some(""));
    assert_eq!(input.eat_one_dash(""), Some(""));
    assert_eq!(input.eat_one_dash("-"), None);
    assert_eq!(input.eat_two_dashes(""), Some(""));
    assert_eq!(input.eat_one_dash(""), None);
}

#[test]
fn test_one_dash_2() {
    let mut input = StringInput::new(input("-a-b=c -d=e"));
    assert_eq!(input.eat_one_dash("a"), Some("a"));
    assert_eq!(input.eat_one_dash("-b"), Some("-b"));
    assert_eq!(input.eat_one_dash("="), None);
    assert_eq!(input.eat_value("c"), Some("c"));
    assert_eq!(input.eat_one_dash("d=e"), Some("d=e"));
    assert!(input.is_empty());
}

#[test]
fn test_one_dash_3() {
    let mut input = StringInput::new(input("--abc=-def -g=h i"));
    assert_eq!(input.eat_one_dash("-"), None);
    assert_eq!(input.eat_one_dash("a"), None);
    assert_eq!(input.eat_two_dashes("abc"), Some("abc"));
    assert_eq!(input.eat_one_dash("d"), None);
    assert_eq!(input.eat_one_dash("-def"), None);
    assert_eq!(input.eat_value("-def"), Some("-def"));
    assert_eq!(input.eat_one_dash("g"), Some("g"));
    assert_eq!(input.eat_one_dash("=h"), None);
    assert_eq!(input.eat_one_dash("h"), None);
    assert_eq!(input.eat_value("h"), Some("h"));
    assert_eq!(input.eat_one_dash("i"), None);
}

#[test]
fn test_two_dashes_1() {
    let mut input = StringInput::new(input("-- --abc --d=e --f=g"));
    assert_eq!(input.eat_two_dashes(""), Some(""));
    assert_eq!(input.eat_two_dashes("ab"), None);
    assert_eq!(input.eat_two_dashes("abc"), Some("abc"));
    assert_eq!(input.eat_two_dashes("d=e"), Some("d=e"));
    assert_eq!(input.eat_two_dashes("f"), Some("f"));
    assert_eq!(input.eat_value("g"), Some("g"));
    assert_eq!(input.eat_two_dashes(""), None);
    assert!(input.is_empty());
}

#[test]
fn test_two_dashes_2() {
    let mut input = StringInput::new(input("--a=b c--d -e--f"));
    assert_eq!(input.eat_two_dashes("a"), Some("a"));
    assert_eq!(input.eat_two_dashes("b"), None);
    assert_eq!(input.eat_value("b"), Some("b"));
    input.bump(1);
    assert_eq!(input.eat_two_dashes("d"), None);
    assert_eq!(input.eat_value("--d"), Some("--d"));
    assert_eq!(input.eat_one_dash("e"), Some("e"));
    assert_eq!(input.eat_two_dashes("f"), None);
}

#[test]
fn test_value() {
    let mut input = StringInput::new(input("ab -cde fg -hi --jk --l=-m -n=--o"));
    assert_eq!(input.eat_value("ab"), Some("ab"));
    assert_eq!(input.eat_one_dash("c"), Some("c"));
    assert_eq!(input.eat_value("de"), Some("de"));
    assert_eq!(input.eat_value("fg"), Some("fg"));
    assert_eq!(input.eat_value("-hi"), None);
    assert_eq!(input.eat_one_dash("hi"), Some("hi"));
    assert_eq!(input.eat_value("--jk"), None);
    assert_eq!(input.eat_two_dashes("jk"), Some("jk"));
    assert_eq!(input.eat_two_dashes("l"), Some("l"));
    assert_eq!(input.eat_value("-m"), Some("-m"));
    assert_eq!(input.eat_one_dash("n"), Some("n"));
    assert_eq!(input.eat_value("--o"), Some("--o"));
    assert!(input.is_empty());
}

#[test]
fn test_value_allows_leading_dashes() {
    let mut input = StringInput::new(input("ab -cde fg -hi --jk --l=-m -n=--o"));
    assert_eq!(input.eat_value_allows_leading_dashes("ab"), Some("ab"));
    assert_eq!(input.eat_value_allows_leading_dashes("-c"), None);
    assert_eq!(input.eat_value_allows_leading_dashes("-cde"), Some("-cde"));
    assert_eq!(input.eat_value_allows_leading_dashes("fg"), Some("fg"));
    assert_eq!(input.eat_value_allows_leading_dashes("-hi"), Some("-hi"));
    assert_eq!(input.eat_value_allows_leading_dashes("--jk"), Some("--jk"));
    assert_eq!(input.eat_two_dashes("l"), Some("l"));
    assert_eq!(input.eat_value_allows_leading_dashes("-m"), Some("-m"));
    assert_eq!(input.eat_one_dash("n"), Some("n"));
    assert_eq!(input.eat_value_allows_leading_dashes("--o"), Some("--o"));
    assert!(input.is_empty());
}
