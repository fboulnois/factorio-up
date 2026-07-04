use crate::{
    error::{AppResult, InvalidData, NotFoundExt},
    hash, http,
};

static REDIRECT_URL_BASE: &str = "https://factorio.com/get-download";
static CHECKSUM_URL: &str = "https://www.factorio.com/download/sha256sums/";

pub struct DownloadInfo {
    pub url: String,
    pub filename: String,
    pub version: String,
    pub hash: String,
}

fn parse_download_filename(url: &str) -> AppResult<String> {
    let filename = url.split('/').next_back().ok_or_not_found("no '/' found")?;
    let filename = filename.split('?').next().ok_or_not_found("no '?' found")?;
    Ok(filename.to_string())
}

fn parse_download_version(filename: &str) -> AppResult<String> {
    let filename = filename
        .strip_suffix(".tar.xz")
        .ok_or_not_found("no '.tar.xz' suffix found")?;
    let version = filename
        .split('_')
        .next_back()
        .ok_or_not_found("no '_' found")?;
    if version.is_empty() {
        return Err(InvalidData::new("empty version found").into());
    }
    Ok(version.to_string())
}

fn fetch_checksum(filename: &str) -> AppResult<String> {
    let hashes = http::fetch_bytes(CHECKSUM_URL)?;
    let sha256 = hash::find_file_hash(filename, hashes)?;
    Ok(sha256)
}

pub fn resolve_download(experimental: bool) -> AppResult<DownloadInfo> {
    let channel = if experimental {
        "experimental"
    } else {
        "stable"
    };
    let redirect_url = format!("{REDIRECT_URL_BASE}/{channel}/headless/linux64");
    let url = http::get_resolved_url(&redirect_url)?;
    let filename = parse_download_filename(&url)?;
    let version = parse_download_version(&filename)?;
    let hash = fetch_checksum(&filename)?;
    Ok(DownloadInfo {
        url,
        filename,
        version,
        hash,
    })
}

fn cache_dir() -> AppResult<std::path::PathBuf> {
    if let Some(home) = std::env::var_os("HOME") {
        if !home.is_empty() {
            let home = std::path::PathBuf::from(home);
            if home.is_dir() {
                return Ok(home.join(".cache").join("factorio-up"));
            }
        }
    }
    Ok(std::env::current_dir()?.join(".cache").join("factorio-up"))
}

fn archive_cache_dir(download: &DownloadInfo) -> AppResult<std::path::PathBuf> {
    Ok(cache_dir()?.join(&download.version))
}

fn append_path_suffix(path: &std::path::Path, suffix: &str) -> AppResult<std::path::PathBuf> {
    let mut filename = path
        .file_name()
        .ok_or_not_found("download path has no filename")?
        .to_os_string();

    filename.push(suffix);
    Ok(path.with_file_name(filename))
}

/// Fetches the Factorio archive into the local cache and returns the verified path.
///
/// 1. Reuse an existing cached archive when the hash matches.
/// 2. Rename an invalid cached archive aside before redownloading.
/// 3. Download new archive bytes to a `.part` path.
/// 4. Verify the `.part` file's hash.
/// 5. Promote the verified `.part` file to the final archive path if the hash matches.
/// 6. Remove the `.part` file if download or verification fails.
pub fn fetch_factorio_archive(download: &DownloadInfo) -> AppResult<std::path::PathBuf> {
    let filepath = archive_cache_dir(download)?
        .join("download")
        .join(&download.filename);
    let download_dir = filepath
        .parent()
        .ok_or_not_found("download directory has no parent")?;
    std::fs::create_dir_all(download_dir)?;

    if filepath.exists() {
        if hash::verify_file_hash(&filepath, &download.hash)? {
            eprintln!("{} already exists", &filepath.display());
            return Ok(filepath);
        }

        eprintln!("{} hash mismatch; redownloading", &filepath.display());
        let invalid = append_path_suffix(&filepath, ".invalid")?;
        std::fs::rename(&filepath, &invalid)?;
    }

    let partial = append_path_suffix(&filepath, ".part")?;
    let result = (|| {
        http::fetch_file(&download.url, &partial)?;
        hash::assert_file_hash(&partial, &download.hash)?;
        std::fs::rename(&partial, &filepath)?;
        Ok(filepath)
    })();

    if result.is_err() {
        let _ = std::fs::remove_file(&partial);
    }

    result
}
