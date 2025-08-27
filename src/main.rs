use std::net::UdpSocket;
mod utils;
use serde_json::from_str;
use std::collections::HashMap;
mod models;
use models::*;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:4000").unwrap();
    println!("Server listening on {}", socket.local_addr().unwrap());
    let mut buffer = [0; 1024];

    //players
    let mut players = HashMap::new();

    loop {
        //read client request
        let (size, source) = socket.recv_from(&mut buffer).unwrap();
        let request = String::from_utf8_lossy(&buffer[..size]);
        //parse request body
        match from_str::<ServerMessage>(request.to_string().as_str()) {
            Ok(mut server_message) => {
                match server_message.player.player_status {
                    PlayerStatus::Disconnent => {
                        players.remove(&server_message.player.id);
                    },
                    PlayerStatus::Killed => {
                        if server_message.player.id == server_message.sender_id{
                           server_message.player.client_ip = format!("{:?}", source);
                           players.insert(server_message.player.id.clone(), server_message.player);
                        } else {
                            //message sent on behalf of other user
                            //find ip of owner
                            let owner = players.get(&server_message.player.id);
                            if let Some(player) = owner{
                               server_message.player.client_ip = player.client_ip.clone();
                               players.insert(server_message.player.id.clone(), server_message.player);
                            }
                        }
                    },
                    _ =>{
                        server_message.player.client_ip = format!("{:?}", source);
                        players.insert(server_message.player.id.clone(), server_message.player);
                    } 
                }
            },

            Err(e) => {
                println!("Error wile parsiing player: {:?}", e);
            }
        }
        broadcast_players(&socket, &players);
    }
}


fn broadcast_players(socket: &UdpSocket, players: &HashMap<String, Player>) {

    for (_, player) in players {
        //get values
        let values: Vec<Player> = players.values().filter(|_player| _player.current_map == player.current_map).cloned().collect();
        if let Ok(message_to_client) = serde_json::to_string(&values) {
            if let Err(e) = socket.send_to(message_to_client.as_bytes(), player.client_ip.clone()) {
                println!("Error while broadcasting: {:?}", e);
            }
        }
    }
}
