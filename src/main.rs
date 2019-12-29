extern crate structopt;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use structopt::StructOpt;

mod teleinfo;

#[derive(StructOpt)]
struct Opt
{
    device: String,
}

fn main()
{
    let opt = Opt::from_args();

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
