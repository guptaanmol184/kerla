use crate::fs::path::Path;
use crate::{arch::UserVAddr, result::Result};
use crate::{process::current_process, syscalls::SyscallHandler};

impl<'a> SyscallHandler<'a> {
    pub fn sys_stat(&mut self, path: &Path, buf: UserVAddr) -> Result<isize> {
        let stat = current_process().root_fs().lock().lookup(path)?.stat()?;
        buf.write(&stat)?;
        Ok(0)
    }
}
