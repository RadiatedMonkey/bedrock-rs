macro_rules! export {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

export!(camera_aim_assist);
export!(camera_aim_assist_presets);
export!(movement_effect);
export!(player_auth_input);
export!(player_list);
export!(resource_packs_info);
export!(set_movement_authority);
export!(crafting_data);
