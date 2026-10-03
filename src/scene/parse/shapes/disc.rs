use serde::{Deserialize, Serialize};

use crate::scene::{
    scene_layout::{CreationOperation, SetupObject},
    shapes::disc::Disc,
};

#[derive(Serialize, Deserialize)]
pub enum SelfOperation {
    StartAngle(f32),
    EndAngle(f32),
    InnerRadius(f32),
    OuterRadius(f32),
    Radius(f32),
    Ring(f32),
}

#[derive(Serialize, Deserialize)]
pub struct ParseDisc {
    self_ops: Vec<SelfOperation>,
    dust_ops: Vec<CreationOperation>,
}

impl ParseDisc {
    pub fn new(self_ops: Vec<SelfOperation>, dust_ops: Vec<CreationOperation>) -> Self {
        ParseDisc { self_ops, dust_ops }
    }
}

impl From<ParseDisc> for Disc {
    fn from(value: ParseDisc) -> Self {
        let mut disc = Disc::new();

        for op in value.self_ops {
            match op {
                SelfOperation::StartAngle(angle) => {
                    disc = disc.start_angle(angle);
                }
                SelfOperation::EndAngle(angle) => {
                    disc = disc.end_angle(angle);
                }
                SelfOperation::InnerRadius(radius) => {
                    disc = disc.inner_radius(radius);
                }
                SelfOperation::OuterRadius(radius) => {
                    disc = disc.outer_radius(radius);
                }
                SelfOperation::Radius(radius) => {
                    disc = disc.radius(radius);
                }
                SelfOperation::Ring(radius) => {
                    disc = disc.ring(radius);
                }
            }
        }

        for op in value.dust_ops {
            disc = disc.add_operation(op);
        }

        disc
    }
}
