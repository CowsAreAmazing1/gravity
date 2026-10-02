pub mod diff_eq;
pub mod gpu;
pub mod system;
pub mod utils;

pub mod prelude {
    pub use crate::{
        sim::diff_eq::{
            AllowedMethod,
            gpuable::VV,
            not_gpuable::{DOP853, EULER, RK4, SSPRK3},
        },
        sim::system::{Attractor, Body, Dust, System, sun_planet_binary_ccw},
        sim::utils::InteractionHandler,
    };

    pub struct SimState {
        pub system: System<VV>,
        pub ih: InteractionHandler,
    }

    // pub fn model(app: &App) -> Model {
    //     app.new_window()
    //         .size(1000, 1000)
    //         .view(view)
    //         .build()
    //         .unwrap();
    //     let window = app.main_window();
    //     let device = window.device();

    //     let mut system = System::new();

    //     // let num = 8;
    //     // let scale = 3.5;
    //     // for i in 0..num {
    //     //     let t = map_range(i, 0, num, 0.0, TAU);

    //     //     let pos: Vec2 = 250.0 * Vec2::from(t.sin_cos());
    //     //     let vel = -scale * pos.normalize();
    //     //     let mut attractor = Attractor::new(pos, vel, -10.0, 0.0);
    //     //     attractor.set_orbit(Vec2::ZERO, 10.0, true);
    //     //     *attractor.velocity_mut() = attractor.velocity() + vel;
    //     //     system.add_attractor(attractor);
    //     // }

    //     // let mut setup = Setup::new();
    //     // setup.add(Disc::new().radius(100.0));

    //     // system.include_setup_random(&setup, 20_000_000);
    //     // system.init_gpu(device);

    //     let ih = InteractionHandler::from_rect(&app.window_rect()).set_dt(0.3);

    //     Model { system, ih }
    // }
}
