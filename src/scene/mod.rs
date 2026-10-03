pub mod parse;
pub mod scene_layout;
pub mod shapes;

pub const SCENE_DATA_PATH: &str = "src/scene/scenes/";

pub mod prelude {
    pub use crate::scene::{
        parse::{object::ParseSetupObject, setup::ParseSetup},
        scene_layout::CreationOperation,
    };
}
