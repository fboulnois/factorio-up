use std::fmt::Write;

use ring::digest::{Context, Digest, SHA256};

use crate::error::{AppResult, InvalidData, NotFoundExt};

pub fn find_file_hash(filename: &str, hashes: Vec<u8>) -> AppResult<String> {
    let mut hash = None;
    let data = String::from_utf8(hashes)
        .map_err(|_| InvalidData::new("failed to parse checksums as utf-8"))?;
    for line in data.lines() {
        if line.ends_with(filename) {
            hash = line.split_whitespace().next();
            break;
        }
    }
    Ok(hash.ok_or_not_found("no matching hash found")?.to_string())
}

impl DisplayChecksum for Digest {
    fn checksum(&self) -> String {
        let mut hash = String::with_capacity(2 * self.as_ref().len());
        for hex in self.as_ref() {
            write!(hash, "{hex:02x}").ok();
        }
        hash
    }
}

trait DisplayChecksum {
    fn checksum(&self) -> String;
}

fn sha256_digest<R: std::io::Read>(mut reader: R) -> AppResult<String> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024 * 1024];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    let digest = context.finish();
    Ok(digest.checksum())
}

pub fn sha256_digest_file(path: &str) -> AppResult<String> {
    let file = std::fs::File::open(path)?;
    sha256_digest(file)
}

pub fn verify_file_hash(filename: &str, hash: &str) -> AppResult<()> {
    let sha256 = sha256_digest_file(filename)?;
    if hash != sha256 {
        let error = InvalidData::new("file hash does not match");
        return Err(error.into());
    }
    Ok(())
}
