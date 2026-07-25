#![cfg_attr(not(feature = "std"), no_std)]
use core::{fmt, iter, mem, ops};

mod immutable;
mod mutable;

pub use immutable::*;
pub use mutable::*;

#[inline]
#[must_use]
const fn normalize_range(
    start: ops::Bound<usize>,
    end: ops::Bound<usize>,
    len: usize,
) -> (usize, usize) {
    let start_idx = match start {
        ops::Bound::Included(n) => n,
        ops::Bound::Excluded(n) => n.strict_add(1),
        ops::Bound::Unbounded => 0,
    };

    let end_idx = match end {
        ops::Bound::Included(n) => n.strict_add(1),
        ops::Bound::Excluded(n) => n,
        ops::Bound::Unbounded => len,
    };

    (start_idx, end_idx)
}

#[inline]
#[must_use]
const fn aligned_portions<'a, 'b, T, U>(
    a: BiSliceMut<'a, T>,
    b: BiSlice<'b, U>,
) -> Option<[(&'a mut [T], &'b [U]); 3]> {
    if a.reborrow().len() != b.len() {
        return None;
    }

    let (dest_start, dest_end) = a.into_mut_slices();
    let (src_start, src_end) = b.into_slices();

    let src_split = src_start.len();
    let dest_split = dest_start.len();

    let mid_len = dest_split.abs_diff(src_split);

    Some(if src_split >= dest_split {
        let (src_start, src_mid) = src_start.split_at_checked(dest_split).unwrap();
        let (dest_mid, dest_end) = dest_end.split_at_mut_checked(mid_len).unwrap();
        [
            (dest_start, src_start),
            (dest_mid, src_mid),
            (dest_end, src_end),
        ]
    } else {
        let (src_mid, src_end) = src_end.split_at_checked(mid_len).unwrap();
        let (dest_start, dest_mid) = dest_start.split_at_mut_checked(src_split).unwrap();
        [
            (dest_start, src_start),
            (dest_mid, src_mid),
            (dest_end, src_end),
        ]
    })
}

#[allow(dead_code)]
#[inline]
#[must_use]
const fn aligned_portions_mut<'a, 'b, T, U>(
    a: BiSliceMut<'a, T>,
    b: BiSliceMut<'b, U>,
) -> Option<[(&'a mut [T], &'b mut [U]); 3]> {
    if a.reborrow().len() != b.reborrow().len() {
        return None;
    }

    let (dest_start, dest_end) = a.into_mut_slices();
    let (src_start, src_end) = b.into_mut_slices();

    let src_split = src_start.len();
    let dest_split = dest_start.len();

    let mid_len = dest_split.abs_diff(src_split);

    Some(if src_split >= dest_split {
        let (src_start, src_mid) = src_start.split_at_mut_checked(dest_split).unwrap();
        let (dest_mid, dest_end) = dest_end.split_at_mut_checked(mid_len).unwrap();
        [
            (dest_start, src_start),
            (dest_mid, src_mid),
            (dest_end, src_end),
        ]
    } else {
        let (src_mid, src_end) = src_end.split_at_mut_checked(mid_len).unwrap();
        let (dest_start, dest_mid) = dest_start.split_at_mut_checked(src_split).unwrap();
        [
            (dest_start, src_start),
            (dest_mid, src_mid),
            (dest_end, src_end),
        ]
    })
}

#[allow(dead_code)]
#[inline]
#[must_use]
const fn aligned_portions_ref<'a, 'b, T, U>(
    a: BiSlice<'a, T>,
    b: BiSlice<'b, U>,
) -> Option<[(&'a [T], &'b [U]); 3]> {
    if a.len() != b.len() {
        return None;
    }

    let (dest_start, dest_end) = a.into_slices();
    let (src_start, src_end) = b.into_slices();

    let src_split = src_start.len();
    let dest_split = dest_start.len();
    
    let mid_len = dest_split.abs_diff(src_split);

    Some(if src_split >= dest_split {
        let (src_start, src_mid) = src_start.split_at_checked(dest_split).unwrap();
        let (dest_mid, dest_end) = dest_end.split_at_checked(mid_len).unwrap();
        [
            (dest_start, src_start),
            (dest_mid, src_mid),
            (dest_end, src_end),
        ]
    } else {
        let (src_mid, src_end) = src_end.split_at_checked(mid_len).unwrap();
        let (dest_start, dest_mid) = dest_start.split_at_checked(src_split).unwrap();
        [
            (dest_start, src_start),
            (dest_mid, src_mid),
            (dest_end, src_end),
        ]
    })
}
