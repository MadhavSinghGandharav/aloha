use std::io::{Read,Write};
use std::net::{TcpStream,UdpSocket,IpAddr,SocketAddr};

pub const PORT_ADDRESS: &str = "0.0.0.0:8080";
pub const LOCAL_HOST: &str ="127.0.0.1";


// function to read msg from the stream
pub fn read_from_stream(mut stream : &TcpStream) -> Result<Option<String>, std::io::Error> {
    let mut len_buf = [0; 2];

    // Try reading message length
    match stream.read_exact(&mut len_buf) {
        Ok(_) => (),
        Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
            // Client closed connection cleanly
            return Ok(None);
        }
        Err(e) => return Err(e),
    }

    let msg_len = u16::from_be_bytes(len_buf) as usize;
    let mut buf = vec![0; msg_len];

    // Try reading message body
    match stream.read_exact(&mut buf) {
        Ok(_) => Ok(Some(String::from_utf8_lossy(&buf).to_string())),
        Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => Ok(None),
        Err(e) => Err(e),
    }
}

// function to read input from the user
pub fn prompt(string: &str) -> String {
    let mut buffer = String::new();

    print!("{}", string);
    std::io::stdout().flush().unwrap();

    //mutating the buffer
    std::io::stdin().read_line(&mut buffer).expect("");

    buffer.trim().to_string()
}

// function to get local IP address
pub fn get_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("7.8.8.8:80").ok()?;

    let local_ip = socket.local_addr().ok()?;
    Some(local_ip.ip().to_string())
}

// funtion to connect to a TCP stream
pub fn connect_to_stream(ip:&str) -> Result<TcpStream,&'static str> {

    let ip: IpAddr = match ip.parse() {
        Ok(val) => val,
        Err(_) => return Err("Invalid IP")
    };

    let stream = match TcpStream::connect(SocketAddr::new(ip, 8080)) {
        Ok(val) => val,
        Err(_) => return Err("Unable to set stream")
    };

    Ok(stream)
}  
