use std::env;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;
use trust_dns::rr::record_type::RecordType;

fn main(){
    let name = env::args().nth(1).unwrap();
    let query = format!("{}.",name);
    
    let resolver = Resolver::new(ResolverConfig::default(),ResolverOpts::default()).unwrap();
    let ack = resolver.lookup_ip(query.as_str());
    println!("同步解析：");
    for x in ack.iter(){
        println!("{:?}",x);
    }

    println!("\n\n系统解析器");
    let sys = Resolver::from_system_conf().unwrap();
    let ack = sys.lookup_ip(query.as_str());
    for x in ack.iter(){
        println!("{:?}",x);
    }

    let ns = resolver.lookup(query.as_str(),RecordType::NS);
    println!("\n\n同步解析器取得的NS");
    for x in ns.iter(){
        println!("{:?}",x);
    }
}

