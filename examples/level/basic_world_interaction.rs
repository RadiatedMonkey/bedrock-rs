use bedrockrs_level::level::level::default_impl::*;
use bedrockrs_level::level::level::LevelConfiguration;
use bedrockrs_shared::world::dimension::Dimension;

fn main() -> Result<(), BedrockLevelError<RawInterface, BedrockSubChunkDecoder, BedrockSubChunk>> {
    // Open a world
    let level = BedrockLevel::open(
        "./crates/level/test_level".into(),
        LevelConfiguration {
            sub_chunk_range: (-4, 20).into(), // This is the range of subchunks to load. This is the default and handles the vanilla world
            rw_cache: false, // This enables an experimental RW caching option. This can be really fast if your doing a lot of reads from the same location. Default is false since in most cases its slower
            create_db_if_missing: false, // This will create the underling DB if it is not in the folder provided
        },
        BedrockState {}, // This is the state which is used for the lifetime of the Level. This is only required to be something when using custom implementations of some features
    )?;

    // Fetch a chunk or construct it if it's not in the world
    let mut chunk = level.get_chunk(
        (0, 0).into(), // This is the location (in chunk space)
        Dimension::Overworld,
    );
}
