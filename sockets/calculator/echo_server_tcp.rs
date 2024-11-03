mod server_protocol;

use std::net::TcpListener;
use std::io::{self, Read, Write};
use server_protocol::ServerProtocol;

const MAXLINE: usize = 4096;
const HOST: &str = "127.0.0.1";
const PORT: &str = "1234";
const EXIT: &str = "!";

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT))?;

    let server_protocol = ServerProtocol;
    if let Ok((mut stream, _)) = listener.accept() {
        println!("Received request from {}", stream.peer_addr().unwrap());

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

    Ok(())
}
