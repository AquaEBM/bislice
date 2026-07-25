#![no_std]
use core::{fmt, iter, mem, ops};

mod mutable;
mod immutable;

pub use mutable::*;
pub use immutable::*;

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
