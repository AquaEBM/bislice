#[allow(clippy::wildcard_imports)]
use super::*;

pub struct Iter<'a, T> {
    start: core::slice::Iter<'a, T>,
    end: core::slice::Iter<'a, T>,
}

impl<T: fmt::Debug> fmt::Debug for Iter<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Iter")
            .field(&self.start.as_slice())
            .field(&self.end.as_slice())
            .finish()
    }
}

impl<T> Clone for Iter<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            start: self.start.clone(),
            end: self.end.clone(),
        }
    }
}

impl<T> Default for Iter<'_, T> {
    #[inline]
    fn default() -> Self {
        Self {
            start: core::slice::Iter::default(),
            end: core::slice::Iter::default(),
        }
    }
}

// TODO: make these const when the relevant std functions become const
impl<'a, T> Iter<'a, T> {
    #[inline]
    pub fn new(first: &'a [T], second: &'a [T]) -> Self {
        Self {
            start: first.iter(),
            end: second.iter(),
        }
    }

    #[inline]
    #[must_use]
    pub fn from_slice(slice: BiSlice<'a, T>) -> Self {
        let (first, second) = slice.into_slices();
        Self::new(first, second)
    }

    #[inline]
    #[must_use]
    pub fn as_ref(&self) -> BiSlice<'_, T> {
        BiSlice::new(self.start.as_slice(), self.end.as_slice())
    }
}

impl<'a, T> IntoIterator for BiSlice<'a, T> {
    type IntoIter = Iter<'a, T>;
    type Item = <Self::IntoIter as Iterator>::Item;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.start.next() {
            return Some(v);
        }
        self.end.next()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let split = self.start.as_slice().len();

        if let Some(x) = self.start.nth(n) {
            return Some(x);
        }

        self.end.nth(n.strict_sub(split))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.end.next_back() {
            return Some(v);
        }
        self.start.next_back()
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let rsplit = self.end.as_slice().len();

        if let Some(x) = self.end.nth_back(n) {
            return Some(x);
        }

        self.start.nth_back(n.strict_sub(rsplit))
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {
    #[inline]
    fn len(&self) -> usize {
        self.as_ref().len()
    }
}

impl<T> iter::FusedIterator for Iter<'_, T> {}
