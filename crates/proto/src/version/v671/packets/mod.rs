macro_rules! export {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

export!(update_player_game_type);
export!(resource_pack_stack);
