#![no_std]

pub use libwildnix_macros::main;

pub const SYS_DEBUG: u64 = 0;
pub const SYS_WRITE: u64 = 1;
pub const SYS_READ_KEY: u64 = 2;
pub const SYS_EXIT: u64 = 3;

pub const SYS_FS_READ: u64 = 10;
pub const SYS_FS_WRITE: u64 = 11;
pub const SYS_FS_CREATE: u64 = 12;
pub const SYS_FS_DELETE: u64 = 13;
pub const SYS_FS_LIST: u64 = 14;
pub const SYS_FS_EXISTS: u64 = 15;

pub const ERR: u64 = u64::MAX;

#[inline(always)]
pub unsafe fn syscall0(num: u64) -> u64 {
    let ret: u64;

    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") num => ret,
            lateout("rcx") _,
            lateout("r11") _,
            lateout("rdx") _,
            lateout("r8") _,
            lateout("r9") _,
            lateout("r10") _,
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
            lateout("rdx") _,
            lateout("r8") _,
            lateout("r9") _,
            lateout("r10") _,
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
            lateout("rdx") _,
            lateout("r8") _,
            lateout("r9") _,
            lateout("r10") _,
        );
    }

    ret
}

#[inline(always)]
pub unsafe fn syscall3(
    num: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
) -> u64 {
    let ret: u64;

    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") num => ret,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            lateout("rcx") _,
            lateout("r11") _,
            lateout("r8") _,
            lateout("r9") _,
            lateout("r10") _,
        );
    }

    ret
}

#[inline(always)]
pub unsafe fn syscall4(
    num: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
) -> u64 {
    let ret: u64;

    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") num => ret,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            in("r10") arg4,
            lateout("rcx") _,
            lateout("r11") _,
            lateout("r8") _,
            lateout("r9") _,
        );
    }

    ret
}

#[inline(always)]
pub unsafe fn syscall1_noreturn(num: u64, arg1: u64) -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") num,
            in("rdi") arg1,
            options(noreturn),
        );
    }
}

pub fn debug() {
    unsafe {
        syscall0(SYS_DEBUG);
    }
}

pub fn write(bytes: &[u8]) {
    unsafe {
        syscall2(
            SYS_WRITE,
            bytes.as_ptr() as u64,
            bytes.len() as u64,
        );
    }
}

pub fn read_key() -> u8 {
    unsafe { syscall0(SYS_READ_KEY) as u8 }
}

pub fn read_line(buffer: &mut [u8]) -> usize {
    let mut len = 0;

    loop {
        let c = read_key();

        if c == 0 {
            continue;
        }

        let c = if c == b'\r' { b'\n' } else { c };

        if c == b'\n' {
            write(b"\n");
            break;
        }

        if c == 8 || c == 127 {
            if len > 0 {
                len -= 1;
                write(b"\x08 \x08");
            }

            continue;
        }

        if len < buffer.len() {
            buffer[len] = c;
            len += 1;
            write(&[c]);
        }
    }

    len
}

pub fn exit(code: u64) -> ! {
    unsafe {
        syscall1_noreturn(SYS_EXIT, code);
    }
}

pub fn fs_read(path: &str, buffer: &mut [u8]) -> Result<usize, u64> {
    let ret = unsafe {
        syscall4(
            SYS_FS_READ,
            path.as_ptr() as u64,
            path.len() as u64,
            buffer.as_mut_ptr() as u64,
            buffer.len() as u64,
        )
    };

    if ret == ERR {
        Err(ret)
    } else {
        Ok(ret as usize)
    }
}

pub fn fs_write(path: &str, data: &[u8]) -> Result<usize, u64> {
    let ret = unsafe {
        syscall4(
            SYS_FS_WRITE,
            path.as_ptr() as u64,
            path.len() as u64,
            data.as_ptr() as u64,
            data.len() as u64,
        )
    };

    if ret == ERR {
        Err(ret)
    } else {
        Ok(ret as usize)
    }
}

pub fn fs_create(path: &str) -> Result<(), u64> {
    let ret = unsafe {
        syscall2(
            SYS_FS_CREATE,
            path.as_ptr() as u64,
            path.len() as u64,
        )
    };

    if ret == ERR {
        Err(ret)
    } else {
        Ok(())
    }
}

pub fn fs_delete(path: &str) -> Result<(), u64> {
    let ret = unsafe {
        syscall2(
            SYS_FS_DELETE,
            path.as_ptr() as u64,
            path.len() as u64,
        )
    };

    if ret == ERR {
        Err(ret)
    } else {
        Ok(())
    }
}

pub fn fs_exists(path: &str) -> bool {
    unsafe {
        syscall2(
            SYS_FS_EXISTS,
            path.as_ptr() as u64,
            path.len() as u64,
        ) == 1
    }
}

pub fn fs_list(path: &str, buffer: &mut [u8]) -> Result<usize, u64> {
    let ret = unsafe {
        syscall4(
            SYS_FS_LIST,
            path.as_ptr() as u64,
            path.len() as u64,
            buffer.as_mut_ptr() as u64,
            buffer.len() as u64,
        )
    };

    if ret == ERR {
        Err(ret)
    } else {
        Ok(ret as usize)
    }
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
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    write(b"panic\n");
    exit(1);
}