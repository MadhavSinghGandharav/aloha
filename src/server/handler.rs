
use super::room::SharedRoom;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc::Sender, Arc};
use std::net::TcpStream;
use crate::common::{utils::read_from_stream, client::Client};
use owo_colors::OwoColorize;


pub fn handle_client(
    shared_room: SharedRoom,
    stream: TcpStream,
    tx: Sender<(Arc<str>, String)>,
    shutdown: Arc<AtomicBool>
) -> Result<(), &'static str> {


    let username:Arc<str> = Arc::from(match read_from_stream(&stream) {
        Ok(Some(msg)) => msg,
        Ok(None) => return Err("Server error: Disconnected early"),
        Err(_) => return Err("Server error: Failed to read header"),
    });
    
    let stream_cloned = match stream.try_clone(){
        Ok(val) => val,
        Err(_) => return Err("Unable to clone stream")
    };
    let client = Client::new(username.to_string(), stream_cloned);

    tx.send((username.clone(), "Joined!".bright_green().bold().to_string())).unwrap();

    {
        let mut clients_guard = shared_room.lock().unwrap();
        clients_guard.add_to_room(client);
    }
    while !shutdown.load(Ordering::Acquire) {
        let msg = match read_from_stream(&stream) {
            Ok(Some(msg)) => msg,
            Ok(None) => {
                shared_room.lock().unwrap().remove_from_room(&username);
                tx.send((username.clone(), "Left!".bright_red().bold().to_string())).unwrap();
                return Err("client left");
            }
            Err(_) => return Err("Server error: Failed to read message"),
        };

        // ← Add this
        if tx.send((username.clone(), msg)).is_err() {
            break;
        }
    }
    // shutting the stream
    stream.shutdown(std::net::Shutdown::Both).unwrap();
    {
        let mut clients_guard = shared_room.lock().unwrap();
        clients_guard.remove_from_room(&username);
    }
    tx.send((username.clone(), "Left!".bright_red().bold().to_string())).ok();

    Ok(())
}

