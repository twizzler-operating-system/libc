// libc port for Twizzler

cfg_if! {
    if #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))] {
        pub type c_char = u8;
    } else {
        pub type c_char = i8;
    }
}

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i64;
pub type c_ulong = u64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;
pub type intptr_t = isize;
pub type uintptr_t = usize;

pub type c_float = f32;
pub type c_double = f64;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type ssize_t = isize;

pub type wint_t = u32;
pub type wctype_t = i64;

pub type regoff_t = size_t;
pub type off_t = c_long;

pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;
pub const STDERR_FILENO: c_int = 2;

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

pub use ffi::c_void;
