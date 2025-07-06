use std::net::TcpListener;
use std::sync::{Arc, Mutex,mpsc,atomic::{AtomicBool,Ordering}};
use crate::common::utils::{PORT_ADDRESS,get_ip};
use crate::server::handler::handle_client;
use super::room::{SharedRoom,ChatRoom};
use ctrlc;
use owo_colors::OwoColorize;

pub fn start() -> Result<(),&'static str> {

    // initialzing a shutdown flag we can use mutex here but that would be slow 
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_cloned = Arc::clone(&shutdown);


    // creating a handler
    match ctrlc::set_handler(move || {
        shutdown_cloned.store(true,Ordering::Release);}){

        Ok(_) => (),
        Err(_) => return Err("Unable to sert handler")

    };
       

    // getting a tcp listner and making it non-blocking 
    let listener = match TcpListener::bind(PORT_ADDRESS){
        Ok(val) => val,

        Err(e) => {
            eprintln!("Bind error: {}", e);
            return Err("Unable to bind to port");
        }
    };

    match listener.set_nonblocking(true){
        Ok(_) => (),
        Err(_) => return Err("Unable to set listener to non-blocking")
    }

    match get_ip() {
        Some(ip) => println!("Server Started : {}",ip),
        None => return Err("Unable to fetch IP")
    };


    // creating a shared list of clients and channel
    let shared_room: SharedRoom = Arc::new(Mutex::new(ChatRoom::new()));
    let (tx,rx) = mpsc::channel();

    let shared_room_cloned = Arc::clone(&shared_room);

    // BRODCASTING THREAD

    let brodcast_shutdown = Arc::clone(&shutdown);
    let handle = std::thread::spawn(move ||{
        
        // Safely exits the broadcasting thread
        while !brodcast_shutdown.load(Ordering::Acquire){
            ChatRoom::broadcast(&shared_room_cloned, &rx,&brodcast_shutdown);
        }
    });

    // Accepting the connection
    while !shutdown.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((stream, _addr)) => {
                let client_room = Arc::clone(&shared_room);
                let tx_cloned = tx.clone();
                let shutdown_client = Arc::clone(&shutdown);

                std::thread::spawn(move || {
                    if let Err(e) = handle_client(client_room, stream, tx_cloned,shutdown_client) {
                        eprintln!("Client error: {}", e.bright_red().bold());
                    }
                });
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No incoming connection, sleep a bit
                std::thread::sleep(std::time::Duration::from_millis(100));
                continue;
            }
            Err(e) => {
                eprintln!("Accept failed: {}", e);
            }
        }
    }

        if let Err(e) = handle.join() {
        eprintln!("Broadcast thread join error: {:?}", e);
    }

    Ok(())}
