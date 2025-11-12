use color_eyre::Result;
use nix::{
    sys::{ptrace, wait::waitpid},
    unistd::Pid,
};

pub struct Debugger {
    pid: Pid,
}

impl Debugger {
    pub fn new(pid: Pid) -> Self {
        Self { pid }
    }

    pub fn attach(&self) -> Result<()> {
        ptrace::attach(self.pid)?;
        waitpid(self.pid, None)?;
        Ok(())
    }

    pub fn cont(&self) -> Result<()> {
        ptrace::cont(self.pid, None)?;
        Ok(())
    }

    pub fn step(&self) -> Result<()> {
        ptrace::step(self.pid, None)?;
        Ok(())
    }

    pub fn kill(&self) -> Result<()> {
        ptrace::kill(self.pid)?;
        Ok(())
    }

    pub fn read_word(&self, addr: usize) -> Result<u64> {
        let data = ptrace::read(self.pid, addr as *mut _)?;
        Ok(data as u64)
    }

    pub fn write_word(&self, addr: usize, data: u64) -> Result<()> {
        ptrace::write(self.pid, addr as *mut _, data as i64)?;
        Ok(())
    }
}
