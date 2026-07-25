#[allow(clippy::wildcard_imports)]
use super::*;

pub struct IterMut<'a, T> {
    start: core::slice::IterMut<'a, T>,
    end: core::slice::IterMut<'a, T>,
}

impl<T: fmt::Debug> fmt::Debug for IterMut<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IterMut")
            .field(&self.start.as_slice())
            .field(&self.end.as_slice())
            .finish()
    }
}

impl<T> Default for IterMut<'_, T> {
    #[inline]
    fn default() -> Self {
        Self {
            start: core::slice::IterMut::default(),
            end: core::slice::IterMut::default(),
        }
    }
}

// TODO: make these const when the relevant std functions become const
impl<'a, T> IterMut<'a, T> {
    #[inline]
    pub fn new(first: &'a mut [T], second: &'a mut [T]) -> Self {
        Self {
            start: first.iter_mut(),
            end: second.iter_mut(),
        }
    }

    #[inline]
    #[must_use]
    pub fn from_slice(slice: BiSliceMut<'a, T>) -> Self {
        let (first, second) = slice.into_mut_slices();
        Self::new(first, second)
    }

    #[inline]
    #[must_use]
    pub fn into_slice(self) -> BiSliceMut<'a, T> {
        BiSliceMut::new(self.start.into_slice(), self.end.into_slice())
    }

    #[inline]
    #[must_use]
    pub fn as_ref(&self) -> BiSlice<'_, T> {
        BiSlice::new(self.start.as_slice(), self.end.as_slice())
    }

    // TODO: as_mut when the as_mut_slice std function on single slice iterators lands
}

impl<'a, T> IntoIterator for BiSliceMut<'a, T> {
    type IntoIter = IterMut<'a, T>;
    type Item = <Self::IntoIter as Iterator>::Item;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

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

impl<T> DoubleEndedIterator for IterMut<'_, T> {
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

impl<T> ExactSizeIterator for IterMut<'_, T> {
    #[inline]
    fn len(&self) -> usize {
        self.as_ref().len()
    }
}

impl<T> iter::FusedIterator for IterMut<'_, T> {}
