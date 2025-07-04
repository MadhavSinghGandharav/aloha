
use std::collections::HashMap;
use std::sync::{Arc, Mutex,mpsc::{Receiver,RecvTimeoutError}};
use std::io::Write;
use crate::common::client::Client;
use std::time::Duration;
use std::sync::atomic::{Ordering,AtomicBool};

pub type SharedRoom = Arc<Mutex<ChatRoom>>;

pub struct ChatRoom {
    // Maps usernames to client sockets
    clients: HashMap<String, Client>,
}

impl ChatRoom {
    pub fn new() -> Self {
        ChatRoom {
            clients: HashMap::new(),
        }
    }

    pub fn add_to_room(&mut self, client: Client) {
        self.clients.insert(client.username.to_string(),client);
    }

    pub fn remove_from_room(&mut self, username: &str) {
        self.clients.remove(username);
    }


    pub fn broadcast(shared_room: &SharedRoom, rx: &Receiver<(Arc<str>, String)>, shutdown: &AtomicBool) {
        loop {
            if shutdown.load(Ordering::Acquire) {
                break;
            }

            match rx.recv_timeout(Duration::from_millis(500)) {
                Ok((from, message)) => {
                    let formatted = format!("{}: {}", from, message);

                    let mut guard = shared_room.lock().unwrap();

                    guard.clients.retain(|username, client| {
                        if username.as_str() == from.as_ref() {
                            return true;
                        }

                        if let Err(_) = client.stream.write_all(&(formatted.len() as u16).to_be_bytes()) {
                            eprintln!("Failed to send length to {}", username);
                            return false;
                        }

                        if let Err(_) = client.stream.write_all(formatted.as_bytes()) {
                            eprintln!("Failed to send message to {}, removing.", username);
                            return false;
                        }

                        true
                    });
                }
                Err(RecvTimeoutError::Timeout) => continue, // waking up 
                Err(RecvTimeoutError::Disconnected) => {
                    println!("Channel disconnected, exiting broadcast loop.");
                    break;
                }
            }
        }
 
    }

}

