use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug, Eq, PartialEq)]
pub struct Data
{
    adco: String,
    optarif: String,
    isousc: u8,
    hchc: u32,
    hchp: u32,
    ptec: String,
    iinst: u32,
    imax: u32,
    papp: u32,
    hhphc: String,
    motdetat: String,
}

impl Data
{
    pub fn new() -> Data
    {
        Data {
            adco: String::new(),
            optarif: String::new(),
            isousc: 0,
            hchc: 0,
            hchp: 0,
            ptec: String::new(),
            iinst: 0,
            imax: 0,
            papp: 0,
            hhphc: String::new(),
            motdetat: String::new(),
        }
    }
}

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
        let mut data = Data::new();

        for line in frame.lines() {
            let mut tokens = line.split_whitespace();

            match tokens.nth(0).unwrap() {
                "ADCO" => data.adco = tokens.nth(0).unwrap().parse().unwrap(),
                "OPTARIF" => data.optarif = tokens.nth(0).unwrap().parse().unwrap(),
                "ISOUSC" => data.isousc = tokens.nth(0).unwrap().parse().unwrap(),
                "HCHC" => data.hchc = tokens.nth(0).unwrap().parse().unwrap(),
                "HCHP" => data.hchp = tokens.nth(0).unwrap().parse().unwrap(),
                "PTEC" => data.ptec = tokens.nth(0).unwrap().parse().unwrap(),
                "IINST" => data.iinst = tokens.nth(0).unwrap().parse().unwrap(),
                "IMAX" => data.imax = tokens.nth(0).unwrap().parse().unwrap(),
                "PAPP" => data.papp = tokens.nth(0).unwrap().parse().unwrap(),
                "HHPHC" => data.hhphc = tokens.nth(0).unwrap().parse().unwrap(),
                "MOTDETAT" => data.motdetat = tokens.nth(0).unwrap().parse().unwrap(),
                _ => println!("Invalid field"),
            };
        }

        data
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

#[test]
fn parse()
{
    let frame = String::from("ADCO 130622778433 D
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

    let parser = Parser::new();

    assert_eq!(parser.parse(frame), Data {
        adco: String::from("130622778433"),
        optarif: String::from("HC.."),
        isousc: 45,
        hchc: 41478078,
        hchp: 68619587,
        ptec: String::from("HP.."),
        iinst: 2,
        imax: 39,
        papp: 440,
        hhphc: String::from("D"),
        motdetat: String::from("000000"),
    });
}
