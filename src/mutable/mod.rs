#[allow(clippy::wildcard_imports)]
use super::*;
mod iterator;
pub use iterator::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BiSliceMut<'a, T> {
    start: &'a mut [T],
    end: &'a mut [T],
}

impl<T> Default for BiSliceMut<'_, T> {
    #[inline]
    fn default() -> Self {
        Self {
            start: Default::default(),
            end: Default::default(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for BiSliceMut<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.start)?;
        write!(f, "{:?}", self.end)
    }
}

impl<'a, T> BiSliceMut<'a, T> {
    #[inline]
    pub const fn new(start: &'a mut [T], end: &'a mut [T]) -> Self {
        Self { start, end }
    }

    #[inline]
    pub const fn from_single(slice: &'a mut [T]) -> Self {
        Self::new(slice, &mut [])
    }

    #[inline]
    #[must_use]
    pub const fn into_mut_slices(self) -> (&'a mut [T], &'a mut [T]) {
        (self.start, self.end)
    }
}

impl<'a, T> From<&'a mut [T]> for BiSliceMut<'a, T> {
    #[inline]
    fn from(value: &'a mut [T]) -> Self {
        Self::from_single(value)
    }
}

impl<'a, T> From<(&'a mut [T], )> for BiSliceMut<'a, T> {
    fn from((value, ): (&'a mut [T], )) -> Self {
        Self::from_single(value)
    }
}

impl<'a, T> From<[&'a mut [T] ; 1]> for BiSliceMut<'a, T> {
    fn from([value]: [&'a mut [T] ; 1]) -> Self {
        Self::from_single(value)
    }
}

impl<'a, T> From<(&'a mut [T], &'a mut [T])> for BiSliceMut<'a, T> {
    #[inline]
    fn from((start, end): (&'a mut [T], &'a mut [T])) -> Self {
        Self::new(start, end)
    }
}

impl<'a, T> From<[&'a mut [T]; 2]> for BiSliceMut<'a, T> {
    #[inline]
    fn from([start, end]: [&'a mut [T]; 2]) -> Self {
        Self::new(start, end)
    }
}

impl<'a, T> From<BiSliceMut<'a, T>> for (&'a mut [T], &'a mut [T]) {
    #[inline]
    fn from(value: BiSliceMut<'a, T>) -> Self {
        value.into_mut_slices()
    }
}

impl<'a, T> From<BiSliceMut<'a, T>> for (&'a mut [T], &'a [T]) {
    #[inline]
    fn from(value: BiSliceMut<'a, T>) -> Self {
        let (start, end) = value.into_mut_slices();
        (start, end)
    }
}

impl<'a, T> From<BiSliceMut<'a, T>> for (&'a [T], &'a mut [T]) {
    #[inline]
    fn from(value: BiSliceMut<'a, T>) -> Self {
        let (start, end) = value.into_mut_slices();
        (start, end)
    }
}

impl<'a, T> From<BiSliceMut<'a, T>> for (&'a [T], &'a [T]) {
    #[inline]
    fn from(value: BiSliceMut<'a, T>) -> Self {
        let (start, end) = value.into_mut_slices();
        (start, end)
    }
}

impl<'a, T> From<BiSliceMut<'a, T>> for [&'a mut [T]; 2] {
    #[inline]
    fn from(value: BiSliceMut<'a, T>) -> Self {
        value.into_mut_slices().into()
    }
}

impl<'a, T> From<BiSliceMut<'a, T>> for [&'a [T]; 2] {
    #[inline]
    fn from(value: BiSliceMut<'a, T>) -> Self {
        let [start, end] = value.into_mut_slices().into();
        [start, end]
    }
}

