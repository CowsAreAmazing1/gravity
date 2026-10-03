use std::path::Path;

use nannou::prelude::*;

use crate::{
    prelude::SimState,
    scene::{SCENE_DATA_PATH, parse::setup::ParseSetup},
    sim::{diff_eq::gpuable::VV, system::System, utils::InteractionHandler},
};

enum GAppState {
    Menu,
    Running,
}

pub struct GApp {
    app_state: GAppState,

    scene_idx: usize,
    scenes: Vec<String>,
}

impl GApp {
    pub fn new() -> Self {
        GApp {
            app_state: GAppState::Menu,

            scene_idx: 0,
            scenes: std::fs::read_dir(SCENE_DATA_PATH)
                .unwrap()
                .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
                .collect(),
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

                    let file = std::fs::File::open(
                        Path::new(SCENE_DATA_PATH).join(self.scenes[self.scene_idx].as_str()),
                    )
                    .unwrap();
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

    pub fn on_key_press(&mut self, _app: &App, key: Key) {
        match key {
            Key::Left => {
                self.scene_idx = (self.scene_idx - 1) % self.scenes.len();
            }
            Key::Right => {
                self.scene_idx = (self.scene_idx + 1) % self.scenes.len();
            }
            _ => {}
        }
    }

    pub fn view(&self, draw: &Draw) {
        draw.background().color(Rgb::new(0.15, 0.15, 0.15));
        draw.translate(vec3(0.0, 250.0, 0.0))
            .text("Pick a scene")
            .font_size(50)
            .no_line_wrap();

        draw.text(self.scenes[self.scene_idx].as_str())
            .font_size(50)
            .no_line_wrap();

        draw.translate(vec3(0.0, -100.0, 0.0))
            .text("Press left/right to change scene")
            .font_size(20);

        draw.translate(vec3(0.0, -250.0, 0.0))
            .text("Click to start the simulation")
            .font_size(25);

        draw.translate(vec3(-350.0, 300.0, 0.0))
            .text("Space to play/pause")
            .font_size(20)
            .no_line_wrap();
        draw.translate(vec3(-350.0, 260.0, 0.0))
            .text("Mouse drag to move the camera")
            .font_size(20)
            .no_line_wrap();
        draw.translate(vec3(-350.0, 220.0, 0.0))
            .text("Scroll to zoom in/out")
            .font_size(20)
            .no_line_wrap();
        draw.translate(vec3(-350.0, 180.0, 0.0))
            .text("Shift + scroll to slow down/speed up time")
            .font_size(20)
            .no_line_wrap();
        draw.translate(vec3(-350.0, 140.0, 0.0))
            .text("T to reverse time")
            .font_size(20)
            .no_line_wrap();
    }
}

impl Default for GApp {
    fn default() -> Self {
        Self::new()
    }
}
