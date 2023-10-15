use crate::cap::CaptureDev ;
use crate::model::{
    FromReader,
    PcapFileHeader,
    PcapPacket,
} ;

pub trait PcapRead {
    fn read_header(&mut self) -> PcapFileHeader ;
    fn read_packet(&mut self) -> PcapPacket ;
    // fn parse_with(parser: PacketParser) ;
}

pub struct PcapReader<'a> {
    pub cap: CaptureDev<'a>,
}
impl<'a> PcapRead for PcapReader<'a> {
    fn read_header(&mut self) -> PcapFileHeader {
        PcapFileHeader::from_reader(self.cap.reader)
    }
    fn read_packet(&mut self) -> PcapPacket {
        PcapPacket::from_reader(self.cap.reader)
    }
}
impl<'a> Iterator for PcapReader<'a> {
    type Item = PcapPacket ;
    fn next(&mut self) -> Option<Self::Item> {  // todo: None on EOF
        todo!()
    }
}
