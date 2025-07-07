
use owo_colors::OwoColorize;
use std::io::Write;
use std::sync::{Arc, atomic::{AtomicBool,Ordering}};
use crate::common::utils::{connect_to_stream, prompt};
use crate::common::client::Client;
use crate::client::handler;

pub fn connect(ip: &str) -> Result<(), &'static str> {
    // 1. Establish connection
    let stream = match connect_to_stream(ip) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{}", e);
            return Err("Unable to connect to server");
        }
    };

    // 2. Prepare for shutdown signaling
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_reader = Arc::clone(&shutdown);
    
    // 3. Setup client identity
    let reader = stream.try_clone().unwrap();
    let client_stream = stream.try_clone().unwrap();
    let client_name = prompt("Enter Username: ");
    let mut client = Client::new(client_name, client_stream);

    // 4. Send initial header (username)
    if let Err(_) = client.send_initial_header() {
        return Err("Unable to send initial headers");
    }

    // 5. Spawn receiver thread
    let read_handler = std::thread::spawn(move || {
        if let Err(e) = handler::spawn_receiver(reader, shutdown_reader) {
            eprintln!("{}", e.bright_red().bold());
        }
    });
    
    // Input and Writer thread
    let (tx, rx) = std::sync::mpsc::channel::<String>(); 
       std::thread::spawn(move || {
        let mut msg = String::new();
        loop {
            msg.clear();
            print!("{}", ".> ".red().bold());
            std::io::stdout().flush().unwrap();

            if std::io::stdin().read_line(&mut msg).is_err() {
                continue;
            }

            let trimmed = msg.trim();
            if trimmed == "/exit" {
                // Inform writer to exit
                tx.send(String::from("/exit")).ok();
                break;
            }

            if !trimmed.is_empty() {
                tx.send(trimmed.to_string()).ok();
            }


        }
    });

    // writer 
    handler::spawn_writer(client, shutdown, rx)?;
    read_handler.join().unwrap();
    stream.shutdown(std::net::Shutdown::Both).unwrap();
    Ok(())

}

