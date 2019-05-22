use std::str;
use std::net::{TcpStream,SocketAddr};
use std::io;
use std::ops::Deref;
use std::cell::RefCell;
use std::io::{BufRead,BufReader,Write};
use std::time::Duration;

fn main() -> Result<(),std::io::Error>{

    let addr : SocketAddr = "127.0.0.1:8888".parse().unwrap();
    let stream = TcpStream::connect_timeout(&addr,Duration::from_millis(1500))?;
    println!("Connected: {} -> {}",stream.local_addr()?,stream.peer_addr()?);
    stream.set_read_timeout(Some(Duration::from_millis(3000))).unwrap();

    let c = RefCell::new(stream);
    let mut buffer : Vec<u8> = Vec::new();
    
    let stdin = io::stdin();
    let mut input = String::new();
    
    loop{
        input.clear();
        stdin.read_line(&mut input)?;
        
        c.borrow_mut().write(input.as_bytes())?;
   
        let t = c.borrow();
        let mut reader = BufReader::new(t.deref());
        buffer.clear();
        reader.read_until(b'\n',&mut buffer)?;

        if let Ok(x) = str::from_utf8(&buffer){
            print!("{}",x);
        }
    }
}
