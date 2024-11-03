mod chat_client_protocol;

use std::net::{TcpStream};
use std::io::{self, Read, Write};
use chat_client_protocol::ChatClintProtocol;
use std::net::Shutdown;

const MAXLINE: usize = 4096;
const HOST: &str = "127.0.0.1";
const PORT: &str = "1234";
const EXIT: &str = "CLOSE";

fn main() -> io::Result<()> {
    let stream = TcpStream::connect(format!("{}:{}", HOST, PORT))?;
    println!("Connection to {} established", HOST);
    
    std::thread::scope(|scope| { 
        scope.spawn(|| send_thread(&stream));
    
        scope.spawn(|| receive_thread(&stream));
    });
   
    Ok(())
}

fn send_thread(mut stream: &std::net::TcpStream) {
    let mut chat_client_protocol = ChatClintProtocol::new();
    loop {
        let outmsg = chat_client_protocol.send_message();
        
        if outmsg == EXIT {
            break;
        }

        if let Err(e) = stream.write_all(outmsg.as_bytes()) {
            eprintln!("Error writing to socket: {}", e);
            break;
        }
    }
    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
}

fn receive_thread(mut stream: &std::net::TcpStream) {
    loop{
        let mut recvline = [0u8; MAXLINE];
        match stream.read(&mut recvline) {
            Ok(0) => break,
            Ok(bytes_read) => {
                let chat_client_protocol = ChatClintProtocol::new();
    
                let inmsg = String::from_utf8_lossy(&recvline[..bytes_read]);
                chat_client_protocol.receive_message(&inmsg);
            },
            Err(_) => {
                break;
            }
        };
    }
}
