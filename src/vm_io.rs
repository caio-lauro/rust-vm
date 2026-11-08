use std::io::{self, Read, StdoutLock, Write};

pub fn get_char() -> u16 {
    let mut stdin_handle = io::stdin().lock();
    let mut byte = [0_u8];
    stdin_handle
        .read_exact(&mut byte)
        .expect("failed to read stdin");
    byte[0] as u16
}

pub fn putc(ascii_char: u8, stdout_handle: &mut StdoutLock) {
    stdout_handle
        .write_all(&[ascii_char])
        .expect("failed to write to stdout");
}

pub fn puts(buf: &[u8], stdout_handle: &mut StdoutLock) {
    stdout_handle
        .write_all(buf)
        .expect("failed to write to stdout");
}

pub fn flush(stdout_handle: &mut StdoutLock) {
    stdout_handle.flush().expect("failed to flush stdout");
}
