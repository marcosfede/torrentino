use serde;
use serde::Deserialize;
use serde_bencode;
use serde_bytes::ByteBuf;
use std::fmt;
use std::fs::read;
use crate::hashing::{Sha1};

type Error = Box<dyn std::error::Error>;

#[derive(Debug, Eq, PartialEq, Deserialize)]
struct BencodeInfo {
    pieces: ByteBuf,
    #[serde(rename = "piece length")]
    piece_length: i64,
    #[serde(default)]
    length: i64,
    name: String,
}

impl BencodeInfo {
    fn hash(&self) -> Sha1 {
        [0; 20]
    }

    fn split_piece_hashes(&self) -> Vec<Sha1> {
        vec![[0; 20]]
    }
}

#[derive(Eq, PartialEq, Deserialize)]
pub struct BencodeTorrent {
    #[serde(default)]
    announce: String,
    info: BencodeInfo,
    #[serde(rename = "comment")]
    comment: Option<String>,
}

impl BencodeTorrent {
    fn from_bytes(bytes: &Vec<u8>) -> Result<BencodeTorrent, serde_bencode::Error> {
        serde_bencode::de::from_bytes(bytes)
    }

    fn to_torrent(&self) -> Torrent {
        let info_hash = self.info.hash();
        let piece_hashes = self.info.split_piece_hashes();
        Torrent {
            announce: self.announce.clone(),
            length: self.info.length,
            name: self.info.name.clone(),
            piece_length: self.info.piece_length,
            info_hash: info_hash,
            piece_hashes: piece_hashes,
        }
    }
}

impl fmt::Debug for BencodeTorrent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "name:\t\t{}", self.info.name)?;
        writeln!(f, "announce:\t{:?}", self.announce)?;
        writeln!(f, "piece length:\t{:?}", self.info.piece_length)?;
        Ok(())
    }
}

#[derive(Eq, PartialEq)]
pub struct Torrent {
    announce: String,
    info_hash: Sha1,
    piece_hashes: Vec<Sha1>,
    piece_length: i64,
    length: i64,
    name: String,
}

impl Torrent {
    pub fn from_file(filename: &str) -> Result<Torrent, Error> {
        let bytes = read(filename)?;
        let bencode_torrent = BencodeTorrent::from_bytes(&bytes)?;
        let torrent = bencode_torrent.to_torrent();
        Ok(torrent)
    }
}

impl fmt::Debug for Torrent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "name:\t\t{}", self.name)?;
        writeln!(f, "announce:\t{:?}", self.announce)?;
        writeln!(f, "piece length:\t{:?}", self.piece_length)?;
        Ok(())
    }
}
