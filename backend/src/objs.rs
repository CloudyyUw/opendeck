use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeckGrid {
    pub height: i32,
    pub width: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeckButton {
    pub color: String,
    pub keys: Vec<String>,
    pub name: String,
    pub pos: i32,
    pub icon: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeckTab {
    pub grid: DeckGrid,
    pub bg: String,
    pub name: String,
    pub buttons: Vec<DeckButton>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keys {
    pub keys: Vec<String>
}