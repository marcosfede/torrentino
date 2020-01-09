use serde;
use serde::{Deserialize};
use serde_bencode;
use std::io;
use std::fs::File;
use std::io::Read;
use serde_bytes::ByteBuf;

#[derive(Debug, Eq, PartialEq, Deserialize)]
struct Info {
    pieces: ByteBuf,
    piece_length: i64,
    length: Option<i64>,
    name: String,
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
struct Torrent {
    announce: String,
    info: Info,
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
    let mut file = File::open("/Users/fede/Downloads/debian-10.2.0-amd64-netinst.iso.torrent").unwrap();
    let mut buf: Vec<u8> = vec![];
    file.read(&mut buf).unwrap();

    match bytes_to_torrent(&buf) {
        Ok(t) => render_torrent(&t),
        Err(e) => println!("ERROR: {:?}", e),
    }
}
