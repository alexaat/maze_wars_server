use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PlayerStatus {
    Active,
    Disconnent,
    Killed,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub position: Position,
    pub score: u32,
    pub player_status: PlayerStatus,
    pub orientation: f32,
    pub current_map: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub client_ip: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerMessage{
    pub sender_id: String,
    pub player: Player
}
