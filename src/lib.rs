#![no_std]

pub use libwildnix_macros::main;

pub const SYS_DEBUG: u64 = 0;
pub const SYS_WRITE: u64 = 1;
pub const SYS_READ_KEY: u64 = 2;
pub const SYS_EXIT: u64 = 3;

#[inline(always)]
pub unsafe fn syscall0(num: u64) -> u64 {
    let ret: u64;

    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") num => ret,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack),
        );
    }

    ret
}

#[inline(always)]
pub unsafe fn syscall1(num: u64, arg1: u64) -> u64 {
    let ret: u64;

    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") num => ret,
            in("rdi") arg1,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack),
        );
    }

    ret
}

#[inline(always)]
pub unsafe fn syscall2(num: u64, arg1: u64, arg2: u64) -> u64 {
    let ret: u64;

    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") num => ret,
            in("rdi") arg1,
            in("rsi") arg2,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack),
        );
    }

    ret
}

pub fn write(bytes: &[u8]) -> u64 {
    unsafe { syscall2(SYS_WRITE, bytes.as_ptr() as u64, bytes.len() as u64) }
}

pub fn read_key() -> Option<u8> {
    let c = unsafe { syscall0(SYS_READ_KEY) };

    if c == 0 {
        None
    } else {
        Some(c as u8)
    }
}

pub fn exit(code: u64) -> ! {
    unsafe {
        syscall1(SYS_EXIT, code);
    }

    loop {}
}

pub struct Writer;

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write(s.as_bytes());
        Ok(())
    }
}

pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;

    let mut writer = Writer;
    let _ = writer.write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::_print(format_args!($($arg)*))
    }};
}

#[macro_export]
macro_rules! println {
    () => {{
        $crate::print!("\n")
    }};

    ($fmt:expr) => {{
        $crate::print!(concat!($fmt, "\n"))
    }};

    ($fmt:expr, $($arg:tt)*) => {{
        $crate::print!(
            concat!($fmt, "\n"),
            $($arg)*
        )
    }};
}

#[cfg(feature = "panic-handler")]
use core::panic::PanicInfo;

#[cfg(feature = "panic-handler")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    write(b"panic\n");

    loop {}
}
