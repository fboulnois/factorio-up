use std::{
    io::{Read, Write},
    sync::LazyLock,
};

use ureq::{Agent, ResponseExt};

use crate::error::AppResult;

static UREQ: LazyLock<Agent> = LazyLock::new(|| Agent::config_builder().build().into());

pub fn get_redirect_url(url: &str) -> AppResult<String> {
    let response = UREQ.get(url).call()?;
    Ok(response.get_uri().to_string())
}

pub fn fetch(url: &str) -> AppResult<Vec<u8>> {
    let mut response = UREQ.get(url).call()?;
    let mut bytes = Vec::new();
    response.body_mut().as_reader().read_to_end(&mut bytes)?;
    Ok(bytes)
}

pub fn fetch_file(url: &str, filename: &str) -> AppResult<()> {
    let mut file = std::fs::File::create(filename)?;
    let mut response = UREQ.get(url).call()?;
    let mut reader = response.body_mut().as_reader();
    let mut buffer = [0; 1024 * 1024];
    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])?;
    }
    file.flush()?;
    Ok(())
}
