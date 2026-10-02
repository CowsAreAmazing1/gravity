pub mod sim;
pub mod ui;

// pub mod temp_app {
//     pub use crate::ui::*;
// }

pub mod prelude {
    pub use crate::sim::prelude::*;
    pub use crate::ui::prelude::*;
}
