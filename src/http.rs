use std::{
    io::{Read, Write},
    sync::LazyLock,
    time::Duration,
};

use ureq::{Agent, ResponseExt};

use crate::error::AppResult;

static UREQ: LazyLock<Agent> = LazyLock::new(|| {
    Agent::config_builder()
        .timeout_connect(Some(Duration::from_secs(30)))
        .timeout_recv_response(Some(Duration::from_secs(30)))
        .build()
        .into()
});

pub fn get_resolved_url(url: &str) -> AppResult<String> {
    let response = UREQ.get(url).call()?;
    Ok(response.get_uri().to_string())
}

pub fn fetch_bytes(url: &str) -> AppResult<Vec<u8>> {
    let mut response = UREQ.get(url).call()?;
    let mut bytes = Vec::new();
    response.body_mut().as_reader().read_to_end(&mut bytes)?;
    Ok(bytes)
}

pub fn fetch_file(url: &str, path: impl AsRef<std::path::Path>) -> AppResult<()> {
    let mut file = std::fs::File::create(path)?;
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
