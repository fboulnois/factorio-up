use std::os::unix::process::CommandExt;

use tar::Archive;
use xz2::read::XzDecoder;

use crate::{
    args::Args,
    download::{fetch_factorio_archive, resolve_download},
    error::{AppResult, NotFound, NotFoundExt},
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

fn init_map_settings(args: &Args, output_dir: &std::path::Path) -> AppResult<()> {
    if !args.init_map() {
        return Ok(());
    }
    let save_file = args.save_file();
    if std::path::Path::new(save_file).exists() {
        eprintln!("{} already exists", save_file);
        return Ok(());
    }
    let map_gen_settings = args.map_gen_settings();
    if !std::path::Path::new(map_gen_settings).exists() {
        let error = format!("{} not found", map_gen_settings);
        return Err(NotFound::new(&error).into());
    }
    let map_settings = args.map_settings();
    if !std::path::Path::new(map_settings).exists() {
        let error = format!("{} not found", map_settings);
        return Err(NotFound::new(&error).into());
    }
    let exe = output_dir.join("bin/x64/factorio");
    let argv = vec![
        "--map-gen-settings",
        map_gen_settings,
        "--map-settings",
        map_settings,
        "--create",
        save_file,
    ];
    let mut cmd = std::process::Command::new(exe);
    if let Some(user) = args.user() {
        cmd.uid(user.uid());
        cmd.gid(user.gid());
    }
    let out = cmd.args(argv).output()?;
    println!("{}", String::from_utf8_lossy(&out.stdout));
    Ok(())
}

fn execute_user_command(args: &Args) -> AppResult<()> {
    let exec = args.exec();
    if exec.is_empty() {
        return Ok(());
    }
    let exe = exec.first().unwrap().to_string();
    let argv = exec.into_iter().skip(1).collect::<Vec<&str>>();
    let mut cmd = std::process::Command::new(exe);
    if let Some(user) = args.user() {
        cmd.uid(user.uid());
        cmd.gid(user.gid());
    }
    let error = cmd.args(argv).exec();
    Err(error.into())
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
    init_map_settings(args, &output_dir)?;
    execute_user_command(args)?;
    Ok(())
}
