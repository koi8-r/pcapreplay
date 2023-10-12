pub struct PcapFileHeader {
    pub magic:u32,
    pub major:u16,
    pub minor:u16,
    pub tz:u32,
    pub accuracy:u32,
    pub snap_len:u32,
    pub link_type:u32,
}
pub struct PcapPacketHeader {
    pub hdr: PcapFileHeader,
    pub ts:u32,
    pub ms:u32,
    pub cap_len:usize,
    pub orig_len:usize,
}

impl Clone for PcapFileHeader {
    fn clone(&self) -> Self {
        Self {
            magic: self.magic,
            major: self.major,
            minor: self.minor,
            tz: self.tz,
            accuracy: self.accuracy,
            snap_len: self.snap_len,
            link_type: self.link_type,
        }
    }
}
impl Clone for PcapPacketHeader {
    fn clone(&self) -> Self {
        Self {
            hdr: self.hdr.clone(),
            ts: self.ts,
            ms: self.ms,
            cap_len: self.cap_len,
            orig_len: self.orig_len,
        }
    }
}
