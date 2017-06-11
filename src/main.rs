extern crate docopt;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use docopt::Docopt;

mod teleinfo;

static USAGE: &'static str = "Usage: teleinfo <device>";

#[derive(Deserialize)]
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

    let args: Args = match docopt.deserialize() {
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

    match serde_json::to_string(&data) {
        Ok(json) => println!("{}", json),
        Err(err) => panic!("JSON error: {}", err),
    };
}