impl<'a, T> BiSliceMut<'a, T> {
    #[inline]
    #[must_use]
    pub const fn reborrow_mut(&mut self) -> BiSliceMut<'_, T> {
        BiSliceMut::new(self.start, self.end)
    }

    #[inline]
    #[must_use]
    pub const fn reborrow(&self) -> BiSlice<'_, T> {
        BiSlice::new(self.start, self.end)
    }

    #[inline]
    #[must_use]
    pub const fn into_ref(self) -> BiSlice<'a, T> {
        BiSlice::new(self.start, self.end)
    }

    #[inline]
    #[must_use]
    pub fn get_mut(self, index: usize) -> Option<&'a mut T> {
        let (start, end) = self.into_mut_slices();
        if let Some(i) = index.checked_sub(start.len()) {
            end.get_mut(i)
        } else {
            start.get_mut(index)
        }
    }

    #[inline]
    pub fn slice_mut(self, range: impl ops::RangeBounds<usize>) -> Option<Self> {
        let this_len = self.reborrow().len();
        let (mut start, mut end) = self.into_mut_slices();

        let (start_idx, end_idx) = normalize_range(
            range.start_bound().cloned(),
            range.end_bound().cloned(),
            this_len,
        );

        let range_len = end_idx.checked_sub(start_idx)?;

        if let Some(k) = start_idx.checked_sub(start.len()) {
            start = &mut [];
            end = end.get_mut(k..)?;
        } else {
            start = &mut start[start_idx..];
        }

        if let Some(k) = range_len.checked_sub(start.len()) {
            end = end.get_mut(..k)?;
        } else {
            end = &mut [];
            start = &mut start[..range_len];
        }

        Some(Self::new(start, end))
    }

    #[inline]
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn copy_from_slice(self, source: BiSlice<'_, T>) -> Option<()>
    where
        T: Copy,
    {
        if self.reborrow().len() != source.len() {
            return None;
        }

        let (src_start, src_end) = source.into_slices();
        let (dest_start, dest_end) = self.into_mut_slices();

        let src_split = src_start.len();
        let dest_split = dest_start.len();

        let mid_len = dest_split.abs_diff(src_split);

        if src_split >= dest_split {
            let (src_start, src_mid) = src_start.split_at_checked(dest_split).unwrap();
            let (dest_mid, dest_end) = dest_end.split_at_mut_checked(mid_len).unwrap();
            dest_start.copy_from_slice(src_start);
            dest_mid.copy_from_slice(src_mid);
            dest_end.copy_from_slice(src_end);
        } else {
            let (src_mid, src_end) = src_end.split_at_checked(mid_len).unwrap();
            let (dest_start, dest_mid) = dest_start.split_at_mut_checked(src_split).unwrap();
            dest_start.copy_from_slice(src_start);
            dest_mid.copy_from_slice(src_mid);
            dest_end.copy_from_slice(src_end);
        }

        Some(())
    }

    #[inline]
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn clone_from_slice(self, source: BiSlice<'_, T>) -> Option<()>
    where
        T: Clone,
    {
        if self.reborrow().len() != source.len() {
            return None;
        }

        let (src_start, src_end) = source.into_slices();
        let (dest_start, dest_end) = self.into_mut_slices();

        let src_split = src_start.len();
        let dest_split = dest_start.len();

        let mid_len = dest_split.abs_diff(src_split);

        if src_split >= dest_split {
            let (src_start, src_mid) = src_start.split_at_checked(dest_split).unwrap();
            let (dest_mid, dest_end) = dest_end.split_at_mut_checked(mid_len).unwrap();
            dest_start.clone_from_slice(src_start);
            dest_mid.clone_from_slice(src_mid);
            dest_end.clone_from_slice(src_end);
        } else {
            let (src_mid, src_end) = src_end.split_at_checked(mid_len).unwrap();
            let (dest_start, dest_mid) = dest_start.split_at_mut_checked(src_split).unwrap();
            dest_start.clone_from_slice(src_start);
            dest_mid.clone_from_slice(src_mid);
            dest_end.clone_from_slice(src_end);
        }

        Some(())
    }

    #[inline]
    #[must_use]
    pub fn split_at(self, mid: usize) -> Option<(Self, Self)> {
        if let Some(i) = mid.checked_sub(self.start.len()) {
            self.end
                .split_at_mut_checked(i)
                .map(|(start, end)| (Self::new(self.start, start), Self::from_single(end)))
        } else {
            let (start, end) = self.start.split_at_mut(mid);
            Some((Self::from_single(start), Self::new(end, self.end)))
        }
    }

    #[inline]
    #[must_use]
    pub fn split_first(self) -> Option<(&'a mut T, Self)> {
        if let Some((first, rem)) = self.start.split_first_mut() {
            Some((first, Self::new(rem, self.end)))
        } else {
            self.end
                .split_first_mut()
                .map(|(first, rem)| (first, Self::from_single(rem)))
        }
    }

    #[inline]
    #[must_use]
    pub fn split_last(self) -> Option<(&'a mut T, Self)> {
        if let Some((last, rem)) = self.end.split_last_mut() {
            Some((last, Self::new(self.start, rem)))
        } else {
            self.start
                .split_last_mut()
                .map(|(first, rem)| (first, Self::from_single(rem)))
        }
    }

    #[inline]
    #[must_use]
    pub fn iter_mut(self) -> IterMut<'a, T> {
        IterMut::from_slice(self)
    }
}

