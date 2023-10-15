mod models ;

use std::io ;
use std::io::ErrorKind::UnexpectedEof ;
use std::io::{IsTerminal, Read} ;
// use crate::models::{PcapFileHeader, PcapPacketHeader} ;

struct ActCtx {  // struct Viz<'a> {
    // reader:&'a mut dyn io::Read,
}

impl ActCtx {  // impl<'a> Viz<'a> {
    fn new() -> Self {
        Self {
            // reader: &mut io::stdin()
        }
    }
    fn read_onstack(  // fn read<'a>(
        &mut self, buf:&mut [u8],  // buf:&'a mut [u8],
    ) -> io::Result<&[u8]> {
        let _is_tty = io::stdin().is_terminal() ;
        // println!("tty={is_tty}") ;
        match io::stdin().read_exact(buf) {
            Ok(_) => Ok(&[0; 1]),
            Err(e) => Err(e),
        }
    }
    #[allow(dead_code)]
    fn read_onheap(
          &mut self, n: usize,
    ) -> io::Result<Vec<u8>> {
        let mut buf:Vec<u8> = vec![0u8; n] ;
        match io::stdin().read_exact(&mut buf) {
            Ok(_) => Ok(buf.clone()),
            Err(e) => Err(e),
        }
    }
    fn parse(&mut self) {
        let mut act: Box<dyn Act> = Box::new(FileHdrEv{}) ;
        let mut n = 0usize ;
        while !act.is_eof() {
            act = act.call(self) ;
            println!("n={}", n / 2) ;
            n += 1 ;
        }
        println!("EOF") ;
    }
}

trait Act {
    // fn call(&mut self, viz:&mut ActCtx) -> &mut dyn Act ;
    fn call(&mut self, viz:&mut ActCtx) -> Box<dyn Act> ;
    fn is_eof(&self) -> bool ;
}

struct FileHdrEv {}
struct PktHdrEv {
    hdr: models::PcapFileHeader,
}
struct PktBdyEv {
    hdr: models::PcapPacketHeader,
}
struct FileEofEv {}

impl Act for FileHdrEv {
    fn call(
        &mut self, viz:&mut ActCtx,
    // ) -> &mut dyn Act {
    ) -> Box<dyn Act> {  // move instead
        let mut buf = [0u8; 24] ;
        viz.read_onstack(&mut buf).expect("read fail") ;
        let (
            magic, major, minor, tz, accuracy, snap_len, link_type
        ) = (
            u32::from_le_bytes(buf[0..4].try_into().unwrap()),
            u16::from_le_bytes(buf[4..6].try_into().unwrap()),
            u16::from_le_bytes(buf[6..8].try_into().unwrap()),
            u32::from_le_bytes(buf[8..12].try_into().unwrap()),
            u32::from_le_bytes(buf[12..16].try_into().unwrap()),
            u32::from_le_bytes(buf[16..20].try_into().unwrap()),
            u32::from_le_bytes(buf[20..24].try_into().unwrap()),
        ) ;
        println!("magic={magic} version={major}.{minor}") ;
        assert_eq!(0xA1B2C3D4, magic) ;
        Box::new(PktHdrEv{ hdr: models::PcapFileHeader{
            magic, major, minor, tz, accuracy, snap_len, link_type,
        }})
    }
    fn is_eof(&self) -> bool { false }
}
impl Act for PktHdrEv {
    fn call(
        &mut self, viz:&mut ActCtx,
    // ) -> &mut dyn Act {
    ) -> Box<dyn Act> {  // move instead
        let mut buf = [0u8; 16] ;
        // viz.read_onstack(&mut buf).expect("read fail") ;
        match viz.read_onstack(&mut buf) {
            Ok(_) => Ok(()),
            Err(e) if e.kind() == UnexpectedEof => {
                return Box::new(FileEofEv{})
            },
            Err(e) => Err(e),
        }.expect("read fail") ;

        let (
            ts, ms, cap_len, orig_len,
        ) = (
            u32::from_le_bytes(buf[0..4].try_into().unwrap()),
            u32::from_le_bytes(buf[4..8].try_into().unwrap()),
            // usize::from_le_bytes(buf[8..12].try_into().unwrap()),
            // usize::from_le_bytes(buf[12..16].try_into().unwrap()),
            u32::from_le_bytes(buf[8..12].try_into().unwrap()) as usize,
            u32::from_le_bytes(buf[12..16].try_into().unwrap()) as usize,
        ) ;
        println!("ts={ts}.{ms} len={orig_len}b") ;
        let fhdr = self.hdr.clone() ;
        let hdr = models::PcapPacketHeader{
            hdr: fhdr, ts, ms, cap_len, orig_len,
        } ;
        Box::new(PktBdyEv{ hdr })
    }
    fn is_eof(&self) -> bool { false }
}
impl Act for PktBdyEv {
    fn call(
        &mut self, viz:&mut ActCtx,
        // ) -> &mut dyn Act {
    ) -> Box<dyn Act> {  // move instead
        let mut buf = vec![0u8; self.hdr.cap_len] ;
        viz.read_onstack(&mut buf).expect("read fail") ;
        // println!("{:02X?}", buf) ;
        println!(
            "{}",
            buf.iter()
            .map(|b| format!("{:02x}", b).to_string())
            .collect::<Vec<String>>().join("")
        ) ;
        // Box::new(FileEofEv{})
        Box::new(PktHdrEv{ hdr: self.hdr.hdr.clone() })
    }
    fn is_eof(&self) -> bool { false }
}
impl Act for FileEofEv {
    fn call(&mut self, _: &mut ActCtx) -> Box<dyn Act> {
        Box::new(FileEofEv{})
    }
    fn is_eof(&self) -> bool { true }
}

fn main() {
    let mut viz = ActCtx::new() ;
    viz.parse() ;
}
