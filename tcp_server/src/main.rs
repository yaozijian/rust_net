
use std::net::{TcpListener,TcpStream};
use std::thread;
use std::io::{Read,Write,Error};
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::time::Duration;

fn handle_client(mut stream : TcpStream) -> Result<(),Error>{

  let addr = format!("{} -> {}",stream.peer_addr()?,stream.local_addr()?);
  let hint = |x| println!("{}: {}",x,addr);
  hint("接受连接");

  let choices = [0,1,2,3,4,5];

  let mut buf = [0; 512];
  loop {
    let bytes_read = stream.read(&mut buf)?;
    if bytes_read == 0 {
      hint("连接断开");
      return Ok(());
    }else{
      let mut rng = thread_rng();
      let secs = *(choices.choose(&mut rng).unwrap());
      let sleep = Duration::from_secs(secs);
      print!("休眠 {} 秒后再发送回应: {}",secs,std::str::from_utf8(&buf[..bytes_read]).unwrap());
      std::thread::sleep(sleep);
      stream.write(&buf[..bytes_read])?;
    }
  }
}

fn main(){
  let listener = TcpListener::bind("0.0.0.0:8888").expect("绑定失败");
  for stream in listener.incoming(){
    match stream{
      Err(e) => eprintln!("接受连接失败: {}",e),
      Ok(s) => {
        thread::spawn(move || {
          handle_client(s).unwrap_or_else(|e|eprintln!("{:?}",e));
        });
      }
  }
}
}

