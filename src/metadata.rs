use serde;
use serde::{Deserialize, Serialize};
use serde_bencode;
use serde_bytes::ByteBuf;
use std::fmt;
use std::fs::read;

use crate::hashing::{calc_sha1, Sha1};
use crate::utils::Error;

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct BencodeInfo {
    pub pieces: ByteBuf,
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    #[serde(default)]
    pub length: i64,
    pub name: String,
}

impl BencodeInfo {
    fn hash(&self) -> Sha1 {
        let bytes = serde_bencode::ser::to_bytes(self).unwrap();
        calc_sha1(&bytes)
    }

    fn split_piece_hashes(&self) -> Vec<Sha1> {
        const HASH_LEN: usize = 20; // Length of SHA-1 hash
        let buf: Vec<u8> = self.pieces.to_vec();
        assert_eq!(buf.len() % HASH_LEN, 0);
        let num_hashes = buf.len() / HASH_LEN;
        let mut hashes: Vec<Sha1> = Vec::with_capacity(num_hashes);
        for chunk in buf.chunks_exact(HASH_LEN) {
            hashes.push(chunk.to_vec());
        }
        for i in 0..num_hashes {
            hashes[i].clone_from_slice(&buf[i * HASH_LEN..(i + 1) * HASH_LEN]);
        }
        hashes
    }
}

#[derive(Eq, PartialEq, Deserialize)]
pub struct BencodeTorrent {
    #[serde(default)]
    pub announce: String,
    pub info: BencodeInfo,
    #[serde(rename = "comment")]
    pub comment: Option<String>,
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
    pub announce: String,
    pub info_hash: Sha1,
    pub piece_hashes: Vec<Sha1>,
    pub piece_length: i64,
    pub length: i64,
    pub name: String,
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
        writeln!(f, "info hash:\t{:?}", self.info_hash)?;
        // writeln!(f, "piece hashes:")?;
        // for (i, piece_hash) in self.piece_hashes.iter().enumerate() {
        //     writeln!(f, "piece {:?}:\t{:?}", i, piece_hash)?;
        // }
        Ok(())
    }
}
