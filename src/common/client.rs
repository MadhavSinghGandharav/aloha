

use std::net::TcpStream;
use std::io::Write;
use std::sync::Arc;

pub struct Client {
    pub username: Arc<str>,
    pub stream: TcpStream,
}

impl Client {
    pub fn new(username: String, stream: TcpStream) -> Self {
        Self {
            username: Arc::from(username),
            stream ,
        }
    }

    pub fn send_initial_header(&mut self) -> std::io::Result<()> {
        let msg = &*self.username; // borrow as &str
        let len = msg.len() as u16;

        self.stream.write_all(&len.to_be_bytes())?;
        self.stream.write_all(msg.as_bytes())?;    

        Ok(())
    }
    pub fn write_to_stream(&mut self, msg: &str) -> std::io::Result<()> {

        let len = msg.len() as u16;
        self.stream.write_all(&len.to_be_bytes())?;
        self.stream.write_all(msg.as_bytes())?;    

        Ok(())
    }
}

