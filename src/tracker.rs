use crate::metadata::Torrent;
use crate::utils::Error;
use reqwest;
use serde::{Serialize};
use percent_encoding::{percent_encode, NON_ALPHANUMERIC, };

pub struct Peer {}

#[derive(Serialize)]
struct Params {
    info_hash: String,
    peer_id: String,
    port: u16,
    uploaded: String,
    downloaded: String,
    compact: String,
    left: i64,
}

impl Torrent {
    pub fn request_peers(&self, peer_id: [u8; 20], port: u16) -> Result<Vec<Peer>, Error> {
        let client = reqwest::blocking::Client::new();
        let params = Params {
            info_hash: percent_encode(&self.info_hash, NON_ALPHANUMERIC).collect::<String>(),
            peer_id: percent_encode(&peer_id.to_vec(), NON_ALPHANUMERIC).collect::<String>(),
            port: port,
            uploaded: "0".to_owned(),
            downloaded: "0".to_owned(),
            compact: "1".to_owned(),
            left: self.length,
        };
        let r = client.get(&self.announce).query(&params);
        let req = r.build().unwrap();
        println!("q params is: {:?}", req.url());
        let res = client.execute(req)?;
        println!("res {:?}", res);
        Ok(vec![Peer {}])
    }
}
