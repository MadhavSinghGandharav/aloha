
mod server;
mod client;
mod common;

use std::env;
use std::thread;
use common::utils::LOCAL_HOST;
use owo_colors::OwoColorize;
use std::process;

fn main() {
    let mut args = env::args();
    args.next(); // skip binary name

    match args.next() {
        Some(ip) => {
            match client::connection::connect(&ip){
                Ok(_) => {
                    println!("\n{}","Connection Closed".bright_red().bold());                    
                    process::exit(1);

                },
                Err(e) => {
                    eprintln!("{}",e.bright_red().bold());
                    process::exit(1);
                }
            };
        }

        None => {
            // Start server
            thread::spawn(|| {
                match server::listener::start(){
                    Ok(_) => {
                        println!("\n{}","Closing Server".bright_red().bold());
                        process::exit(1);
                    },
                    Err(e) => {
                        eprintln!("\n{}: {}","Server Error".bright_red().bold(),e);
                        process::exit(1);
                    }
                };
            });

            // Give server time to start
            std::thread::sleep(std::time::Duration::from_millis(200));

            // Connect self as admin
            match client::connection::connect(LOCAL_HOST){
                Ok(_) => {
                    println!("\n{}","Connection Closed".bright_red().bold());                    
                    process::exit(1);

                },
                Err(e) => {
                    eprintln!("{}",e.bright_red().bold());
                    process::exit(1);
                }
            };

        }
    }
}

