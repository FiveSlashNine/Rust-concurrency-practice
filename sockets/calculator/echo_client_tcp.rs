mod client_protocol;

use std::net::{TcpStream};
use std::io::{self, Read, Write};
use client_protocol::ClientProtocol;

const MAXLINE: usize = 4096;
const EXIT: &str = "!";
const HOST: &str = "127.0.0.1";
const PORT: &str = "1234";

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", HOST, PORT))?;
    println!("Connection to {} established", HOST);
    
    let mut client_protocol = ClientProtocol::new();
    loop {
        let outmsg = client_protocol.prepare_request();

        stream.write_all(outmsg.as_bytes())?;

        if outmsg == EXIT { break; }

        let mut recvline = [0u8; MAXLINE];
        let bytes_read = stream.read(&mut recvline)?;
        if bytes_read == 0 {
            break;
        }

        let inmsg = String::from_utf8_lossy(&recvline[..bytes_read]);
        client_protocol.process_reply(&inmsg);
    }

    println!("Data Socket closed");

    Ok(())
}
