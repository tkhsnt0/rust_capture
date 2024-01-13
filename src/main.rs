extern crate pnet;

use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use std::env;
mod packets;
use packets::GettableEndPoints;
const WIDTH: usize = 20;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please specify target interface name");
        std::process::exit(1);
    }
    let interface_name = &args[1];

    //マシンのネットワークインターフェース一覧を取得
    let interfaces = datalink::interfaces();

    //一覧から引数で指定したインターフェースを取得
    let interface = interfaces
        .into_iter()
        .filter(|iface| iface.name == *interface_name)
        .next()
        .expect("Failed to get interface.");

    //取得したネットワークインターフェースからチャネルを取得
    //チャネルの実体はソケットであり、デフォルトではAF_PACKET(データリンク)が返される
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => {
            panic!("Fialed to create datalink channel {}", e)
        }
    };

    loop {
        match rx.next() {
            Ok(frame) => {
                //受信データからイーサネットフレームの構築
                let frame = EthernetPacket::new(frame).unwrap();
                match frame.get_ethertype() {
                    EtherTypes::Ipv4 => {
                        //イーサネットフレームからIPv4処理へ
                        ipv4_handler(&frame);
                    }
                    EtherTypes::Ipv6 => {
                        //イーサネットフレームからIPv6処理へ
                        ipv6_handler(&frame);
                    }
                    _ => {
                        println!("Not a ipv4 or ipv6");
                    }
                }
            }
            Err(e) => {
                panic!("Failed to read: {}", e);
            }
        }
    }
}

fn print_packet_info(l3: &dyn GettableEndPoints, l4: &dyn GettableEndPoints, proto: &str) {
    println!(
        "Captured a {} packet from {}|{} to {}|{}\n",
        proto,
        l3.get_source(),
        l4.get_source(),
        l3.get_destination(),
        l4.get_destination()
    );
    let payload = l4.get_payload();
    let len = payload.len();

    for i in 0..len {
        print!("{:<02X} ", payload[i]);
        //20byte毎に
        if i % WIDTH == WIDTH - 1 || i == len - 1 {
            //受信データの隣にブランク+パイプを表示し
            for _j in 0..WIDTH - (i % (WIDTH)) {
                print!("    ");
            }
            print!("| ");
            //受信データのASCII文字を表示し
            for j in i - 1 % WIDTH..i + 1 {
                if payload[j].is_ascii_alphabetic() {
                    print!("{}", payload[j] as char);
                } else {
                    print!(".");
                }
            }
            //次の行へ
            print!("\n");
        }
    }
    //1byteデータ+空白で3文字分として"="を表示
    println!("{}", "=".repeat(WIDTH * 3));
    print!("\n");
}

fn ipv4_handler(ethernet: &EthernetPacket) {
    //イーサネットフレームのペイロードからIPv4のパケットを取得
    if let Some(packet) = Ipv4Packet::new(ethernet.payload()) {
        match packet.get_next_level_protocol() {
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(&packet);
            }
            IpNextHeaderProtocols::Udp => {
                udp_handler(&packet);
            }
            _ => {
                println!("Not a tcp or udp packet");
            }
        }
    }
}
fn ipv6_handler(ethernet: &EthernetPacket) {
    //イーサネットフレームのペイロードからIPv6のパケットを取得
    if let Some(packet) = Ipv6Packet::new(ethernet.payload()) {
        match packet.get_next_header() {
            IpNextHeaderProtocols::Tcp => {
                tcp_handler(&packet);
            }
            IpNextHeaderProtocols::Udp => {
                udp_handler(&packet);
            }
            _ => {
                println!("Not a tcp or udp packet");
            }
        }
    }
}
fn tcp_handler(packet: &dyn GettableEndPoints) {
    //IPパケットのペイロードからTCPセグメントを取得
    let tcp = TcpPacket::new(packet.get_payload());
    if let Some(tcp) = tcp {
        print_packet_info(packet, &tcp, "TCP");
    }
}
fn udp_handler(packet: &dyn GettableEndPoints) {
    //IPパケットのペイロードからUDPセグメントを取得
    let udp = UdpPacket::new(packet.get_payload());
    if let Some(udp) = udp {
        print_packet_info(packet, &udp, "UDP");
    }
}
