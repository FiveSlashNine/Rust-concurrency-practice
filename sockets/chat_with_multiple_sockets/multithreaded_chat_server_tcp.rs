mod server_protocol;

use std::net::TcpListener;
use std::io::{self, Read, Write};
use server_protocol::ServerProtocol;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use std::thread;

const MAXLINE: usize = 4096;
const HOST: &str = "127.0.0.1";
const PORT: &str = "1234";
const EXIT: &str = "CLOSE";

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT))?;

    let connection_list: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        println!("Server is waiting for a client in port: {}", PORT);
        match stream {
            Ok(stream) => {   
                println!("Received request from {}", stream.peer_addr().unwrap());

                let connection_list = Arc::clone(&connection_list);
                
                connection_list.lock().unwrap().push(stream.try_clone().unwrap());
                thread::spawn(|| server_thread(stream, connection_list));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }

    }

    Ok(())
}

fn server_thread(mut stream: TcpStream, connection_list: Arc<Mutex<Vec<TcpStream>>>) -> io::Result<()> {
    let server_protocol = ServerProtocol;
    let mut buf = [0u8; MAXLINE];
    let peer_addr = stream.peer_addr().unwrap();

    loop {
        match stream.read(&mut buf) {
            Ok(0) => {
                println!("Data socket closed");
                break;
            },
            Ok(bytes_read) =>{
                let inmsg = String::from_utf8_lossy(&buf[..bytes_read]);
                let outmsg = server_protocol.process_request(&inmsg);
        
                if outmsg == EXIT {
                    break;
                }
        
                let connection_list = connection_list.lock().unwrap();
                for mut client in &*connection_list {
                    if client.peer_addr().unwrap() != peer_addr {
                        if let Err(_) = client.write_all(outmsg.as_bytes()) {
                          continue;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
                break;
            }
        };
    }

    let mut connection_list = connection_list.lock().unwrap();
    connection_list.retain(|client| client.peer_addr().unwrap() != peer_addr);

    Ok(())
}
