use serde::{Deserialize, Serialize};

use crate::scene::parse::shapes::{disc::ParseDisc, quad::ParseQuad};

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
