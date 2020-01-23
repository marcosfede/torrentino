use torrentino::{Torrent};
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please pass a torrent file path as the first argument");
        exit(1);
    }
    let filename = args[1].clone();

    let torrent = Torrent::from_file(filename.as_str()).unwrap();
    println!("{:?}", torrent);

    let peer_id: [u8; 20] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
	let port = 6882u16;
    torrent.request_peers(peer_id, port).unwrap();
}
