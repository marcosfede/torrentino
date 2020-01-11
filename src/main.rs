mod torrentino;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please pass a torrent file path as the first argument");
        exit(1);
    }
    let filename = args[1].clone();

    match torrentino::Torrent::from_file(filename.as_str()) {
        Ok(torrent) => println!("{:?}", torrent),
        Err(e) => println!("ERROR: {:?}", e),
    }
}
