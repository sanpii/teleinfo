use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Eq, PartialEq, serde_derive::Serialize)]
pub struct Data {
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

impl Data {
    pub fn new() -> Data {
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

pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn read_frame(&self, path: String) -> Result<String, String> {
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => panic!("Unable to open {}: {}", path, err),
        };

        let mut buffer = BufReader::new(&file);
        let mut line: Vec<u8> = vec![];

        match buffer.read_until(0x2, &mut line) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string()),
        };

        buffer.consume(1);
        line = vec![];

        match buffer.read_until(0x3, &mut line) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string()),
        };

        match String::from_utf8(line) {
            Ok(s) => Ok(s.trim_end_matches('\u{3}').replace("\r", "")),
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn parse(self, frame: String) -> Result<Data, String> {
        let mut data = Data::new();

        for line in frame.lines() {
            let mut tokens = line.split_whitespace();

            let key = match tokens.next() {
                Some(key) => key.to_lowercase(),
                None => continue,
            };

            let value = match tokens.next() {
                Some(value) => value,
                None => continue,
            };

            match key.as_str() {
                "adco" => data.adco = value.parse().unwrap(),
                "optarif" => data.optarif = value.parse().unwrap(),
                "isousc" => data.isousc = value.parse().unwrap(),
                "hchc" => data.hchc = value.parse().unwrap(),
                "hchp" => data.hchp = value.parse().unwrap(),
                "ptec" => data.ptec = value.parse().unwrap(),
                "iinst" => data.iinst = value.parse().unwrap(),
                "imax" => data.imax = value.parse().unwrap(),
                "papp" => data.papp = value.parse().unwrap(),
                "hhphc" => data.hhphc = value.parse().unwrap(),
                "motdetat" => data.motdetat = value.parse().unwrap(),
                _ => return Err(format!("Invalid field: {}", key)),
            };
        }

        Ok(data)
    }
}

#[test]
fn read_frame() {
    let parser = Parser::new();

    let frame = match parser.read_frame(String::from("./teleinfo.txt")) {
        Ok(frame) => frame,
        Err(err) => panic!("{}", err),
    };

    assert_eq!(
        frame,
        "ADCO 130622778433 D
OPTARIF HC.. <
ISOUSC 45 ?
HCHC 041478078 -
HCHP 068619587 E
PTEC HP..  
IINST 002 Y
IMAX 039 K
PAPP 00440 )
HHPHC D /
MOTDETAT 000000 B"
    );
}

#[test]
fn parse() {
    let parser = Parser::new();

    let frame = String::from(
        "ADCO 130622778433 D
OPTARIF HC.. <
ISOUSC 45 ?
HCHC 041478078 -
HCHP 068619587 E
PTEC HP..
IINST 002 Y
IMAX 039 K
PAPP 00440 )
HHPHC D /
MOTDETAT 000000 B",
    );

    let data = match parser.parse(frame) {
        Ok(data) => data,
        Err(err) => panic!("{}", err),
    };

    assert_eq!(
        data,
        Data {
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
        }
    );
}
