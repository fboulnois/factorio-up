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
    update::run(&args)
}
