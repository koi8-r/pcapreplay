pub const PCAP_FILE_MAGIC:u32 = 0xA1B2C3D4 ;

#[derive(Debug, Copy, Clone)]
pub struct PcapFileHeader {
    pub magic:u32,
    pub major:u16,
    pub minor:u16,
    pub tz:u32,
    pub accuracy:u32,
    pub snap_len:u32,
    pub link_type:u32,
}
#[derive(Debug, Copy, Clone)]
pub struct PcapPacketHeader {
    pub ts: u32,         // seconds
    pub us: u32,         // microseconds
    pub cap_len: usize,  // captured len
    pub pkt_len: usize,  // real len
}
#[derive(Debug, Clone)]
pub struct _PcapPacket<'a> {
    pub header: &'a PcapPacketHeader,
    pub data: &'a [u8],
}

#[derive(Debug, Clone)]
pub struct PcapPacket {
    pub header: PcapPacketHeader,
    pub data: Vec<u8>,
}

pub trait FromReader {
    fn from_reader(reader: &mut dyn std::io::Read) -> Self ;
}
impl From<&mut dyn std::io::Read> for PcapPacket {
    fn from(reader: &mut dyn std::io::Read) -> Self {
        Self::from_reader(reader)
    }
}
impl FromReader for PcapPacket {
    fn from_reader(reader: &mut dyn std::io::Read) -> Self {
        let mut buf: [u8; 16] = [0; 16] ;
        // todo: return Option::None on EOF
        reader.read_exact(&mut buf).expect("err: read pkt hdr") ;
        let hdr = PcapPacketHeader{
            ts: u32::from_le_bytes(buf[0..4].try_into().unwrap()),
            us: u32::from_le_bytes(buf[4..8].try_into().unwrap()),
            cap_len: u32::from_le_bytes(buf[8..12].try_into().unwrap()) as usize,
            pkt_len: u32::from_le_bytes(buf[12..16].try_into().unwrap()) as usize,
        } ;
        println!("read body {}b", hdr.cap_len) ;
        let mut body = vec![0u8; hdr.cap_len] ;
        reader.read_exact(body.as_mut_slice()).expect("err: read pkt body") ;
        PcapPacket{ header: hdr, data: body }
    }
}
impl FromReader for PcapFileHeader {
    fn from_reader(reader: &mut dyn std::io::Read) -> Self {
        let mut buf: [u8; 24] = [0; 24] ;
        reader.read_exact(&mut buf).expect("err: read file hdr") ;
        let ret = PcapFileHeader{
            magic: u32::from_le_bytes(buf[0..4].try_into().unwrap()),
            major: u16::from_le_bytes(buf[4..6].try_into().unwrap()),
            minor: u16::from_le_bytes(buf[6..8].try_into().unwrap()),
            tz: u32::from_le_bytes(buf[8..12].try_into().unwrap()),
            accuracy: u32::from_le_bytes(buf[12..16].try_into().unwrap()),
            snap_len: u32::from_le_bytes(buf[16..20].try_into().unwrap()),
            link_type: u32::from_le_bytes(buf[20..24].try_into().unwrap()),
        } ;
        assert_eq!(ret.magic, PCAP_FILE_MAGIC) ;
        ret
    }
}