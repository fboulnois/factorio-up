#![deny(unsafe_code)]

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