impl<'a, T> BiSliceMut<'a, mem::MaybeUninit<T>> {
    #[inline]
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn write_copy_of_slice(self, slice: BiSlice<'_, T>) -> Option<BiSliceMut<'a, T>>
    where
        T: Copy,
    {
        if self.reborrow().len() != slice.len() {
            return None;
        }

        let (src_start, src_end) = slice.into_slices();
        let (dest_start, dest_end) = self.into_mut_slices();

        let src_split = src_start.len();
        let dest_split = dest_start.len();

        let mid_len = dest_split.abs_diff(src_split);

        if src_split >= dest_split {
            let (src_start, src_mid) = src_start.split_at_checked(dest_split).unwrap();
            let (dest_mid, dest_end) = dest_end.split_at_mut_checked(mid_len).unwrap();
            dest_start.write_copy_of_slice(src_start);
            dest_mid.write_copy_of_slice(src_mid);
            dest_end.write_copy_of_slice(src_end);
        } else {
            let (src_mid, src_end) = src_end.split_at_checked(mid_len).unwrap();
            let (dest_start, dest_mid) = dest_start.split_at_mut_checked(src_split).unwrap();
            dest_start.write_copy_of_slice(src_start);
            dest_mid.write_copy_of_slice(src_mid);
            dest_end.write_copy_of_slice(src_end);
        }

        Some(BiSliceMut::new(
            unsafe { dest_start.assume_init_mut() },
            unsafe { dest_end.assume_init_mut() },
        ))
    }

    #[inline]
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn write_clone_of_slice(self, slice: BiSlice<'_, T>) -> Option<BiSliceMut<'a, T>>
    where
        T: Clone,
    {
        if self.reborrow().len() != slice.len() {
            return None;
        }

        let (src_start, src_end) = slice.into_slices();
        let (dest_start, dest_end) = self.into_mut_slices();

        let src_split = src_start.len();
        let dest_split = dest_start.len();

        let mid_len = dest_split.abs_diff(src_split);

        if src_split >= dest_split {
            let (src_start, src_mid) = src_start.split_at_checked(dest_split).unwrap();
            let (dest_mid, dest_end) = dest_end.split_at_mut_checked(mid_len).unwrap();
            dest_start.write_clone_of_slice(src_start);
            dest_mid.write_clone_of_slice(src_mid);
            dest_end.write_clone_of_slice(src_end);
        } else {
            let (src_mid, src_end) = src_end.split_at_checked(mid_len).unwrap();
            let (dest_start, dest_mid) = dest_start.split_at_mut_checked(src_split).unwrap();
            dest_start.write_clone_of_slice(src_start);
            dest_mid.write_clone_of_slice(src_mid);
            dest_end.write_clone_of_slice(src_end);
        }

        Some(BiSliceMut::new(
            unsafe { dest_start.assume_init_mut() },
            unsafe { dest_end.assume_init_mut() },
        ))
    }
}

impl<T> ops::Index<usize> for BiSliceMut<'_, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.reborrow().get(index).expect("index out of bounds")
    }
}

impl<T> ops::IndexMut<usize> for BiSliceMut<'_, T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.reborrow_mut()
            .get_mut(index)
            .expect("index out of bounds")
    }
}
