use std::os::unix::process::CommandExt;

use crate::{
    args::Args,
    error::{AppResult, InvalidData, NotFound},
};

pub fn init_map_settings(args: &Args, output_dir: &std::path::Path) -> AppResult<()> {
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
    print!("{}", String::from_utf8_lossy(&out.stdout));
    let stderr = String::from_utf8_lossy(&out.stderr);
    if !stderr.is_empty() {
        eprint!("{stderr}");
    }
    if !out.status.success() {
        let error = format!(
            "failed to initialize map settings: factorio exited with {}",
            out.status
        );
        return Err(InvalidData::new(&error).into());
    }
    Ok(())
}
