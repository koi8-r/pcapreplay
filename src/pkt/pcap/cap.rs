pub struct CaptureDev<'a> {
    pub reader: &'a mut dyn std::io::Read,
}
impl<'a> CaptureDev<'a> {
    pub fn _read_on_stack(&mut self) {
        let mut buf: [u8; 24] = [0; 24] ;
        let _ = self.reader.read(&mut buf) ;
        println!("{:?}", buf) ;
    }
    // todo: use
    pub fn _read(&mut self, n: usize) -> Vec<u8> {
        let mut buf = vec![0u8; n] ;
        self.reader.read_exact(buf.as_mut_slice()).expect("") ;
        buf
    }
}
