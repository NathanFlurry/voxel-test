pub type BlockSides = u8;  // 0b000000 flags for each side
pub type BlockEdges = u32;  // 0b00000000000 flags for each edge
pub type BlockCorners = u8;  // 0b0000000 flags for each corner

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Block(u8);

impl Block {
    pub const AIR: Block = Block(0);
    pub const DIRT: Block = Block(1);
    pub const STONE: Block = Block(2);
    pub const GRAVEL: Block = Block(3);
    pub const STONE_BRICK: Block = Block(4);
    pub const BRICK: Block = Block(5);

    pub fn is_air(&self) -> bool { *self == Block::AIR }

    pub fn is_transparent(&self) -> bool { self.is_air() }

    pub fn is_invisible(&self) -> bool { self.is_air() }

    pub fn texture_pos(&self) -> (u8, u8) {
        match *self {
            Block::DIRT => (5, 1),
            Block::STONE => (2, 5),
            Block::GRAVEL => (4, 0),
            Block::STONE_BRICK => (0, 0),
            Block::BRICK => (2, 12),
            _ => (0, 0)  // TODO: Add unknown block
        }
    }
}
