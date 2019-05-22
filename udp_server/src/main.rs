use std::thread;
use std::net::UdpSocket;

fn main() -> Result<(),std::io::Error>{
    let s = UdpSocket::bind("0.0.0.0:8888")?;
    loop{
        let mut buf = [0u8;1500];
        let sock = s.try_clone()?;
        match sock.recv_from(&mut buf){
            Err(e) => eprintln!("接收出错: {}",e),
            Ok((n,src)) =>{
                thread::spawn(move ||{
                    let x = std::str::from_utf8(&buf[..n]).unwrap();
                    print!("收到来自 {} 的报文 {}",src,x);
                    sock.send_to(&buf[..n],&src).unwrap();
                });
            },
        }
    }
}

