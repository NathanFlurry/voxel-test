pub type BlockSides = u8;  // 0b000000 flags for each side
pub type BlockEdges = u32;  // 0b00000000000 flags for each edge
pub type BlockCorners = u8;  // 0b0000000 flags for each corner

lazy_static! {
    static ref BLOCK_CONFIG: Vec<BlockConfig> = {  // TODO: Move this to a TOML file
        let mut bc = Vec::new();

        bc.push(BlockConfig::invisible("air"));
        bc.push(BlockConfig::new("brick_stone", false, BlockConfig::texture_all((0, 0))));
        bc.push(BlockConfig::new("rails_straight_wood", true, BlockConfig::texture_all((1, 0))));
        bc.push(BlockConfig::new("stone_diamond", false, BlockConfig::texture_all((2, 0))));
        bc.push(BlockConfig::new("furnace", false, BlockConfig::texture_sides((2, 5), (3, 0), (2, 5))));
        bc.push(BlockConfig::new("stone_gravel", false, BlockConfig::texture_all((4, 0))));
        bc.push(BlockConfig::new("dirt_grass", false, BlockConfig::texture_sides((4, 6), (5, 0), (5, 1))));
        bc.push(BlockConfig::new("wood_red", false, BlockConfig::texture_all((0, 1))));
        bc.push(BlockConfig::new("rails_curve", true, BlockConfig::texture_all((1, 1))));
        bc.push(BlockConfig::new("stone_coal_alt", false, BlockConfig::texture_all((2, 1))));
        bc.push(BlockConfig::new("mushroom_tan", true, BlockConfig::texture_all((3, 1))));
        bc.push(BlockConfig::new("dirt_gravel", false, BlockConfig::texture_all((4, 1))));
        bc.push(BlockConfig::new("dirt", false, BlockConfig::texture_all((5, 1))));
        bc.push(BlockConfig::new("wood", false, BlockConfig::texture_all((0, 2))));
        bc.push(BlockConfig::new("rails_curve_wood", true, BlockConfig::texture_all((1, 2))));
        bc.push(BlockConfig::new("stone_coal", false, BlockConfig::texture_all((2, 2))));
        bc.push(BlockConfig::new("mushroom_red", true, BlockConfig::texture_all((3, 2))));
        bc.push(BlockConfig::new("grass_large", true, BlockConfig::texture_all((4, 2))));
//        bc.push(BlockConfig::new("cotton_tan", true, BlockConfig::texture_all((4, 2))));

        bc
    };
}

type BlockTexturePosition = (usize, usize);

struct BlockConfig {
    /// The name of the block.
    name: &'static str,

    /// If the block is completely invisible and has no visible voxel properties to the player.
    /// This is useful for things like barrier block which are completely invisible, but still
    /// has a particle effect visible only to admins.
    is_invisible: bool,

    /// If the block can be seen through. For example, glass, stairs, and chests still has solid
    /// aspects but blocks can be seen on the other side. This should be true if `is_invisible`
    /// is true.
    is_transparent: bool,

    /// The index of the texture to use on each side of the block. The indexes correspond to
    /// `Chunk::SIDE_DIRS`.
    textures: [BlockTexturePosition; 6],
}

impl BlockConfig {
    pub fn new(name: &'static str, is_transparent: bool, textures: [BlockTexturePosition; 6]) -> BlockConfig {
        BlockConfig {
            name,
            is_invisible: false,
            is_transparent,
            textures
        }
    }

    pub fn invisible(name: &'static str) -> BlockConfig {
        BlockConfig {
            name,
            is_invisible: true,
            is_transparent: true,
            textures: BlockConfig::texture_all((0, 0))
        }
    }

    pub fn texture_all(position: BlockTexturePosition) -> [BlockTexturePosition; 6] {
        [position, position, position, position, position, position]
    }

    pub fn texture_sides(top: BlockTexturePosition, side: BlockTexturePosition, bottom: BlockTexturePosition) -> [BlockTexturePosition; 6] {
        [side, side, side, side, top, bottom]
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Block(u8);

impl Block {
    pub const AIR: Block = Block(0);

    pub fn from_id(id: &str) -> Block {
        for (i, block) in BLOCK_CONFIG.iter().enumerate() {
            if block.name == id {
                return Block(i as u8);
            }
        }

        panic!("Unknown block ID {}", id);
    }
}

impl Block {
    fn get_config(&self) -> &BlockConfig { &BLOCK_CONFIG[self.0 as usize] }

    pub fn is_transparent(&self) -> bool { self.get_config().is_transparent }

    pub fn is_invisible(&self) -> bool { self.get_config().is_invisible }

    pub fn texture_pos(&self, side: usize) -> BlockTexturePosition {
        self.get_config().textures[side]
    }
}
