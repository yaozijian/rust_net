use std::{env,str};
use std::net::{UdpSocket,Ipv4Addr};

fn main(){
    let mcast_group : Ipv4Addr = "239.0.0.1".parse().unwrap();
    let port = 6000u16;
    let any = "0.0.0.0".parse().unwrap();
    let mut buffer = [0u8;1600];
    if env::args().count() > 1{
        let socket = UdpSocket::bind((any,port)).expect("创建UDP套接字失败");
        socket.join_multicast_v4(&mcast_group,&any).expect("加入多播组失败");
        socket.recv_from(&mut buffer).expect("接收失败");
        print!("{}",str::from_utf8(&buffer).unwrap());
    }else{
        let socket = UdpSocket::bind((any,0)).expect("创建服务套接字失败");
        socket.send_to("Hello world!".as_bytes(),&(mcast_group,port)).expect("发送数据失败");
    }
}
