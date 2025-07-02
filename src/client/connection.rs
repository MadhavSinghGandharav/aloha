use crate::common::utils::{connect_to_stream,prompt};
use crate::common::client::Client;
use crate::client::handler;


pub fn connect(ip: &str) { 

        let stream = match connect_to_stream(ip) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    // creating a client     
    let reader = stream.try_clone().unwrap();
    let client_name = prompt("Enter Username: ");
    let mut client = Client::new(client_name, stream);

    // 3. Send initial header
    if let Err(_) = client.send_initial_header() {
        eprintln!("Unable to send initial header");
        return;
    }

    // handler for reading messages from server
    let read_handler = handler::spawn_receiver(reader);
    
    // handler to write messages to server
    handler::spawn_writer(client);
    
    read_handler.join().unwrap();
}
    


