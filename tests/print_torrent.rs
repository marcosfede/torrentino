use torrentino::Torrent;

#[test]
fn parse_torrent_file() {
    match Torrent::from_file("./debian-10.2.0-amd64-netinst.iso.torrent") {
        Ok(torrent) => println!("{:?}", torrent),
        Err(e) => println!("ERROR: {:?}", e),
    }
}
