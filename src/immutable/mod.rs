#[allow(clippy::wildcard_imports)]
use super::*;
mod iterator;
pub use iterator::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BiSlice<'a, T> {
    start: &'a [T],
    end: &'a [T],
}

impl<T> Default for BiSlice<'_, T> {
    #[inline]
    fn default() -> Self {
        Self {
            start: Default::default(),
            end: Default::default(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for BiSlice<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.start)?;
        write!(f, "{:?}", self.end)
    }
}

impl<T> Clone for BiSlice<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for BiSlice<'_, T> {}

impl<'a, T> BiSlice<'a, T> {
    #[inline]
    pub const fn new(start: &'a [T], end: &'a [T]) -> Self {
        Self { start, end }
    }

    #[inline]
    pub const fn from_single(slice: &'a [T]) -> Self {
        Self::new(slice, &[])
    }

    #[inline]
    #[must_use]
    pub const fn into_slices(self) -> (&'a [T], &'a [T]) {
        (self.start, self.end)
    }
}

impl<'a, T> From<&'a [T]> for BiSlice<'a, T> {
    #[inline]
    fn from(value: &'a [T]) -> Self {
        Self::from_single(value)
    }
}

impl<'a, T> From<&'a mut [T]> for BiSlice<'a, T> {
    #[inline]
    fn from(value: &'a mut [T]) -> Self {
        Self::from_single(value)
    }
}

impl<'a, T> From<(&'a [T],)> for BiSlice<'a, T> {
    fn from((value,): (&'a [T],)) -> Self {
        Self::from_single(value)
    }
}

impl<'a, T> From<(&'a mut [T],)> for BiSlice<'a, T> {
    fn from((value,): (&'a mut [T],)) -> Self {
        Self::from_single(value)
    }
}

impl<'a, T> From<[&'a [T]; 1]> for BiSlice<'a, T> {
    fn from([value]: [&'a [T]; 1]) -> Self {
        Self::from_single(value)
    }
}

impl<'a, T> From<[&'a mut [T]; 1]> for BiSlice<'a, T> {
    fn from([value]: [&'a mut [T]; 1]) -> Self {
        Self::from_single(value)
    }
}

impl<'a, T> From<(&'a [T], &'a [T])> for BiSlice<'a, T> {
    #[inline]
    fn from((start, end): (&'a [T], &'a [T])) -> Self {
        Self::new(start, end)
    }
}

impl<'a, T> From<(&'a mut [T], &'a [T])> for BiSlice<'a, T> {
    #[inline]
    fn from((start, end): (&'a mut [T], &'a [T])) -> Self {
        Self::new(start, end)
    }
}

impl<'a, T> From<(&'a [T], &'a mut [T])> for BiSlice<'a, T> {
    #[inline]
    fn from((start, end): (&'a [T], &'a mut [T])) -> Self {
        Self::new(start, end)
    }
}

impl<'a, T> From<(&'a mut [T], &'a mut [T])> for BiSlice<'a, T> {
    #[inline]
    fn from((start, end): (&'a mut [T], &'a mut [T])) -> Self {
        Self::new(start, end)
    }
}

impl<'a, T> From<BiSlice<'a, T>> for (&'a [T], &'a [T]) {
    #[inline]
    fn from(value: BiSlice<'a, T>) -> Self {
        value.into_slices()
    }
}

impl<'a, T> From<[&'a [T]; 2]> for BiSlice<'a, T> {
    #[inline]
    fn from([start, end]: [&'a [T]; 2]) -> Self {
        Self::new(start, end)
    }
}

impl<'a, T> From<[&'a mut [T]; 2]> for BiSlice<'a, T> {
    #[inline]
    fn from([start, end]: [&'a mut [T]; 2]) -> Self {
        Self::new(start, end)
    }
}

impl<'a, T> From<BiSlice<'a, T>> for [&'a [T]; 2] {
    #[inline]
    fn from(value: BiSlice<'a, T>) -> Self {
        value.into_slices().into()
    }
}

