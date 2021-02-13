use std::{fmt, iter::FusedIterator};

use crate::input::PalrInput;

impl<S: Into<String>> PalrInput<VecStringIter<S>> {
    /// Create a [`PalrInput`] from a [`Vec`] of things that can be converted to [`String`].
    /// This is useful for unit tests.
    pub fn new_from_vec(args: Vec<S>) -> Self {
        Self::new(VecStringIter {
            iter: args.into_iter(),
        })
    }

    /// Create a [`PalrInput`] from an iterator over things that can be converted to [`String`].
    /// This is useful for unit tests.
    pub fn new_from_iter(args: impl Iterator<Item = S>) -> Self {
        Self::new(VecStringIter {
            iter: args.collect::<Vec<_>>().into_iter(),
        })
    }
}

/// An iterator over a [`Vec`] of things that can be converted to [`String`]
#[derive(Clone)]
pub struct VecStringIter<S> {
    iter: std::vec::IntoIter<S>,
}

impl<E: fmt::Debug> fmt::Debug for VecStringIter<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter.fmt(f)
    }
}

impl<S: Into<String>> Iterator for VecStringIter<S> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Into::into)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<S: Into<String>> DoubleEndedIterator for VecStringIter<S> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Into::into)
    }
}

impl<S: Into<String>> ExactSizeIterator for VecStringIter<S> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<S: Into<String>> FusedIterator for VecStringIter<S> {}
