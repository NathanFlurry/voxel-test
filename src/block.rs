#[derive(Copy, Clone)]
pub struct Block(u8);

impl Block {
    pub const AIR: Block = Block(0);

    pub fn is_air(&self) -> bool { self.0 == 0 }

    pub fn is_transparent(&self) -> bool { self.is_air() }
}
