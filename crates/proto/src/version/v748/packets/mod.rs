macro_rules! export {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

export!(add_actor);
export!(add_player);
export!(add_item_actor);
export!(award_achievement);
export!(boss_event);
export!(camera_aim_assist);
export!(camera_instruction);
export!(camera_presets);
export!(change_dimension);
export!(client_bound_close_form);
export!(client_bound_debug_renderer);
export!(client_bound_map_item_data);
export!(container_registry_cleanup);
export!(correct_player_move_prediction);
export!(disconnect);
export!(inventory_content);
export!(inventory_slot);
export!(item_stack_request);
export!(item_stack_response);
export!(legacy_telemetry_event);
export!(player_action);
export!(player_auth_input);
export!(resource_pack_stack);
export!(resource_packs_info);
export!(set_actor_data);
export!(start_game);
export!(text);
export!(update_attributes);
export!(update_player_game_type);
export!(update_soft_enum);

export!(code_builder_source);
export!(container_close);
export!(current_structure_feature);
export!(editor_network);
export!(emote);
export!(jigsaw_structure_data);
export!(mob_armor_equipment);
export!(player_armor_damage);
export!(server_bound_diagnostics);
export!(server_bound_loading_screen);
export!(set_title);
export!(stop_sound);
export!(transfer_player);
export!(set_actor_link);
