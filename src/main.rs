use std::net::UdpSocket;
mod utils;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
fn main() {
    let socket = UdpSocket::bind("0.0.0.0:4000").unwrap();
    println!("Server listening on {}", socket.local_addr().unwrap());
    let mut buffer = [0; 1024];

    //players
    let mut playes = HashMap::new();

    loop {
        //read client request
        let (size, source) = socket.recv_from(&mut buffer).unwrap();
        let request = String::from_utf8_lossy(&buffer[..size]);
        //parse request body
        let player_result = from_str::<Player>(request.to_string().as_str());
        match player_result {
            Ok(player) => {
                if player.is_active {
                    playes.insert(player.id.clone(), player);
                }else{
                    playes.remove(player.id.as_str());
                }
            },
            Err(e) => {
                println!("Error wile parsiing player: {:?}",e);
            }
        }

        //print request
        println!("Recived {} from {}", request.trim(), source);
        //print list
        println!("playes: {playes:?}");

        let response = format!("Server Response: {}", request.trim());
        socket.send_to(response.as_bytes(), source).unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Player {
    pub id: String,
    pub name: String,
    pub position: Position,
    pub score: u32,
    pub is_active: bool
}
impl Player {
    pub fn new(id: String, name: String) -> Player {
        Player {
            id,
            name,
            position: Position::new(),
            score: 0,
            is_active: true
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Position{
   pub x: f32,
   pub y: f32
}
impl Position {
   pub fn new() -> Self{
      Position { x: 0.0, y: 0.0 }
   }
}