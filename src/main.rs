use serde;
use serde::{Deserialize};
use serde_bencode;
use std::fs::read;
use serde_bytes::ByteBuf;

#[derive(Debug, Eq, PartialEq, Deserialize)]
struct Info {
    pieces: ByteBuf,
    #[serde(rename="piece length")]
    piece_length: i64,
    #[serde(default)]
    length: Option<i64>,
    name: String,
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
struct Torrent {
    #[serde(default)]
    announce: String,
    info: Info,
    #[serde(rename="comment")]
    comment: Option<String>,
}

fn render_torrent(torrent: &Torrent) {
    println!("name:\t\t{}", torrent.info.name);
    println!("announce:\t{:?}", torrent.announce);
    println!("piece length:\t{:?}", torrent.info.piece_length);
}

fn bytes_to_torrent(bytes: &Vec<u8>) -> Result<Torrent, serde_bencode::Error> {
    return serde_bencode::de::from_bytes(bytes);
}

fn main() {
    let bytes = read("/Users/fede/Downloads/debian-10.2.0-amd64-netinst.iso.torrent").unwrap();
    
    match bytes_to_torrent(&bytes) {
        Ok(t) => render_torrent(&t),
        Err(e) => println!("ERROR: {:?}", e),
    }
}
