use std::os::unix::process::CommandExt;

use crate::{args::Args, error::AppResult};

pub fn execute_user_command(args: &Args) -> AppResult<()> {
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
