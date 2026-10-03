use main_gravity::{scene::parse::prelude::*, sim::prelude::*};
use nannou::prelude::*;
use std::{f32::consts::TAU, fs::File};

fn main() {
    let size = 100.0;
    let scale = 4.0;

    let attractors = (0..7)
        .flat_map(|i| {
            let mut atts = Vec::new();
            let t = 500.0 + 300.0 * i as f32;

            let mass = -100.0 * if i % 2 == 0 { -1.0 } else { 1.0 };

            let mut pos = vec2(-t, 0.0);
            let vel = -scale * pos.normalize();
            pos += vec2(0.0, size);
            let attractor = Attractor::new(pos, vel, mass, 0.0);
            atts.push(attractor);

            let mut pos = vec2(0.0, t);
            let vel = -scale * pos.normalize();
            pos += vec2(size, 0.0);
            let attractor = Attractor::new(pos, vel, mass, 0.0);
            atts.push(attractor);

            let mut pos = vec2(t, 0.0);
            let vel = -scale * pos.normalize();
            pos += vec2(0.0, -size);
            let attractor = Attractor::new(pos, vel, mass, 0.0);
            atts.push(attractor);

            let mut pos = vec2(0.0, -t);
            let vel = -scale * pos.normalize();
            pos += vec2(-size, 0.0);
            let attractor = Attractor::new(pos, vel, mass, 0.0);
            atts.push(attractor);
            atts
        })
        .collect::<Vec<_>>();

    let scene_objects =
        vec![ParseQuad::new(vec![quad::SelfOperation::Square(100.0)], vec![]).into()];

    let parse_setup = ParseSetup::new(8_000_000, scene_objects, attractors);

    let file = File::create("src/scene/scenes/smash.yaml").unwrap();
    yaml_serde::to_writer(file, &parse_setup).unwrap();
}
