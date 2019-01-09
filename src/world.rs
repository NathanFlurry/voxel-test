use crate::chunk::Chunk;
use std::collections::HashMap;

pub struct ChunkIndex(u8, u8);

pub struct World {
    chunks: HashMap<ChunkIndex, Chunk>
}
