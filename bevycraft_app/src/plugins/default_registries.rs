use crate::prelude::*;
use bevycraft_core::prelude::*;
use bevycraft_world::prelude::*;
use bevycraft_world::presets::*;
use phf::phf_ordered_map;

pub static BLOCKS: CompiledRegistry<Block> = CompiledRegistry::new(
    phf_ordered_map!(
        "stone" => STONE_BLOCK,
        "cobblestone" => COBBLESTONE_BLOCK,
        "grass" => GRASS_BLOCK,
        "dirt" => DIRT_BLOCK,
        "bedrock" => BEDROCK_BLOCK,
        "oak_log" => OAK_LOG_BLOCK,
        "oak_plank" => OAK_PLANK_BLOCK,
        "oak_leaves" => OAK_LEAVES_BLOCK,
    ),
);
