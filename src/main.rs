use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    match listener.accept() {
        Ok((socket, addr)) => {
            println!("new client: {}", addr);
            handle_client(socket);
        }
        Err(e) => println!("got e: {}", e),
    }
}

fn handle_client(mut socket: TcpStream) {
    let mut line;
    loop {
        line = [0; 512];
        match socket.read(&mut line) {
            Ok(_) => {
                let s = String::from_utf8_lossy(&line);
                println!("server got message: {}", s);
                let _ = socket.write(&line);
            }
            Err(e) => {
                panic!("got error: {}", e)
            }
        }
    }
}

