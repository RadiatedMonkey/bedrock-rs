macro_rules! export {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

export!(shaped_chemistry_recipe);
export!(shaped_recipe);
export!(level_settings);
