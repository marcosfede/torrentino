use crate::metadata::Torrent;
use crate::utils::Error;
use reqwest;
use std::collections::HashMap;

struct Peer {}
impl Torrent {
    fn request_peers(&self, peerID: [u8; 20], port: u16) -> Result<Vec<Peer>, Error> {
        let mut params = HashMap::new();
        params.insert("info_hash", self.info_hash);
        let client = reqwest::blocking::Client::new();
        let res = client.get(&self.announce).query(&params).send()?;
    }
}
