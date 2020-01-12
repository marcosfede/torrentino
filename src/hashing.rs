use crypto;
use self::crypto::digest::Digest;

pub type Sha1 = Vec<u8>;

pub fn calc_sha1(input: &[u8]) -> Sha1 {
    let mut hasher = crypto::sha1::Sha1::new();
    hasher.input(input);

    assert_eq!(hasher.output_bytes(), 20);
    let mut buf = vec![0u8; 20];
    hasher.result(&mut buf);
    buf
}