/*
 *  Module made with help of AI:
 *  non-blocking io
 */

use std::{mem::MaybeUninit, os::fd::RawFd, process};

pub struct TerminalModeGuard;

impl TerminalModeGuard {
    pub fn new() -> Self {
        setup_signal_handler();
        disable_input_buffering();
        TerminalModeGuard
    }
}

impl Drop for TerminalModeGuard {
    fn drop(&mut self) {
        restore_input_buffering();
    }
}

const STDIN_FILENO: RawFd = 0;
const SIGINT: i32 = 2;

// Replicating termios structure
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Termios {
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: u32,
    pub c_line: u8,
    pub c_cc: [u8; 32],
    pub c_ispeed: u32,
    pub c_ospeed: u32,
}

const ICANON: u32 = 0x00000002;
const ECHO: u32 = 0x00000008;
const TCSANOW: i32 = 0;

unsafe extern "C" {
    fn tcgetattr(fd: RawFd, termios_p: *mut Termios) -> i32;
    fn tcsetattr(fd: RawFd, optional_actions: i32, termios_p: *const Termios) -> i32;
    fn select(
        nfds: i32,
        readfds: *mut FdSet,
        writefds: *mut std::ffi::c_void,
        exceptfds: *mut std::ffi::c_void,
        timeout: *mut TimeVal,
    ) -> i32;
    fn signal(sig: i32, handler: extern "C" fn(i32)) -> *mut std::ffi::c_void;
}

#[repr(C)]
pub struct TimeVal {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
pub struct FdSet {
    fds_bits: [u64; 16], // 1024 bits / 64 bits = 16 u64
}

impl FdSet {
    pub fn new() -> Self {
        Self { fds_bits: [0; 16] }
    }

    pub fn set(&mut self, fd: RawFd) {
        let fd = fd as usize;
        self.fds_bits[fd / 64] |= 1 << (fd % 64);
    }
}

static mut ORIGINAL_TERMIOS: MaybeUninit<Termios> = MaybeUninit::uninit();

extern "C" fn handle_interrupt(_sig: i32) {
    restore_input_buffering();
    println!();
    process::exit(-2);
}

pub fn setup_signal_handler() {
    unsafe {
        signal(SIGINT, handle_interrupt);
    }
}

pub fn disable_input_buffering() {
    unsafe {
        let mut termios = MaybeUninit::uninit();
        if tcgetattr(STDIN_FILENO, termios.as_mut_ptr()) == 0 {
            let mut new_termios = termios.assume_init();

            let static_ptr = std::ptr::addr_of_mut!(ORIGINAL_TERMIOS);
            *static_ptr = MaybeUninit::new(new_termios);

            // Deactivate ICANON and ECHO
            new_termios.c_lflag &= !(ICANON | ECHO);
            tcsetattr(STDIN_FILENO, TCSANOW, &new_termios);
        }
    }
}

pub fn restore_input_buffering() {
    unsafe {
        let ptr = std::ptr::addr_of!(ORIGINAL_TERMIOS);
        tcsetattr(STDIN_FILENO, TCSANOW, (*ptr).as_ptr());
    }
}

pub fn check_key() -> bool {
    unsafe {
        let mut readfds = FdSet::new();
        readfds.set(STDIN_FILENO);

        let mut timeout = TimeVal {
            tv_sec: 0,
            tv_usec: 0,
        };

        // nfds must be STDIN_FILENO + 1 (1)
        select(
            STDIN_FILENO + 1,
            &mut readfds as *mut _,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut timeout as *mut _,
        ) > 0
    }
}
