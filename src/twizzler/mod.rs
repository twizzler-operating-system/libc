// libc port for Twizzler

use crate::prelude::*;

pub type intmax_t = i64;
pub type uintmax_t = u64;
pub type intptr_t = isize;
pub type uintptr_t = usize;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type ssize_t = isize;

pub type wint_t = u32;
pub type wctype_t = i64;

pub type regoff_t = size_t;
pub type off_t = c_long;

pub type pid_t = i32;
pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;
pub const STDERR_FILENO: c_int = 2;

mod error;
pub use error::*;

cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use self::aarch64::*;
    } else if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
        pub use self::x86_64::*;
    } else {
        // Unknown target_arch
    }
}
