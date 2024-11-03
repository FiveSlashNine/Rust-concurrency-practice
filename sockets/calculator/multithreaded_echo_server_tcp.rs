mod server_protocol;

use std::net::TcpListener;
use std::io::{self, Read, Write};
use server_protocol::ServerProtocol;
use std::thread;

const MAXLINE: usize = 4096;
const HOST: &str = "127.0.0.1";
const PORT: &str = "1234";
const EXIT: &str = "!";

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT))?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Received request from {}", stream.peer_addr().unwrap());
                server_thread(stream)?;
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}

fn server_thread(mut stream: std::net::TcpStream) -> io::Result<()> {
    thread::spawn(move || {
        let server_protocol = ServerProtocol;
        let mut buf = [0u8; MAXLINE];
        loop {
            let bytes_read = match stream.read(&mut buf) {
                Ok(0) => break,
                Ok(bytes_read) => bytes_read,
                Err(e) => {
                    eprintln!("Error reading from socket: {}", e);
                    break;
                }
            };

            let inmsg = String::from_utf8_lossy(&buf[..bytes_read]);
            let outmsg = server_protocol.process_request(&inmsg);

            if outmsg == EXIT {
                break;
            }

            if let Err(e) = stream.write_all(outmsg.as_bytes()) {
                eprintln!("Error writing to socket: {}", e);
                break;
            }
        }
    });
    Ok(())
}
