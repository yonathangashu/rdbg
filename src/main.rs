mod debugger;
use env_logger::Env;
use nix::sys::signal::Signal;
use nix::sys::{
    ptrace,
    wait::{WaitStatus, waitpid},
};
use nix::unistd::Pid;
use std::path::Path;
use std::process::Command;

use crate::debugger::Debugger;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let program = Path::new("./tests/bin/sleep");
    let mut child = Command::new(program)
        .spawn()
        .expect("Failed to spawn {program}");

    let pid = Pid::from_raw(child.id().try_into().unwrap());
    log::info!("Attaching to PID: {}", pid);

    let target = Debugger::new(pid);

    target.attach()?;
    log::info!("Attached successfully");

    let remote_addr = 0x0000000000400486;
    let remote_mem = target.read_word(remote_addr)?;
    println!("Data at {remote_addr:x?}:{remote_mem}");

    log::info!("Writing INT3 to {remote_addr:x?}");
    target.write_word(remote_addr, 0xCC)?;

    let remote_mem = target.read_word(remote_addr)?;
    log::info!("Data at {remote_addr:x?} after the write:{remote_mem}");

    target.cont()?;

    match waitpid(pid, None)? {
        WaitStatus::Stopped(_, sig) => {
            println!("Process stopped with {:?}", sig);
            if sig == Signal::SIGTRAP {
                println!("Got SIGTRAP (breakpoint or single-step).");
            }
        }
        other => println!("Other wait status:{:?}", other),
    }

    for _ in 0..5 {
        target.step()?;
        waitpid(pid, None)?;
        let data = ptrace::getregs(pid)?;
        let rip = data.rip;
        println!("REG RIP: 0x{rip:x?}");
    }

    target.kill()?;
    log::info!("Killed the process successfully.");

    child.kill()?;
    Ok(())
}
