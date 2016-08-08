extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;

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
        Ok(d) => d,
        Err(e) => e.exit(),
    };

    let args: Args = match docopt.decode() {
        Ok(args) => args,
        Err(e) => e.exit(),
    };

    let parser = teleinfo::Parser::new();
    let frame = parser.read_frame(args.arg_device);
    let data = parser.parse(frame);

    println!("{:?}", data);
}
