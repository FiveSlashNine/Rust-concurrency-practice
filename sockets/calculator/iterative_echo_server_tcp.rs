mod server_protocol;

use std::net::TcpListener;
use std::io::{self, Read, Write};
use server_protocol::ServerProtocol;

const MAXLINE: usize = 4096;
const HOST: &str = "127.0.0.1";
const PORT: &str = "1234";
const EXIT: &str = "CLOSE";

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT))?;

    let mut server_protocol = ServerProtocol;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Server is listening to port: {}", PORT);
                println!("Received request from {}", listener.peer_addr().unwrap());
                
                let mut buf = [0u8; MAXLINE];
                loop {
                    let bytes_read = stream.read(&mut buf)?;
                    if bytes_read == 0 {
                        break;
                    }

                    let inmsg = String::from_utf8_lossy(&buf[..bytes_read]);
                    let outmsg = server_protocol.process_request(&inmsg);

                    if outmsg == EXIT {
                        break;
                    }

                    stream.write_all(outmsg.as_bytes())?;
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    Ok(())
}
