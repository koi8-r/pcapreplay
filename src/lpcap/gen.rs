#![allow(dead_code)]
use tokio :: sync :: mpsc ;
use pcap :: {
    Packet,
    PacketCodec,
    PacketStream,
    Capture,
    Active,
    Inactive,
    Error
} ;

// it returns owned copies
pub struct BoxCodec ;
impl PacketCodec for BoxCodec {
    type Item = Box<[u8]> ;

    fn decode(&mut self, pkt: Packet) -> Self::Item {
        pkt.data.into()
    }
}

fn create_stream(
    cap: Capture<Inactive>
) -> Result<PacketStream<Active, BoxCodec>, Error> {
    let new_cap = cap
    .promisc(true)
    .immediate_mode(true)  // no buffering
    .open()?
    ;
    new_cap.stream(BoxCodec)
}


async fn gen_l2_pkt() -> Vec<u8> {
    let mut pkt: Vec<u8> = Vec::new() ;
    type Mac = [u8;6] ;
    let src_mac: Mac = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF] ;
    let dst_mac: Mac = [0x00, 0xC0, 0xCA, 0xAE, 0xBE, 0x03] ;
    pkt.extend_from_slice(&dst_mac) ;
    pkt.extend_from_slice(&src_mac) ;
    pkt.extend_from_slice(&[0x08, 0x00]) ;  // IPv4
    pkt.extend_from_slice("PING".as_bytes()) ;
    pkt
}


#[tokio::main]
async fn main() {
    let cap = Capture::from_device("en0").unwrap() ;
    let (tx, mut rx): (mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>) =
        mpsc::channel(1024*4)
    ;
    tokio::spawn(async move {
        let mut stream = create_stream(cap).expect("create stream failed") ;
        loop {
        }
    }) ;

}
