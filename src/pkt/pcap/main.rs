mod model ;
mod reader ;
mod cap ;

use std::fs::File ;
use cap::CaptureDev ;
use reader::{PcapRead, PcapReader} ;

// #[derive(Debug, Clone)]
// pub struct Packet {
//     pub header: PcapPacketHeader,
//     pub data: Box<[u8]>,
// }
// because GaTs are unstable
// pub trait PacketCodec {
//     type Item ;
//     fn decode(&mut self, packet: PcapPacket) -> Self::Item ;
// }
// pub struct Codec ;
// impl PacketCodec for Codec {
//     type Item = Packet ;
//     fn decode(&mut self, pkt: PcapPacket) -> Self::Item {
//         Packet {
//             header: *pkt.header,
//             data: pkt.data.into(),
//         }
//     }
// }


fn main() {
    let mut _reader = File::open("assets/ping.pcap").expect("err: file") ;
    let mut reader = std::io::stdin() ;
    let cap = CaptureDev { reader: &mut reader } ;
    let mut reader = PcapReader{ cap } ;
    let hdr = reader.read_header() ;
    println!("{:?}", hdr) ;
    let pkt = reader.read_packet() ;
    println!("{:?}", pkt.header) ;
}
