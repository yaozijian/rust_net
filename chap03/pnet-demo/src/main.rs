
use pnet::datalink::{self,NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes,EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;

use std::env;

fn handle_packet(ethernet: &EthernetPacket){
    match ethernet.get_ethertype(){
        EtherTypes::Ipv4 => {
            let header = Ipv4Packet::new(ethernet.payload());
            if let Some(header) = header{
                match header.get_next_level_protocol(){
                    IpNextHeaderProtocols::Tcp => {
                        let tcp = TcpPacket::new(header.payload());
                        if let Some(tcp) = tcp{
                            println!(
                                "TCP {}:{} -> {}:{}",
                                header.get_source(),tcp.get_source(),
                                header.get_destination(),tcp.get_destination()
                            );
                        }
                    }
                    _ => (),
                }
            }
        }
        _ => ()
    }
}

fn main(){
    let name = env::args().nth(1).unwrap();
    let nics = datalink::interfaces();
    let nic = nics.into_iter().filter(|nic:&NetworkInterface|{
        nic.name == name
    }).next().expect("找不到指定名称的网卡");

    let (_tx,mut rx) = match datalink::channel(&nic,Default::default()){
        Ok(Ethernet(tx,rx)) => (tx,rx),
        Ok(_) => panic!("不是以太网卡"),
        Err(e) => {
            panic!("创建数据链路通道失败： {}",e);
        }
    };

    loop{
        match rx.next(){
            Ok(pkt) => {
                let p = EthernetPacket::new(pkt).unwrap();
                handle_packet(&p);
            }
            Err(e) => {
                panic!("获取报文出错：{}",e);
            }
        }
    }
}

