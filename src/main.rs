#![deny(unsafe_code)]
#![doc = include_str!("../README.md")]

mod args;
mod download;
mod error;
mod exec;
mod hash;
mod http;
mod init;
mod update;
mod user;

fn main() -> error::AppResult<()> {
    let args = args::Args::new();
    let extract_dir = update::run(&args)?;
    if args.init_map() {
        init::init_map_settings(&args, &extract_dir)?;
    }
    if args.has_exec() {
        exec::execute_user_command(&args, &extract_dir)?;
    }
    Ok(())
}
