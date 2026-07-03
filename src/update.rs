use tar::Archive;
use xz2::read::XzDecoder;

use crate::{
    args::Args,
    download::{fetch_factorio_archive, resolve_download},
    error::{AppResult, NotFoundExt},
    exec, init,
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

fn symlink_exe_and_data(args: &Args, output_dir: &std::path::Path) -> AppResult<()> {
    if let Some(dir) = args.exe_path() {
        let orig = output_dir.join("bin/x64/factorio");
        let link = std::path::Path::new(dir);
        std::os::unix::fs::symlink(orig, link)?;
    }
    if let Some(dir) = args.data_dir() {
        let orig = output_dir.join("data");
        let link = std::path::Path::new(dir);
        std::os::unix::fs::symlink(orig, link)?;
    }
    Ok(())
}

pub fn run(args: &Args) -> AppResult<()> {
    let download = resolve_download()?;
    let archive = fetch_factorio_archive(&download)?;
    let output_dir = extract_tar_xz(&archive)?;
    chown_output_dir(args, &output_dir)?;
    println!(
        "{} extracted to {}",
        download.filename,
        output_dir.display()
    );
    symlink_exe_and_data(args, &output_dir)?;
    if args.init_map() {
        init::init_map_settings(args, &output_dir)?;
    }
    exec::execute_user_command(args)?;
    Ok(())
}
