//imports 
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read from client");
    //this line converts the data in the buffer into utf-8 encoded string
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("recieved request: {}",request);
    let response = "hello client".as_bytes();
    stream.write(response).expect("Failed to write response");
}

fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").expect("failed to bind address");
    println!("server listening on 127.0.0.1:8080");

    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("failed to establish connection: {}", e);
            }
        }
    }
}
