use nannou::prelude::*;

use crate::{
    prelude::SimState,
    scene::parse::setup::ParseSetup,
    sim::{diff_eq::gpuable::VV, system::System, utils::InteractionHandler},
};

enum GAppState {
    Menu,
    Running,
}

pub struct GApp {
    app_state: GAppState,
}

impl GApp {
    pub fn new() -> Self {
        GApp {
            app_state: GAppState::Menu,
        }
    }

    pub fn on_mouse_click(&mut self, app: &App, button: MouseButton) -> Option<SimState> {
        match self.app_state {
            GAppState::Menu => {
                if button == MouseButton::Left {
                    self.app_state = GAppState::Running;

                    let window = app.main_window();
                    let device = window.device();

                    let mut system = System::new();

                    let file = std::fs::File::open("test.txt").unwrap();
                    let parse_setup: ParseSetup = yaml_serde::from_reader(file).unwrap();
                    parse_setup.build::<VV>(&mut system);

                    system.init_gpu(device);

                    let ih = InteractionHandler::from_rect(&app.window_rect()).set_dt(0.3);

                    let sim_state = SimState { system, ih };

                    Some(sim_state)
                } else {
                    None
                }
            }
            GAppState::Running => None,
        }
    }
}

impl Default for GApp {
    fn default() -> Self {
        Self::new()
    }
}
