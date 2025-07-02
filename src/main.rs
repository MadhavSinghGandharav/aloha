
mod server;
mod client;
mod common;

use std::env;
use std::thread;
use common::utils::LOCAL_HOST;

fn main() {
    let mut args = env::args();
    args.next(); // skip binary name

    match args.next() {
        Some(ip) => {
            client::connection::connect(&ip);
        }

        None => {
            // Start server
            thread::spawn(|| {
                server::listener::start();
            });

            // Give server time to start
            std::thread::sleep(std::time::Duration::from_millis(200));

            // Connect self as admin
            client::connection::connect(LOCAL_HOST);
        }
    }
}

