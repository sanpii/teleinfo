extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;
use rustc_serialize::json;

mod teleinfo;

static USAGE: &'static str = "Usage: ws2300 <device>";

#[derive(RustcDecodable)]
struct Args
{
    arg_device: String,
}

fn main()
{
    let docopt = match Docopt::new(USAGE) {
        Ok(docopt) => docopt,
        Err(err) => err.exit(),
    };

    let args: Args = match docopt.decode() {
        Ok(args) => args,
        Err(err) => err.exit(),
    };

    let parser = teleinfo::Parser::new();

    let frame = match parser.read_frame(args.arg_device) {
        Ok(frame) => frame,
        Err(err) => panic!("Unable to read device: {}", err),
    };

    let data = match parser.parse(frame) {
        Ok(data) => data,
        Err(err) => panic!("Unable to parse frame: {}", err),
    };

    match json::encode(&data) {
        Ok(json) => println!("{}", json),
        Err(err) => panic!("JSON error: {}", err),
    };
}
