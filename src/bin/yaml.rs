use main_gravity::{scene::parse::prelude::*, sim::prelude::*};
use nannou::prelude::*;
use std::{f32::consts::TAU, fs::File};

fn main() {
    let num = 8;
    let scale = 3.5;
    let attractors = (0..num)
        .map(|i| {
            let t = map_range(i, 0, num, 0.0, TAU);

            let pos: Vec2 = 250.0 * Vec2::from(t.sin_cos());
            let vel = -scale * pos.normalize();
            let mut attractor = Attractor::new(pos, vel, -10.0, 0.0);
            attractor.set_orbit(Vec2::ZERO, 10.0, true);
            *attractor.velocity_mut() = attractor.velocity() + vel;
            attractor
        })
        .collect::<Vec<_>>();

    let scene_objects =
        vec![ParseDisc::new(vec![disc::SelfOperation::Radius(100.0)], vec![]).into()];

    let parse_setup = ParseSetup::new(800_000, scene_objects, attractors);

    let file = File::create("src/scene/scenes/crush.txt").unwrap();
    yaml_serde::to_writer(file, &parse_setup).unwrap();

    // let deserialized_point: Point = yaml_serde::from_str(&yaml)?;
}
