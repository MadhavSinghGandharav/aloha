
use super::room::SharedRoom;
use std::sync::{mpsc::Sender, Arc};
use std::net::TcpStream;
use crate::common::{utils::read_from_stream, client::Client};

pub fn handle_client(
    shared_room: SharedRoom,
    stream: TcpStream,
    tx: Sender<(Arc<str>, String)>
) -> Result<(), &'static str> {


    let username:Arc<str> = Arc::from(match read_from_stream(&stream) {
        Ok(Some(msg)) => msg,
        Ok(None) => return Err("Server error: Disconnected early"),
        Err(_) => return Err("Server error: Failed to read header"),
    });
    
    let stream_cloned = stream.try_clone().unwrap();
    let client = Client::new(username.to_string(), stream_cloned);

    tx.send((username.clone(), "Joined!".to_string())).unwrap();

    {
        let mut clients_guard = shared_room.lock().unwrap();
        clients_guard.add_to_room(client);
    }
         loop {
        let msg = match read_from_stream(&stream) {
            Ok(Some(msg)) => msg,
            Ok(None) => {
                shared_room.lock().unwrap().remove_from_room(&username);
                tx.send((username.clone(), "Left".to_string())).unwrap();
                return Err("client left");
            }
            Err(_) => return Err("Server error: Failed to read message"),
        };

      // ← Add this
        if tx.send((username.clone(), msg)).is_err() {
            break;
        }
    }

    Ok(())
}

