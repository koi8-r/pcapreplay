#[derive(Debug)]
struct X {}

trait FromX<T: ?Sized> {
    fn from_x(reader: &mut dyn std::io::Read) -> Option<Self> where Self: Sized ;
    fn from_y(reader: &mut dyn std::io::Read) -> Option<Self> where Self: Sized ;
}

impl FromX<&mut dyn std::io::Read> for X {
    fn from_x(_reader: &mut dyn std::io::Read) -> Option<Self> {
        Some(Self{})
    }
    fn from_y(_reader: &mut dyn std::io::Read) -> Option<Self> {
        None
    }
}

fn main() {
    let r = &mut std::io::stdin().lock() ;
    println!("{:?}", X::from_x(r)) ;
    println!("{:?}", X::from_y(r)) ;
}
