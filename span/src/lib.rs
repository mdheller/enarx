// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![deny(clippy::all)]
// TODO: https://github.com/enarx/enarx/issues/348
#![deny(missing_docs)]
#![allow(missing_docs)]

use core::ops::*;

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct Span<T, U = T> {
    /// The beginning of the span
    pub start: T,

    /// The number of elments in the span
    pub count: U,
}

impl<T: Clone + Add<U, Output = T>, U: Clone + Sub<U, Output = U>> Span<T, U> {
    pub fn split(self, offset: U) -> (Self, Self) {
        (
            Span {
                start: self.start.clone(),
                count: offset.clone(),
            },
            Span {
                start: self.start + offset.clone(),
                count: self.count - offset,
            },
        )
    }
}

impl<T: Clone + Sub<T, Output = U>, U> From<Range<T>> for Span<T, U> {
    fn from(value: Range<T>) -> Self {
        Span {
            start: value.start.clone(),
            count: value.end - value.start,
        }
    }
}

impl<T: Clone + Add<U, Output = T>, U> From<Span<T, U>> for Range<T> {
    fn from(value: Span<T, U>) -> Self {
        Range {
            start: value.start.clone(),
            end: value.start + value.count,
        }
    }
}
