use std::os::unix::process::CommandExt;

use crate::{args::Args, error::AppResult};

pub fn execute_user_command(args: &Args, output_dir: &std::path::Path) -> AppResult<()> {
    let exec = args.exec();
    let exe = exec.first().expect("exec command must be non-empty");
    let exe = if *exe == "factorio" {
        output_dir.join("bin/x64/factorio")
    } else {
        exe.into()
    };
    let argv = exec.into_iter().skip(1).collect::<Vec<&str>>();
    let mut cmd = std::process::Command::new(exe);
    if let Some(user) = args.user() {
        cmd.uid(user.uid());
        cmd.gid(user.gid());
    }
    let error = cmd.args(argv).exec();
    Err(error.into())
}
