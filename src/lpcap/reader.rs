use std::os::fd::AsRawFd;
use pcap::{Capture, Packet};
use pcap ;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketOwned {
    pub header: pcap::PacketHeader,
    pub data: Box<[u8]>,
}


pub struct Codec ;
impl pcap::PacketCodec for Codec {
    type Item = PacketOwned ;

    fn decode(&mut self, pkt: Packet) -> Self::Item {
        PacketOwned {
            header: *pkt.header,
            data: pkt.data.into(),
        }
    }
}


pub fn reader() -> Result<(), Box<dyn std::error::Error>> {
    // pcap::raw::pcap_open_offline()
    unsafe {
        let cap = Capture::from_raw_fd(
            std::io::stdin().as_raw_fd()
        ) ;
    }
    let cap = Capture::from_file("-").unwrap() ;
    for pkt in cap.iter(Codec) {
        let pkt = pkt? ;
        println!("{:?}", pkt)
    }
    Ok(())
}

fn main() {
    let _ = reader() ;
}
