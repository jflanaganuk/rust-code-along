use std::net::TcpStream;
use std::io::Write;

fn main() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8000") {
        println!("Connected to the server!");
        println!("{:?}", stream);

        match stream.write(b"Hello?") {
            Ok(_) => println!("Sent message"),
            Err(e) => println!("{:?}", e)
        }
    } else {
        println!("Couldnt connect to server...");
    }

}
