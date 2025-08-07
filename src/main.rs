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
    let mut playes = HashMap::new();

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

