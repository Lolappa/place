use std::num::TryFromIntError;

use syscalls::{raw_syscall, Sysno};
use users::uid_t;

pub fn get_current_uid() -> uid_t {
    unsafe { raw_syscall!(Sysno::getuid).try_into().unwrap() }
}
