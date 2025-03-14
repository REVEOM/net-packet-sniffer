use pnet::datalink::{self, NetworkInterface};
use pnet::packet::{Packet, ethernet::EthernetPacket, ip::IpNextHeaderProtocols};
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use std::env;

fn main() {
    // Ağ arayüzlerini listele
    let interfaces = datalink::interfaces();

    // Kullanıcıdan bir arayüz seçmesini iste
    let interface_name = env::args().nth(1).expect("Usage: net-packet-sniffer <interface>");
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == interface_name)
        .expect("Interface not found");

    // Paketleri dinle
    let (_, mut rx) = datalink::channel(&interface, Default::default())
        .expect("Failed to create datalink channel");

    println!("Listening on interface: {}", interface_name);

    loop {
        match rx.next() {
            Ok(packet) => {
                // Ethernet paketini al
                let eth_packet = EthernetPacket::new(packet).unwrap();
                handle_packet(&eth_packet);
            }
            Err(e) => {
                eprintln!("Failed to read packet: {}", e);
            }
        }
    }
}

fn handle_packet(eth_packet: &EthernetPacket) {
    match eth_packet.get_ethertype() {
        pnet::packet::ethernet::EtherTypes::Ipv4 => {
            println!("IPv4 Packet: {:?}", eth_packet);
        }
        pnet::packet::ethernet::EtherTypes::Ipv6 => {
            println!("IPv6 Packet: {:?}", eth_packet);
        }
        _ => {
            println!("Other Packet: {:?}", eth_packet);
        }
    }
}
