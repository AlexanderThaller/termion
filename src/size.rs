use std::io;

#[cfg(not(target_os = "redox"))]
use libc::c_ushort;

#[cfg(not(target_os = "redox"))]
#[repr(C)]
struct TermSize {
    row: c_ushort,
    col: c_ushort,
    _x: c_ushort,
    _y: c_ushort,
}

// Since attributes on non-item statements is not stable yet, we use a function.
#[cfg(not(target_os = "android"))]
#[cfg(not(target_os = "redox"))]
#[cfg(target_pointer_width = "64")]
#[cfg(not(target_env = "musl"))]
fn tiocgwinsz() -> u64 {
    use termios::TIOCGWINSZ;
    TIOCGWINSZ as u64
}
#[cfg(not(target_os = "android"))]
#[cfg(not(target_os = "redox"))]
#[cfg(target_pointer_width = "32")]
#[cfg(not(target_env = "musl"))]
fn tiocgwinsz() -> u32 {
    use termios::TIOCGWINSZ;
    TIOCGWINSZ as u32
}

#[cfg(any(target_env = "musl", target_os = "android"))]
fn tiocgwinsz() -> i32 {
    use termios::TIOCGWINSZ;
    TIOCGWINSZ as i32
}

/// Get the size of the terminal.
#[cfg(not(target_os = "redox"))]
pub fn terminal_size() -> io::Result<(u16, u16)> {
    use libc::ioctl;
    use libc::STDOUT_FILENO;

    use std::mem;
    unsafe {
        let mut size: TermSize = mem::zeroed();

        if ioctl(STDOUT_FILENO, tiocgwinsz(), &mut size as *mut _) == 0 {
            Ok((size.col as u16, size.row as u16))
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Unable to get the terminal size."))
        }
    }
}

/// Get the size of the terminal.
#[cfg(target_os = "redox")]
pub fn terminal_size() -> io::Result<(u16, u16)> {
    use redox_termios;
    use syscall;

    if let Ok(fd) = syscall::dup(1, b"winsize") {
        let mut winsize = redox_termios::Winsize::default();
        let res = syscall::read(fd, &mut winsize);
        let _ = syscall::close(fd);
        if let Ok(count) = res {
            if count == winsize.len() {
                return Ok((winsize.ws_col, winsize.ws_row));
            }
        }
    }

    Err(io::Error::new(io::ErrorKind::Other, "Unable to get the terminal size."))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_size() {
        assert!(terminal_size().is_ok());
    }
}
