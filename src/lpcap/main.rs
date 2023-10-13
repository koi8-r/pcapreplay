use clap::Parser ;  // https://docs.rs/clap/latest/clap/struct.Arg.html

#[derive(Debug, Parser)]
struct Cli {
    #[structopt(global=true, default_value="-")]
    file: String,
    #[structopt(short='v', global=true, required=false, default_value="false")]
    verbose: bool,
    #[structopt(short='l')]
    list: bool,
}

fn show_ifaces() {
    for dev in pcap::Device::list().expect("device lookup error") {
        println!("{}:", dev.name) ;
        for addr in dev.addresses {
            print!("  ") ;
            if addr.addr.is_ipv4() { print!("ipv4: ") } else { print!("ipv6: ") }
            println!("{}/{}", addr.addr, addr.netmask.unwrap()) ;
        }
    }
}


fn main() -> () {
    let args = Cli::parse() ;

    if args.list {
        show_ifaces()
    } ;
}
