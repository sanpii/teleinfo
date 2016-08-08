mod teleinfo;

fn main()
{
    let parser = teleinfo::Parser::new();
    let frame = parser.read_frame(String::from("teleinfo.txt"));
    let data = parser.parse(frame);

    println!("{:?}", data);
}
