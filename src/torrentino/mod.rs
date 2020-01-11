use serde;
use serde::{Deserialize};
use serde_bencode;
use serde_bytes::ByteBuf;
use std::fs::read;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Deserialize)]
struct Info {
    pieces: ByteBuf,
    #[serde(rename="piece length")]
    piece_length: i64,
    #[serde(default)]
    length: Option<i64>,
    name: String,
}

#[derive(Eq, PartialEq, Deserialize)]
pub struct Torrent {
    /// adasdasdasd  asdasd
    #[serde(default)]
    announce: String,
    info: Info,
    #[serde(rename="comment")]
    comment: Option<String>,
}

impl Torrent {
    pub fn from_file(filename: &str) -> Result<Torrent, serde_bencode::Error> {
        let bytes = read(filename).unwrap();
        serde_bencode::de::from_bytes(&bytes)
    }
}

impl fmt::Debug for Torrent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "name:\t\t{}", self.info.name)?;
        writeln!(f, "announce:\t{:?}", self.announce)?;
        writeln!(f, "piece length:\t{:?}", self.info.piece_length)?;
        Ok(())
    }
}
