use std::{f32::consts::TAU, fs::File};

use main_gravity::{
    scene::{
        parse::{
            setup::ParseSetup,
            shapes::{
                disc::{self, ParseDisc},
                quad::{self, ParseQuad},
            },
        },
        scene_layout::CreationOperation,
    },
    sim::system::{Attractor, Body},
};
use nannou::{
    glam::{Vec2, vec2},
    math::map_range,
};

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

    let scene_objects = vec![
        ParseDisc::new(
            vec![],
            vec![CreationOperation::CenterOffset(vec2(200.0, 0.0))],
        )
        .into(),
        ParseDisc::new(
            vec![],
            vec![CreationOperation::CenterOffset(vec2(100.0, 0.0))],
        )
        .into(),
        ParseDisc::new(
            vec![],
            vec![CreationOperation::CenterOffset(vec2(0.0, 0.0))],
        )
        .into(),
        ParseDisc::new(
            vec![],
            vec![CreationOperation::CenterOffset(vec2(-100.0, 0.0))],
        )
        .into(),
        ParseDisc::new(
            vec![],
            vec![CreationOperation::CenterOffset(vec2(-200.0, 0.0))],
        )
        .into(),
        ParseDisc::new(
            vec![
                disc::SelfOperation::InnerRadius(10.0),
                disc::SelfOperation::OuterRadius(20.0),
            ],
            vec![
                CreationOperation::CenterOffset(vec2(-250.0, 0.0)),
                CreationOperation::VelocityOffset(vec2(30.0, 0.1)),
                CreationOperation::VelocityScale(0.5),
                CreationOperation::Orbit(vec2(-250.0, 0.0), 300.0, true),
            ],
        )
        .into(),
        ParseQuad::new(
            vec![
                quad::SelfOperation::Width(100.0),
                quad::SelfOperation::Height(5.0),
            ],
            vec![
                CreationOperation::RotateAround(Vec2::ZERO, 1.0),
                CreationOperation::CenterOffset(vec2(-150.0, 0.0)),
                CreationOperation::VelocityOffset(vec2(-30.0, 0.0)),
            ],
        )
        .into(),
    ];

    let parse_setup = ParseSetup::new(800_000, scene_objects, attractors);

    let file = File::create("test.txt").unwrap();
    yaml_serde::to_writer(file, &parse_setup).unwrap();

    // let deserialized_point: Point = yaml_serde::from_str(&yaml)?;
}
