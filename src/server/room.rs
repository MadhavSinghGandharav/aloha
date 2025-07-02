
use std::collections::HashMap;
use std::sync::{Arc, Mutex,mpsc::Receiver};
use std::io::Write;
use crate::common::client::Client;

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




    pub fn broadcast(shared_room: SharedRoom, rx: Receiver<(Arc<str>, String)>) {
        for (from, message) in rx {
            let formatted = format!("{}: {}", from, message);

            let mut guard = shared_room.lock().unwrap();

            guard.clients.retain(|username, client| {
                if username.as_str() == from.as_ref() {
                    return true; // skip sender
                }

                if let Err(_) = client.stream.write_all(&(formatted.len() as u16).to_be_bytes()) {
                    eprintln!("❌ Failed to send length to {}", username);
                    return false;
                }

                if let Err(_) = client.stream.write_all(formatted.as_bytes()) {
                    eprintln!("❌ Failed to send message to {}, removing.", username);
                    return false;
                }

                true
            });
        }

        println!("🔴 Broadcast loop ended — channel closed");
    }

}

