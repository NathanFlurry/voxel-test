use std::time::Duration;
use std::ops;

// Polyfill for https://github.com/rust-lang/rust/issues/54361
const NANOS_PER_SEC: u32 = 1_000_000_000;

pub trait AsFloatSeconds {
    fn as_float_seconds(&self) -> f64;
}

impl AsFloatSeconds for Duration {
    fn as_float_seconds(&self) -> f64 {
        (self.as_secs() as f64) + (self.subsec_nanos() as f64) / (NANOS_PER_SEC as f64)
    }
}

// Polyfill for https://github.com/rust-lang/rust/issues/32311
pub trait RangeContains<Idx> {
    fn range_contains<U>(&self, item: &U) -> bool where Idx: PartialOrd<U>, U: ?Sized + PartialOrd<Idx>;
}

impl<Idx> RangeContains<Idx> for ops::RangeInclusive<Idx> {
    fn range_contains<U>(&self, item: &U) -> bool where Idx: PartialOrd<U>, U: ?Sized + PartialOrd<Idx> {
        !(item < self.start() || item > self.end())
    }
}
