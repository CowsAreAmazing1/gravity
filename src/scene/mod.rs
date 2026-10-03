pub mod parse;
pub mod scene_layout;
pub mod shapes;

pub mod prelude {
    pub use crate::scene::parse::{object::ParseSetupObject, setup::ParseSetup};
}