impl<'a, T> From<BiSliceMut<'a, T>> for BiSlice<'a, T> {
    #[inline]
    fn from(value: BiSliceMut<'a, T>) -> Self {
        let (start, end) = value.into_mut_slices();
        Self { start, end }
    }
}

impl<'a, T> BiSlice<'a, T> {
    #[inline]
    #[must_use]
    pub const fn len(self) -> usize {
        let (start, end) = self.into_slices();
        start.len().strict_add(end.len())
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.len() == 0
    }

    #[inline]
    #[must_use]
    pub fn get(self, index: usize) -> Option<&'a T> {
        let (start, end) = self.into_slices();
        if let Some(i) = index.checked_sub(start.len()) {
            end.get(i)
        } else {
            start.get(index)
        }
    }

    #[inline]
    pub fn slice(self, range: impl ops::RangeBounds<usize>) -> Option<Self> {
        let this_len = self.len();
        let (mut start, mut end) = self.into_slices();

        let (start_idx, end_idx) = normalize_range(
            range.start_bound().cloned(),
            range.end_bound().cloned(),
            this_len,
        );

        let range_len = end_idx.checked_sub(start_idx)?;

        if let Some(k) = start_idx.checked_sub(start.len()) {
            start = &[];
            end = end.get(k..)?;
        } else {
            start = &start[start_idx..];
        }

        if let Some(k) = range_len.checked_sub(start.len()) {
            end = end.get(..k)?;
        } else {
            end = &[];
            start = &start[..range_len];
        }

        Some(Self::new(start, end))
    }

    #[inline]
    #[must_use]
    pub fn split_at(self, mid: usize) -> Option<(Self, Self)> {
        let split = self.start.len();

        self.start
            .split_at_checked(mid)
            .map(|(start, end)| (Self::from_single(start), Self::new(end, self.end)))
            .or_else(|| {
                self.end
                    .split_at_checked(mid.strict_sub(split))
                    .map(|(start, end)| (Self::new(self.start, start), Self::from_single(end)))
            })
    }

    #[inline]
    #[must_use]
    pub fn split_first(self) -> Option<(&'a T, Self)> {
        self.start
            .split_first()
            .map(|(first, rem)| (first, Self::new(rem, self.end)))
            .or_else(|| {
                self.end
                    .split_first()
                    .map(|(first, rem)| (first, Self::from_single(rem)))
            })
    }

    #[inline]
    #[must_use]
    pub fn split_last(self) -> Option<(&'a T, Self)> {
        self.end
            .split_last()
            .map(|(last, rem)| (last, Self::new(self.start, rem)))
            .or_else(|| {
                self.start
                    .split_last()
                    .map(|(first, rem)| (first, Self::from_single(rem)))
            })
    }

    #[inline]
    #[must_use]
    pub fn iter(self) -> Iter<'a, T> {
        Iter::from_slice(self)
    }
}

impl<T> ops::Index<usize> for BiSlice<'_, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}

#[cfg(feature = "std")]
impl std::io::Read for BiSlice<'_, u8> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let amt = buf.len().min(self.len());
        let (a, b) = self.split_at(amt).unwrap();

        BiSliceMut::from_single(&mut buf[..amt])
            .copy_from_slice(a)
            .unwrap();

        *self = b;
        Ok(amt)
    }

    fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut<'_>]) -> std::io::Result<usize> {
        let mut nread = 0usize;
        for buf in bufs {
            nread = nread.strict_add(self.read(buf)?);
            if self.is_empty() {
                break;
            }
        }

        Ok(nread)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        let len = self.len();
        buf.try_reserve(len)?;
        let (start, end) = self.into_slices();
        buf.extend_from_slice(start);
        buf.extend_from_slice(end);
        *self = self.slice(len..).unwrap();
        Ok(len)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        (self.read(buf)? >= self.len()).ok_or_else(|| std::io::ErrorKind::UnexpectedEof.into())
    }
}
