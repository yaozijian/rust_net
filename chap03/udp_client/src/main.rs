use std::net::UdpSocket;
use std::{str,io};

fn main() -> Result<(),io::Error>{
    
    let socket = UdpSocket::bind("127.0.0.1:8000")?;
    socket.connect("127.0.0.1:8888")?;
    
    let stdin = io::stdin();
    let mut input = String::new();
    let mut buffer = [0u8;1500];
    
    loop{
        input.clear();
        stdin.read_line(&mut input)?;
       
        socket.send(input.as_bytes())?;
   
        socket.recv_from(&mut buffer)?; 

        if let Ok(x) = str::from_utf8(&buffer){
            print!("{}",x);
        }
    }
}

