use std::fs::File;

use main_gravity::scene::{
    parse::ParseSetup,
    scene_layout::CreationOperation,
    shapes::{
        disc::{self, ParseDisc},
        quad::{self, ParseQuad},
    },
};
use nannou::glam::{Vec2, vec2};

fn main() {
    let mut binding = ParseSetup::new(800_000);
    let ps = binding
        .add(
            ParseDisc::new(
                vec![],
                vec![CreationOperation::CenterOffset(vec2(200.0, 0.0))],
            )
            .into(),
        )
        .add(
            ParseDisc::new(
                vec![],
                vec![CreationOperation::CenterOffset(vec2(100.0, 0.0))],
            )
            .into(),
        )
        .add(
            ParseDisc::new(
                vec![],
                vec![CreationOperation::CenterOffset(vec2(0.0, 0.0))],
            )
            .into(),
        )
        .add(
            ParseDisc::new(
                vec![],
                vec![CreationOperation::CenterOffset(vec2(-100.0, 0.0))],
            )
            .into(),
        )
        .add(
            ParseDisc::new(
                vec![],
                vec![CreationOperation::CenterOffset(vec2(-200.0, 0.0))],
            )
            .into(),
        )
        .add(
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
        )
        .add(
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
        );

    let file = File::create("test.txt").unwrap();
    yaml_serde::to_writer(file, &ps).unwrap();

    // let deserialized_point: Point = yaml_serde::from_str(&yaml)?;
}
