//! This module provides functionality for automatically generated help
//! messages.

use std::fmt;
use std::iter::FusedIterator;

/// This struct defines the possible values of a type representing a _value_.
/// See the [`crate::FromInputValue`] trait for more information.
#[derive(Debug)]
pub enum PossibleValues {
    /// A literal value. For example, use `String("1")` if the value `1` is
    /// accepted.
    String(String),

    /// A string describing the kind of accepted values. For example, use
    /// `Other("positive number")` if all positive numbers are accepted.
    Other(String),

    /// A list of possible values. For example:
    ///
    /// ```
    /// # use parkour::help::PossibleValues;
    ///
    /// PossibleValues::OneOf(vec![
    ///     PossibleValues::String("yes".into()),
    ///     PossibleValues::String("no".into()),
    ///     PossibleValues::Other("number".into()),
    /// ]);
    /// ```
    ///
    /// This variant allows nesting lists of possible values. When displaying
    /// them, they should be flattened automatically.
    OneOf(Vec<PossibleValues>),
}

/// This struct defines a possible value of a type representing a _value_.
/// See the [`crate::FromInputValue`] trait for more information.
///
/// This struct is used only as item when iterating over [`PossibleValues`].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PossibleValue<'a> {
    /// A literal value. For example, `String("1")` means the value `1` is
    /// accepted.
    String(&'a str),
    /// A string describing the kind of accepted values. For example,
    /// `Other("positive number")` means all positive numbers are accepted.
    Other(&'a str),
}

impl PartialEq for PossibleValues {
    fn eq(&self, other: &Self) -> bool {
        use PossibleValues::*;

        match (self, other) {
            (String(a), String(b)) | (Other(a), Other(b)) => a == b,
            (String(_), Other(_)) | (Other(_), String(_)) => false,
            (_, _) => self.iter().eq(other.iter()),
        }
    }
}

impl Eq for PossibleValues {}

impl fmt::Display for PossibleValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.iter().peekable();
        match iter.next() {
            Some(v) => {
                write!(f, "{}", v)?;
                while let Some(next) = iter.next() {
                    if iter.peek().is_some() {
                        f.write_str(", ")?;
                    } else {
                        f.write_str(" or ")?;
                    }
                    write!(f, "{}", next)?;
                }
                Ok(())
            }
            None => f.write_str("nothing"),
        }
    }
}

impl fmt::Display for PossibleValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PossibleValue::String(s) => write!(f, "`{}`", s.escape_debug()),
            PossibleValue::Other(o) => f.write_str(o),
        }
    }
}

impl PossibleValues {
    /// Returns an iterator over all the possible values. This iterator flattens
    /// [`PossibleValues::OneOf`].
    pub fn iter(&self) -> PossibleValueIter<'_> {
        PossibleValueIter { values: Some(self), index: 0, then: None }
    }
}

/// Iterator over possible values that flattens [`PossibleValues::OneOf`].
pub struct PossibleValueIter<'a> {
    values: Option<&'a PossibleValues>,
    index: usize,
    then: Option<Box<PossibleValueIter<'a>>>,
}

impl<'a> Iterator for PossibleValueIter<'a> {
    type Item = PossibleValue<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.values {
            Some(PossibleValues::String(s)) => {
                advance(self);
                Some(PossibleValue::String(s))
            }
            Some(PossibleValues::Other(o)) => {
                advance(self);
                Some(PossibleValue::Other(o))
            }

            Some(PossibleValues::OneOf(o)) => {
                let next = &o[self.index];
                if self.index + 1 >= o.len() {
                    advance(self);
                } else {
                    self.index += 1;
                }

                let then = std::mem::replace(
                    self,
                    PossibleValueIter { values: Some(next), index: 0, then: None },
                );
                self.then = Some(Box::new(then));
                self.next()
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.values {
            Some(PossibleValues::String(_)) => (1, Some(1)),
            Some(PossibleValues::Other(_)) => (1, Some(1)),
            Some(PossibleValues::OneOf(v)) => (v.len(), None),
            None => (0, None),
        }
    }
}

impl FusedIterator for PossibleValueIter<'_> {}

fn advance(iter: &mut PossibleValueIter) {
    if let Some(then) = iter.then.take() {
        *iter = *then;
    } else {
        iter.values = None;
    }
}

#[test]
fn test_values_iterator() {
    use PossibleValues::*;

    let values = OneOf(vec![
        OneOf(vec![String("A".into())]),
        OneOf(vec![OneOf(vec![String("B".into())])]),
        OneOf(vec![OneOf(vec![String("C".into()), String("D".into())])]),
        OneOf(vec![OneOf(vec![String("E".into()), OneOf(vec![String("F".into())])])]),
        String("G".into()),
        String("H".into()),
        OneOf(vec![OneOf(vec![OneOf(vec![String("I".into())])])]),
        OneOf(vec![OneOf(vec![OneOf(vec![String("J".into()), String("K".into())])])]),
    ]);
    let collected: Vec<_> = values.iter().collect();
    assert_eq!(
        collected,
        vec![
            PossibleValue::String("A"),
            PossibleValue::String("B"),
            PossibleValue::String("C"),
            PossibleValue::String("D"),
            PossibleValue::String("E"),
            PossibleValue::String("F"),
            PossibleValue::String("G"),
            PossibleValue::String("H"),
            PossibleValue::String("I"),
            PossibleValue::String("J"),
            PossibleValue::String("K"),
        ]
    );
}
