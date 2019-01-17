use std::time::Duration;
use std::ops;
use crate::world;

/// Polyfill for https://github.com/rust-lang/rust/issues/54361
const NANOS_PER_SEC: u32 = 1_000_000_000;

pub trait AsFloatSeconds {
    fn as_float_seconds(&self) -> f64;
}

impl AsFloatSeconds for Duration {
    fn as_float_seconds(&self) -> f64 {
        (self.as_secs() as f64) + (self.subsec_nanos() as f64) / (NANOS_PER_SEC as f64)
    }
}

/// Polyfill for https://github.com/rust-lang/rust/issues/32311
pub trait RangeContains<Idx> {
    fn range_contains<U>(&self, item: &U) -> bool where Idx: PartialOrd<U>, U: ?Sized + PartialOrd<Idx>;
}

impl<Idx> RangeContains<Idx> for ops::RangeInclusive<Idx> {
    fn range_contains<U>(&self, item: &U) -> bool where Idx: PartialOrd<U>, U: ?Sized + PartialOrd<Idx> {
        !(item < self.start() || item > self.end())
    }
}

/// Quickly clamps values to the chunk.
pub trait ChunkClamp {
    fn chunk_clamp_x(self) -> Self;
    fn chunk_clamp_y(self) -> Self;
    fn chunk_clamp_z(self) -> Self;
}

impl ChunkClamp for f32 {
    fn chunk_clamp_x(self) -> Self {
        self.min(world::Chunk::SIZE_X_F32).max(0.)
    }

    fn chunk_clamp_y(self) -> Self {
        self.min(world::Chunk::SIZE_Y_F32).max(0.)
    }

    fn chunk_clamp_z(self) -> Self {
        self.min(world::Chunk::SIZE_Z_F32).max(0.)
    }
}

impl ChunkClamp for u32 {
    fn chunk_clamp_x(self) -> Self {
        self.min(world::Chunk::SIZE_X_U32)
    }

    fn chunk_clamp_y(self) -> Self {
        self.min(world::Chunk::SIZE_Y_U32)
    }

    fn chunk_clamp_z(self) -> Self {
        self.min(world::Chunk::SIZE_Z_U32)
    }
}

impl ChunkClamp for usize {
    fn chunk_clamp_x(self) -> Self {
        self.min(world::Chunk::SIZE_X)
    }

    fn chunk_clamp_y(self) -> Self {
        self.min(world::Chunk::SIZE_Y)
    }

    fn chunk_clamp_z(self) -> Self {
        self.min(world::Chunk::SIZE_Z)
    }
}
