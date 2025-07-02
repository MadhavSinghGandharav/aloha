use std::net::TcpListener;
use std::sync::{Arc, Mutex,mpsc};
use crate::common::utils::{PORT_ADDRESS,get_ip};
use crate::server::handler::handle_client;
use std::process;
use super::room::{SharedRoom,ChatRoom};


pub fn start() {
    // getting a tcp listner 
    let listener = match TcpListener::bind(PORT_ADDRESS){
        Ok(val) => val,
        Err(e) => {
            eprintln!("Server Error: {}",e);
            process::exit(1);

        }
    };

    match get_ip() {
        Some(ip) => println!("Server Started : {}",ip),
        None => {
            eprintln!("Unable to fetch IP");
            return;
        }
    };


    // creating a shared list of clients and channel
    let shared_room: SharedRoom = Arc::new(Mutex::new(ChatRoom::new()));
    let (tx,rx) = mpsc::channel();

    let shared_room_cloned = Arc::clone(&shared_room);
    let handle = std::thread::spawn(move || {
       ChatRoom::broadcast(shared_room_cloned, rx);
    });
    // Accepting the connection
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // spawing individual thread for every user

                let shared_room_cloned = Arc::clone(&shared_room);
                let tx_cloned = tx.clone();

                std::thread::spawn(move || {
                    if let Err(_) = handle_client(shared_room_cloned, stream, tx_cloned){
                        return;
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    handle.join().unwrap();
}
