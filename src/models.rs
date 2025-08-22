use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub position: Position,
    pub score: u32,
    pub is_active: bool,
    pub orientation: f32,
    pub current_map: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub client_ip: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub time_since_inactive: Option<u64>,
}
impl Player {
    pub fn new() -> Self {
        Player {
            id: String::from(""),
            name: String::from(""),
            position: Position::new(),
            score: 0,
            is_active: true,
            orientation: 0.0,
            current_map: String::from(""),
            client_ip: String::from(""),
            time_since_inactive: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub z: f32,
}
impl Position {
    pub fn new() -> Self {
        Position { x: 0.0, z: 0.0 }
    }
}
