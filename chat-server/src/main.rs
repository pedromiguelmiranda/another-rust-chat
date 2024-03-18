use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client( mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let client_stream = stream.try_clone().expect("Failed to clone stream.");

    {
        println!("Client connected: {}", client_stream.peer_addr().unwrap());
        let mut clients = clients.lock().unwrap();
        clients.push(client_stream.try_clone().expect("Failed to clone stream."));
    }

    let mut buf = [0; 512];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => {
                // let mut clients = clients.lock().unwrap();
                // clients.retain(|c| *c != stream);
                println!("Client disconnected: {}", stream.peer_addr().unwrap());
                return;
            }
            Ok(n) => {
                let msg = String::from_utf8_lossy(&buf[..n]);
                println!("Received message: {} from {}", msg, stream.peer_addr().unwrap());
                broadcast(&clients, msg);
            }
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
                return;
            }
        }
    }
}

fn broadcast(clients: &Arc<Mutex<Vec<TcpStream>>>, msg: std::borrow::Cow<'_, str>) {

    let clients = clients.lock().unwrap();
                
    for mut client in clients.iter() {
        let _ = client.write(msg.as_bytes());
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    println!("Server listening on port 8080...");

    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients = Arc::clone(&clients);
                thread::spawn(move || {
                    handle_client(stream, clients);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
