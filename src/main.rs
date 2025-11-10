use env_logger::Env;
use nix::sys::{ptrace, wait::waitpid};
use nix::unistd::Pid;
use std::process::Command;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut child = Command::new("./tests/bin/sleep")
        .spawn()
        .expect("Failed to spawn sleep");

    let pid = Pid::from_raw(child.id().try_into().unwrap());

    log::info!("Attaching to PID: {}", pid);

    ptrace::attach(pid)?;
    waitpid(pid, None)?;
    log::info!("Attached successfully");

    let data = ptrace::getregs(pid)?;
    println!("{data:#?}");

    ptrace::kill(pid)?;
    log::info!("Killed the process successfully.");

    Ok(())
}
