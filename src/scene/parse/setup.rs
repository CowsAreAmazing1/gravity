use serde::{Deserialize, Serialize};

use crate::{
    scene::{
        parse::object::ParseSetupObject,
        scene_layout::Setup,
        shapes::{disc::Disc, quad::Quad},
    },
    sim::{
        diff_eq::AllowedMethod,
        system::{Attractor, System},
    },
};

#[derive(Serialize, Deserialize)]
pub struct ParseSetup {
    dust_count: u32,
    objects: Vec<ParseSetupObject>,
    attractors: Vec<Attractor>,
}

impl ParseSetup {
    pub fn new(
        dust_count: u32,
        objects: Vec<ParseSetupObject>,
        attractors: Vec<Attractor>,
    ) -> Self {
        ParseSetup {
            objects,
            dust_count,
            attractors,
        }
    }
    pub fn build<M: AllowedMethod<M>>(self, system: &mut System<M>) {
        let dust_count = self.dust_count;
        for att in &self.attractors {
            system.add_attractor(*att);
        }
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
