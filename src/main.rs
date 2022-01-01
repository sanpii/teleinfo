#![warn(warnings)]

use clap::Parser;

mod teleinfo;

#[derive(Parser)]
struct Opt {
    device: String,
}

fn main() {
    let opt = Opt::parse();

    let parser = teleinfo::Parser::new();

    let frame = match parser.read_frame(opt.device) {
        Ok(frame) => frame,
        Err(err) => panic!("Unable to read device: {}", err),
    };

    let data = match parser.parse(frame) {
        Ok(data) => data,
        Err(err) => panic!("Unable to parse frame: {}", err),
    };

    match serde_json::to_string(&data) {
        Ok(json) => println!("{}", json),
        Err(err) => panic!("JSON error: {}", err),
    };
}
