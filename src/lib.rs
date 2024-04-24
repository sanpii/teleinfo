#![warn(warnings)]

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Invalid field: {0}")]
    InvalidField(String),
}

#[derive(Debug, Default, Eq, PartialEq, serde::Serialize)]
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
        Self::default()
    }
}

#[derive(Clone, Default)]
pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Self::default()
    }

    pub fn read_frame<P: AsRef<std::path::Path>>(&self, path: P) -> Result<String> {
        use std::io::BufRead;

        let file = std::fs::File::open(&path)?;

        let mut buffer = std::io::BufReader::new(&file);
        let mut line: Vec<u8> = Vec::new();

        buffer.read_until(0x2, &mut line)?;

        buffer.consume(1);
        line = Vec::new();

        buffer.read_until(0x3, &mut line)?;

        String::from_utf8(line)
            .map(|s| s.trim_end_matches('\u{3}').replace('\r', ""))
            .map_err(Error::from)
    }

    pub fn parse(self, frame: String) -> Result<Data> {
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
                _ => return Err(Error::InvalidField(key)),
            };
        }

        Ok(data)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn read_frame() -> super::Result {
        let parser = super::Parser::new();

        let frame = parser.read_frame(String::from("./teleinfo.txt"))?;

        assert_eq!(
            frame,
            "ADCO 130622778433 D
OPTARIF HC.. <
ISOUSC 45 ?
HCHC 041478078 -
HCHP 068619587 E
PTEC HP..\u{20}\u{20}
IINST 002 Y
IMAX 039 K
PAPP 00440 )
HHPHC D /
MOTDETAT 000000 B"
        );

        Ok(())
    }

    #[test]
    fn parse() -> super::Result {
        let parser = super::Parser::new();

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

        let data = parser.parse(frame)?;

        assert_eq!(
            data,
            super::Data {
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

        Ok(())
    }
}
