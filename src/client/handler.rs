
use std::{io::Write, net::TcpStream};
use std::thread;
use crate::common::{utils::read_from_stream,client::Client};
use std::io;

// function to receive messages from server
pub fn spawn_receiver(reader: TcpStream) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            match read_from_stream(&reader) {
                Ok(Some(msg)) => {
                    print!("\r\x1b[2K"); // clear line
                    println!("{msg}");
                    print!(".> ");
                    io::stdout().flush().unwrap();},                
                Ok(None) => {
                        eprintln!("\n🔌 Server closed the connection.");
                        break;
                }
                Err(e) => {
                    eprintln!("\n❌ Error reading from server: {e}");
                    break;
                }
            }
        }
    })
}
// function to write messages to the server 

pub fn spawn_writer(mut client: Client){

    let mut msg = String::new();
    loop {
        msg.clear();
        print!(".> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut msg).expect("");

        if msg.trim() == "/exit" {
            break;
        }
        if let Err(e) = client.write_to_stream(msg.trim()) {
            eprintln!("Error sending message: {}", e);
            break;
        }
       
    }
}

