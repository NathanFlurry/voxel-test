pub struct BlockType(u8);

impl BlockType {
    pub fn is_air(&self) -> bool { self.0 == 0 }

    pub fn is_transparent(&self) -> bool { self.is_air() }
}

pub struct Block {
    block_type: BlockType
}
