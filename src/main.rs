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
        //let player_result = from_str::<Player>(request.to_string().as_str());
        match from_str::<Player>(request.to_string().as_str()) {
            Ok(mut player) => match player.player_status {
                PlayerStatus::Disconnent => {
                    players.remove(&player.id);
                }
                _ => {
                    player.client_ip = format!("{:?}", source);
                    players.insert(player.id.clone(), player);
                }
            },
            Err(e) => {
                println!("Error wile parsiing player: {:?}", e);
            }
        }

        //print request
        //println!("Recived {} from {}", request.trim(), source);
        //print list
        //println!("players: {players:?}");

        broadcast_players(&socket, &players);

        //implement garbage collector
        /*
           if !player.is_active{
               if let None = player.time_since_inactive{
                   player.time_since_inactive = Some(96);
               }
           }
        */
    }
}

fn broadcast_players(socket: &UdpSocket, players: &HashMap<String, Player>) {
    let values: Vec<Player> = players.values().cloned().collect();
    if let Ok(message_to_client) = serde_json::to_string(&values) {
        for (_, player) in players {
            if let Err(e) = socket.send_to(message_to_client.as_bytes(), player.client_ip.clone()) {
                println!("Error while broadcasting: {:?}", e);
            }
        }
    }
}

fn broadcast_players_old(socket: &UdpSocket, players: &HashMap<String, Player>) {
    //make sure not to broadcast to player info about itself and only broadcast players on the same map

    for (receiver_id, receiver_player) in players {
        let mut enemies = vec![];
        for (id, player) in players {
            if receiver_id != id && receiver_player.current_map == player.current_map {
                enemies.push(player);
            }
            if let Ok(message_to_client) = serde_json::to_string(&enemies) {
                if let Err(e) = socket.send_to(
                    message_to_client.as_bytes(),
                    receiver_player.client_ip.clone(),
                ) {
                    println!("Error while broadcasting: {:?}", e);
                }
            }
        }
    }
}
