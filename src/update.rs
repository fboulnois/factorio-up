use tar::Archive;
use xz2::read::XzDecoder;

use crate::{
    args::Args,
    download::{fetch_factorio_archive, resolve_download},
    error::{AppResult, NotFoundExt},
};

fn extract_tar_xz(filename: &std::path::Path) -> AppResult<std::path::PathBuf> {
    let version_dir = filename
        .parent()
        .and_then(std::path::Path::parent)
        .ok_or_not_found("archive path not found")?;

    let extract_dir = version_dir.join("factorio");

    let file = std::fs::File::open(filename)?;
    let xz = XzDecoder::new(file);
    let mut archive = Archive::new(xz);
    archive.unpack(version_dir)?;

    Ok(extract_dir)
}

fn chown_output_dir(args: &Args, output_dir: &std::path::Path) -> AppResult<()> {
    if let Some(user) = args.user() {
        let uid = Some(user.uid());
        let gid = Some(user.gid());
        std::os::unix::fs::chown(output_dir, uid, gid)?;
    }
    Ok(())
}

pub fn run(args: &Args) -> AppResult<std::path::PathBuf> {
    let download = resolve_download()?;
    let archive = fetch_factorio_archive(&download)?;
    let output_dir = extract_tar_xz(&archive)?;
    chown_output_dir(args, &output_dir)?;
    println!(
        "{} extracted to {}",
        download.filename,
        output_dir.display()
    );
    Ok(output_dir)
}
