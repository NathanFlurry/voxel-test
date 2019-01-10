#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Block(u8);

impl Block {
    pub const AIR: Block = Block(0);
    pub const DIRT: Block = Block(1);

    pub fn is_air(&self) -> bool { *self == Block::AIR }

    pub fn is_transparent(&self) -> bool { self.is_air() }
}
