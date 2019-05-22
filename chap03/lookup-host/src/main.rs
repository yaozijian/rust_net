use std::env;
use dns_lookup::lookup_host; 

fn main(){
    let args : Vec<_> = env::args().collect();
    if args.len() != 2{
        eprintln!("需要提供一个主机名");
        std::process::exit(1);
    }else{
        let addr = lookup_host(&args[1]).unwrap();
        for a in addr{
            println!("{}",a);
        }
    }
}

