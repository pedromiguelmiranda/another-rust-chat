use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, BufRead, Read, Write};
use std::net::{Shutdown, TcpStream};
use std::thread;
use termion::color::{self};

/// JSON based message to send to server
#[derive(Serialize, Deserialize)]
pub struct Message {
    ip: String,
    username: String,
    msg: String,
}

/// * Handles messages received from tcp stream on dedicated thread
///
/// - parses received from a tcp stream into a json structure.
/// - pretty prints parsed json.
///
/// return: thread join handler
fn handle_messages(stream_clone: TcpStream, username: String) -> thread::JoinHandle<()> {
    let receive_handle = {
        thread::spawn(move || {
            let mut reader = io::BufReader::new(&stream_clone);
            let mut buffer = [0; 512];
            let mut contacts = HashMap::new();

            loop {
                match reader.read(&mut buffer) {
                    Ok(bytes_read) => {
                        if bytes_read == 0 {
                            println!("Server disconnected.");
                            break;
                        }

                        //de-serialize Json
                        let json_data: Message =
                            serde_json::from_slice(&buffer[..bytes_read]).unwrap();

                        // skip our self message
                        if json_data.username == username {
                            continue;
                        }

                        //fetch color
                        let color = {
                            match contacts.get(&json_data.ip) {
                                Some(&saved_color) => saved_color,
                                _ => {
                                    let color = rand::random::<u8>();
                                    contacts.insert(json_data.ip, color);
                                    color
                                }
                            }
                        };

                        //print
                        let timestamp = Utc::now();
                        println!(
                            "{} [{}] {}: {}{}",
                            color::Fg(color::AnsiValue(color)),
                            timestamp.format("%H:%M:%S"),
                            json_data.username,
                            color::Fg(color::White),
                            json_data.msg
                        );
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        })
    };
    receive_handle
}

/// Gets username from stdin
/// If no username is read from stdin, "anon" username is given
fn get_username() -> Result<String, io::Error> {
    let mut username = String::new();

    println!(" Please enter your username:");
    io::stdin().read_line(&mut username)?;
    username.pop(); // remove newline char '\n'

    if username.is_empty() {
        username = String::from("anon");
    }
    Ok(username)
}

/// Read and parses input from stdin
/// - serialize as JSON
/// - sends message via TCP stream
/// Returns Result enum
fn handle_input(stream: TcpStream, username: &String) -> Result<(), io::Error> {
    let mut reader = io::BufReader::new(io::stdin());
    let mut writer = io::BufWriter::new(&stream);
    println!("You can start chatting. Enter /quit to exit.");
    Ok(loop {
        let mut input = String::new();

        reader.read_line(&mut input)?;
        input.pop();
        if input.trim() == "/quit" {
            stream.shutdown(Shutdown::Both)?;
            break;
        }

        if input.is_empty() {
            continue;
        }

        let msg = Message {
            ip: stream.local_addr().unwrap().to_string(),
            username: username.to_owned(),
            msg: input.to_owned(),
        };

        let msg_data = serde_json::to_string(&msg)?;

        writer.write_all(msg_data.as_bytes())?;
        writer.flush()?;
    })
}

fn main() -> io::Result<()> {
    println!("Enter the server IP address and port (e.g., 127.0.0.1:8080):");
    let mut server_address = String::new();
    io::stdin().read_line(&mut server_address)?;

    let stream = TcpStream::connect(server_address.trim())?;
    println!("\nConnected to server.");

    let username = get_username()?;
    let stream_clone = stream.try_clone().unwrap();
    let receive_handle = handle_messages(stream_clone, username.clone());

    handle_input(stream, &username)?;

    receive_handle.join().unwrap();
    Ok(())
}
