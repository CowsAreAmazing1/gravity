use main_gravity::prelude::*;
use nannou::prelude::*;

struct Model {
    system: System<VV>,
    ih: InteractionHandler,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1000, 1000)
        .view(view)
        .build()
        .unwrap();
    let window = app.main_window();
    let device = window.device();

    let mut system = System::new();

    let size = 100.0;
    let scale = 4.0;

    for i in 0..7 {
        let t = 500.0 + 300.0 * i as f32;

        let mass = -100.0 * if i % 2 == 0 { -1.0 } else { 1.0 };

        let mut pos = vec2(-t, 0.0);
        let vel = -scale * pos.normalize();
        pos += vec2(0.0, size);
        let attractor = Attractor::new(pos, vel, mass, 0.0);
        system.add_attractor(attractor);

        let mut pos = vec2(0.0, t);
        let vel = -scale * pos.normalize();
        pos += vec2(size, 0.0);
        let attractor = Attractor::new(pos, vel, mass, 0.0);
        system.add_attractor(attractor);

        let mut pos = vec2(t, 0.0);
        let vel = -scale * pos.normalize();
        pos += vec2(0.0, -size);
        let attractor = Attractor::new(pos, vel, mass, 0.0);
        system.add_attractor(attractor);

        let mut pos = vec2(0.0, -t);
        let vel = -scale * pos.normalize();
        pos += vec2(-size, 0.0);
        let attractor = Attractor::new(pos, vel, mass, 0.0);
        system.add_attractor(attractor);
    }

    // let center = Attractor::new(Vec2::ZERO, Vec2::ZERO, 1000.0, 200.0);

    let mut setup = Setup::new();
    setup.add(Quad::new().square(size));

    // system.add_attractor(center);

    system.include_setup_random(&setup, 8_000_000);
    system.init_gpu(device);

    let ih = InteractionHandler::from_rect(&app.window_rect());

    Model { system, ih }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if model.ih.play {
        let window = app.main_window();
        let device = window.device();
        let queue = window.queue();

        model.system.update(
            model.ih.dt,
            10,
            Some(device),
            Some(queue),
            Some(window.rect()),
        );
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let window = app.main_window();
    let device = window.device();
    let queue = window.queue();
    let texture_view = frame.texture_view();

    if let Some(gpu_state) = &model.system.gpu_state {
        let uniform = model.ih.uniform();
        gpu_state.update_uniforms(queue, &uniform);
    }

    let draw = model.ih.draw(app.draw());
    model
        .system
        .draw(&draw, device, queue, texture_view, model.ih.scale);
    draw.to_frame(app, &frame).unwrap();
}

fn event(app: &App, model: &mut Model, event: Event) {
    model.ih.custom_event_handler(app, event);
}

fn main() {
    nannou::app(model).update(update).event(event).run();
}
