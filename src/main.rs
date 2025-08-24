#![deny(unsafe_code)]
#![doc = include_str!("../README.md")]

mod args;
mod error;
mod hash;
mod http;
mod update;
mod user;

fn main() -> error::AppResult<()> {
    let args = args::Args::new();
    update::run(&args)
}
