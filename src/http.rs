use curl::easy::Easy;
use std::{
    io::Write,
    sync::{Arc, LazyLock, Mutex},
};

use crate::error::{AppResult, NotFoundExt};

static CURL: LazyLock<Arc<Mutex<Easy>>> = LazyLock::new(|| Arc::new(Mutex::new(Easy::new())));

pub fn get_redirect_url(url: &str) -> AppResult<String> {
    let mut curl = CURL.lock().unwrap();
    curl.url(url)?;
    curl.perform()?;
    Ok(curl
        .redirect_url()?
        .ok_or_not_found("no redirect url found")?
        .to_string())
}

pub fn fetch(url: &str) -> AppResult<Vec<u8>> {
    let mut curl = CURL.lock().unwrap();
    let mut hashes = Vec::new();
    curl.url(url)?;
    {
        let mut transfer = curl.transfer();
        transfer.write_function(|chunk| {
            hashes.extend_from_slice(chunk);
            Ok(chunk.len())
        })?;
        transfer.perform()?;
    };
    Ok(hashes)
}

pub fn fetch_file(url: &str, filename: &str) -> AppResult<()> {
    let mut curl = CURL.lock().unwrap();
    let mut file = std::fs::File::create(filename)?;
    curl.url(url)?;
    {
        curl.write_function(move |data| {
            file.write_all(data).unwrap();
            Ok(data.len())
        })?;
        curl.perform()?;
    };
    Ok(())
}
