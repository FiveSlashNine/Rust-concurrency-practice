mod worker_protocol;

use std::net::TcpStream;
use std::io::{self, Read, Write};
use worker_protocol::WorkerProtocol;

const MAXLINE: usize = 4096;
const HOST: &str = "127.0.0.1";
const PORT: &str = "1234";
const NUM_WORKERS: i32 = 4;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", HOST, PORT))?;
    println!("Connection to {} established", HOST);
    
    let worker_protocol = WorkerProtocol::new(NUM_WORKERS);
    
    let mut recvline = [0u8; MAXLINE];
    
    let bytes_read = stream.read(&mut recvline)?;

    let inmsg = String::from_utf8_lossy(&recvline[..bytes_read]);
    let outmsg = worker_protocol.compute(&inmsg);
    stream.write_all(outmsg.as_bytes())?;

    Ok(())
}
