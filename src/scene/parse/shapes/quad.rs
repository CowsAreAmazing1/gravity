use serde::{Deserialize, Serialize};

use crate::scene::{
    scene_layout::{CreationOperation, SetupObject},
    shapes::quad::Quad,
};

#[derive(Serialize, Deserialize)]
pub enum SelfOperation {
    Square(f32),
    Width(f32),
    Height(f32),
}

#[derive(Serialize, Deserialize)]
pub struct ParseQuad {
    self_ops: Vec<SelfOperation>,
    dust_ops: Vec<CreationOperation>,
}

impl ParseQuad {
    pub fn new(self_ops: Vec<SelfOperation>, dust_ops: Vec<CreationOperation>) -> Self {
        ParseQuad { self_ops, dust_ops }
    }
}

impl From<ParseQuad> for Quad {
    fn from(value: ParseQuad) -> Self {
        let mut quad = Quad::new();

        for op in value.self_ops {
            match op {
                SelfOperation::Square(size) => {
                    quad = quad.square(size);
                }
                SelfOperation::Width(width) => {
                    quad = quad.width(width);
                }
                SelfOperation::Height(height) => {
                    quad = quad.height(height);
                }
            }
        }

        for op in value.dust_ops {
            quad = quad.add_operation(op);
        }

        quad
    }
}
