use bedrockrs_level::level::level::default_impl::*;
use bedrockrs_level::level::level::LevelConfiguration;

fn main() -> Result<(), BedrockLevelError<RawInterface, BedrockSubChunkDecoder, BedrockSubChunk>>{
    // Open a world
    let level = BedrockLevel::open("./crates/level/test_level".into(), LevelConfiguration {
        sub_chunk_range: (-4, 20).into(),


    })

}