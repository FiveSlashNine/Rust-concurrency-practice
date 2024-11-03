mod master_protocol;
mod pi;

use std::net::TcpListener;
use std::io::{self, Read, Write};
use std::sync::Arc;
use master_protocol::MasterProtocol;
use pi::Pi;
use std::thread;

const MAXLINE: usize = 4096;
const HOST: &str = "127.0.0.1";
const PORT: &str = "1234";
const NUM_WORKERS: usize = 4;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT))?;
    let mut threads = vec![];
    let pi = Arc::new(Pi::new(100000000));

    for id in 0..NUM_WORKERS {
        let pi = pi.clone();
        let (stream, _) = listener.accept().unwrap();
        let handle = thread::spawn(move || {
            let _ = server_thread(stream, id, pi);
        });
        threads.push(handle);
    }

    println!("All Started");

    for handle in threads {
        handle.join().unwrap();
    }

    pi.print_pi();
    
    Ok(())
}

fn server_thread(mut stream: std::net::TcpStream, id: usize, pi: Arc<Pi>) -> io::Result<()> {
    let master_protocol = MasterProtocol::new(pi, id as i32);
    let mut buf = [0u8; MAXLINE];

    let outmsg = master_protocol.prepare_request();
    stream.write_all(outmsg.as_bytes())?;
    
    let bytes_read = stream.read(&mut buf)?;
        
    let inmsg = String::from_utf8_lossy(&buf[..bytes_read]);

    master_protocol.process_reply(&inmsg);

    Ok(())
}

