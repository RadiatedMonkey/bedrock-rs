macro_rules! export {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

export!(base_description);
export!(camera_preset);
export!(user_data_shapeless_recipe);
export!(crafting_data_entry);
export!(item_stack_response_slot_info);
export!(player_block_action_data);
export!(player_input_tick);
export!(player_block_actions);
