use nannou::prelude::*;

use crate::{prelude::SimState, ui::app::GApp};
pub struct Model {
    pub app_state: GApp,
    pub sim_state: Option<SimState>,
}

pub fn model(app: &App) -> Model {
    app.new_window()
        .size(1000, 1000)
        .view(view)
        .build()
        .unwrap();

    let state = GApp::new();

    Model {
        app_state: state,
        sim_state: None,
    }
}

pub fn event(app: &App, model: &mut Model, event: Event) {
    if let Some(sim_state) = model.sim_state.as_mut() {
        sim_state.ih.custom_event_handler(app, event);
    } else {
        if let Event::WindowEvent {
            simple: Some(event),
            ..
        } = event
            && let WindowEvent::MousePressed(button) = event
            && let Some(sim_state) = model.app_state.on_mouse_click(app, button)
        {
            model.sim_state = Some(sim_state);
        }
    }
}

pub fn update(app: &App, model: &mut Model, _update: Update) {
    if let Some(sim_state) = model.sim_state.as_mut()
        && sim_state.ih.play
    {
        let window = app.main_window();
        let device = window.device();
        let queue = window.queue();

        sim_state
            .system
            .update(sim_state.ih.dt, 5, Some(device), Some(queue));
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if let Some(sim_state) = &model.sim_state {
        let window = app.main_window();
        let device = window.device();
        let queue = window.queue();
        let texture_view = frame.texture_view();

        if let Some(gpu_state) = &sim_state.system.gpu_state {
            let uniform = sim_state.ih.uniform();
            gpu_state.update_uniforms(queue, &uniform);
        }

        let draw = sim_state.ih.draw(&draw);
        sim_state
            .system
            .draw(&draw, device, queue, texture_view, sim_state.ih.scale);
    } else {
        draw.background().color(Rgb::new(0.15, 0.15, 0.15));
        draw.translate(vec3(0.0, -150.0, 0.0))
            .text("System is not initialized")
            .font_size(50);
    }

    draw.to_frame(app, &frame).unwrap();
}
