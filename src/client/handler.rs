

use std::{io::{self, Write}, net::TcpStream, sync::{atomic::{AtomicBool, Ordering}, Arc}};
use crate::common::{utils::read_from_stream, client::Client};
use owo_colors::OwoColorize;

/// Function to receive messages from server

pub fn spawn_receiver(reader: TcpStream, shutdown: Arc<AtomicBool>) -> Result<(), &'static str> {
    loop {
        if shutdown.load(Ordering::Acquire) {
            break;
        }

        match read_from_stream(&reader) {
            Ok(Some(msg)) => {
                print!("\r\x1b[2K");
                println!("{}", msg.bright_white().bold());
                print!("{}", ".> ".red().bold());
                io::stdout().flush().unwrap();
            }
            Ok(None) => {
                shutdown.store(true, Ordering::Release);
                break;
            }
            Err(_) => {
                shutdown.store(true, Ordering::Release);

                return Err("\nRead error — assuming server is gone.");
            }
        }
    }

    Ok(())
}

/// Function to write user input to the server

use std::time::Duration;

pub fn spawn_writer(mut client: Client, shutdown: Arc<AtomicBool>, rx: std::sync::mpsc::Receiver<String>) -> Result<(), &'static str> {
    loop {
        if shutdown.load(Ordering::Acquire) {
            break;
        }

        match rx.recv_timeout(Duration::from_millis(200)) {
            Ok(msg) => {
                if msg == "/exit" {
                    break;
                }
                if let Err(_) = client.write_to_stream(&msg) {
                    shutdown.store(true, Ordering::Release);

                    return Err("Error Sending message to server");

                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // no message — loop again to check shutdown
                continue;
            }
            Err(_) => break, // disconnected
        }
    }

    Ok(())
}

