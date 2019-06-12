#![feature(type_alias_enum_variants)]

use std::str;
use nom::{ErrorKind,IResult};
use nom::{named,return_error,alt,map_res,map};
use nom::{do_parse,tag,take_until,is_a};

#[derive(Debug)]
enum Method{
    GET,
    POST,
}

#[derive(Debug)]
struct Request{
    method: Method,
    url: String,
    version: String,
}

named!(parse_method<&[u8],Method>,
    return_error!(
        ErrorKind::Custom(12),
        alt!(
            map!(tag!("GET"),|_|Method::GET) |
            map!(tag!("POST"),|_|Method::POST)
        )
    )
);

named!(parse_request<&[u8],Request>,
    do_parse!(
        method: parse_method >>
        is_a!("\x20") >>
        url: map_res!(take_until!("\x20"),str::from_utf8) >>
        is_a!("\x20") >>
        tag!("HTTP/") >>
        version: map_res!(take_until!("\r\n"),str::from_utf8) >>
        tag!("\r\n") >>
        (Request{
            method: method,
            url: url.into(),
            version: version.into(),
        })
    )
);

fn run_parser(input: &str){
    match parse_request(input.as_bytes()){
        IResult::Ok((rest,value)) => println!("Rest: {:?} Value: {:?}",rest,value),
        IResult::Err(err) => println!("{}",err),
    }
}

fn main(){
    let get = "GET     /home/     HTTP/1.1\r\n";
    run_parser(get);

    let post = "POST /update/ HTTP/1.1\r\n";
    run_parser(post);

    let wrong = "WRONG /wrong/ HTTP/1.1\r\n";
    run_parser(wrong);
}




