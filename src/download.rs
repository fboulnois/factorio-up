use crate::{
    error::{AppResult, NotFoundExt},
    hash, http,
};

pub static REDIRECT_URL: &str = "https://factorio.com/get-download/stable/headless/linux64";
pub static CHECKSUM_URL: &str = "https://www.factorio.com/download/sha256sums/";

pub fn parse_download_filename(url: &str) -> AppResult<String> {
    let filename = url.split('/').next_back().ok_or_not_found("no '/' found")?;
    let filename = filename.split('?').next().ok_or_not_found("no '?' found")?;
    Ok(filename.to_string())
}

pub fn fetch_checksum(filename: &str) -> AppResult<String> {
    let hashes = http::fetch(CHECKSUM_URL)?;
    let sha256 = hash::find_file_hash(filename, hashes)?;
    Ok(sha256)
}

pub fn fetch_factorio_archive(url: &str, filename: &str, hash: &str) -> AppResult<()> {
    let path = std::path::Path::new(filename);
    if !path.exists() {
        http::fetch_file(url, filename)?;
    } else {
        eprintln!("{} already exists", filename);
    }
    hash::verify_file_hash(filename, hash)?;
    Ok(())
}
