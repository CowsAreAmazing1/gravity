pub mod object;
pub mod setup;
pub mod shapes;

pub mod prelude {
    pub use super::{
        setup::ParseSetup,
        shapes::{
            disc::{self, ParseDisc},
            quad::{self, ParseQuad},
        },
    };
}
