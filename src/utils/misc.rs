use std::time::Duration;

// Polyfill for https://github.com/rust-lang/rust/issues/54361
const NANOS_PER_SEC: u32 = 1_000_000_000;

pub trait AsFloatSecs {
    fn as_float_secs(&self) -> f64;
}

impl AsFloatSecs for Duration {
    fn as_float_secs(&self) -> f64 {
        (self.as_secs() as f64) + (self.subsec_nanos() as f64) / (NANOS_PER_SEC as f64)
    }
}
