use serde::{Deserialize, Serialize};

use crate::{
    scene::{
        scene_layout::Setup,
        shapes::{
            disc::{Disc, ParseDisc},
            quad::{ParseQuad, Quad},
        },
    },
    sim::{diff_eq::AllowedMethod, system::System},
};

#[derive(Serialize, Deserialize)]
pub enum ParseSetupObject {
    Disc(ParseDisc),
    Quad(ParseQuad),
}

impl From<ParseDisc> for ParseSetupObject {
    fn from(value: ParseDisc) -> Self {
        ParseSetupObject::Disc(value)
    }
}

impl From<ParseQuad> for ParseSetupObject {
    fn from(value: ParseQuad) -> Self {
        ParseSetupObject::Quad(value)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ParseSetup {
    dust_count: u32,
    objects: Vec<ParseSetupObject>,
}

impl ParseSetup {
    pub fn new(dust_count: u32) -> Self {
        ParseSetup {
            objects: Vec::new(),
            dust_count,
        }
    }
    pub fn add(&mut self, object: ParseSetupObject) -> &mut Self {
        self.objects.push(object);
        self
    }
}

impl ParseSetup {
    pub fn build<M: AllowedMethod<M>>(self, system: &mut System<M>) {
        let dust_count = self.dust_count;
        let setup = Setup::from(self);
        system.include_setup_random(&setup, dust_count);
    }
}

impl From<ParseSetup> for Setup {
    fn from(value: ParseSetup) -> Self {
        let mut setup = Setup::new();
        for obj in value.objects {
            match obj {
                ParseSetupObject::Disc(parse_disc) => setup.add(Disc::from(parse_disc)),
                ParseSetupObject::Quad(parse_quad) => setup.add(Quad::from(parse_quad)),
            };
        }
        setup
    }
}
