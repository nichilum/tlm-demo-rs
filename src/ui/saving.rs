use serde::Serialize;

use crate::components::microphone::Microphone;
use crate::components::source::Source;
use crate::components::wall::{CircWall, RectWall};

/// The data that is saved to a file. Used for serialization.
#[derive(Serialize)]
struct SaveData<'a> {
    sources: &'a Vec<&'a Source>,
    mics: &'a Vec<&'a Microphone>,
    rect_walls: &'a Vec<&'a RectWall>,
    circ_walls: &'a Vec<&'a CircWall>,
}

/// Serializes the given data to a byte vector of JSON.
pub fn save(
    sources: &Vec<&Source>,
    mics: &Vec<&Microphone>,
    rect_walls: &Vec<&RectWall>,
    circ_walls: &Vec<&CircWall>,
) -> Result<Vec<u8>, serde_json::Error> {
    let save_data = SaveData {
        sources,
        mics,
        rect_walls,
        circ_walls,
    };

    serde_json::to_vec(&save_data)
}
