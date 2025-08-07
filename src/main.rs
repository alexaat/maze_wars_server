use std::net::UdpSocket;
mod utils;
use std::collections::HashMap;
use serde_json::from_str;
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
        let player_result = from_str::<Player>(request.to_string().as_str());
        match player_result {
            Ok(mut player) => {
                player.client_ip = format!("{:?}",source);
                if player.is_active {
                    players.insert(player.id.clone(), player);
                }else{
                    players.remove(player.id.as_str());
                }
            },
            Err(e) => {
                println!("Error wile parsiing player: {:?}",e);
            }
        }

        //print request
        //println!("Recived {} from {}", request.trim(), source);
        //print list
        //println!("players: {players:?}");


        broadcast_players(&socket, &players);
    }
}

fn broadcast_players(socket: &UdpSocket, players: &HashMap<String, Player>){   
    for (_, v) in players{
        let client_message = format!("Server Response to {}", v.id);
        socket.send_to(client_message.as_bytes(), v.client_ip.clone()).unwrap();
    }  
}

