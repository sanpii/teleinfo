use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub struct Parser
{
}

impl Parser
{
    pub fn new() -> Parser
    {
        Parser {}
    }

    pub fn read_frame(&self, path: String) -> String
    {
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => panic!("Unable to open {}: {}", path, err),
        };

        let mut buffer = BufReader::new(&file);
        let mut line: Vec<u8> = vec![];

        buffer.read_until(0x2, &mut line);
        buffer.consume(1);
        line = vec![];

        buffer.read_until(0x3, &mut line);

        String::from_utf8(line)
            .unwrap()
            .trim_right_matches("\u{3}")
            .replace("\r", "")
    }

    pub fn parse(self: Self, frame: String) -> Data
    {
        Data {}
    }
}

#[test]
fn read_frame()
{
    let parser = Parser::new();

    assert_eq!(parser.read_frame(&Path::new("./teleinfo.txt")), "ADCO 130622778433 D
OPTARIF HC.. <
ISOUSC 45 ?
HCHC 041478078 -
HCHP 068619587 E
PTEC HP..  
IINST 002 Y
IMAX 039 K
PAPP 00440 )
HHPHC D /
MOTDETAT 000000 B");
}
