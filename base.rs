//! This is Rust that was generated by Numlang.
//! Learn rust here: https://www.rust-lang.org/
//! The source code for Numlang is here: https://github.com/Aityz/Numlang
#![allow(dead_code, unused_variables, unused_mut)]

use std::io::{Read, Write};

#[cfg(unix)]
#[repr(C)]
pub struct Termios {
    pub c_iflag: i32,
    pub c_oflag: i32,
    pub c_cflag: i32,
    pub c_lflag: i32,
}

static ECHO: i32 = 0o0000010;
static ICANON: i32 = 0x00002;
static TCSAFLUSH: i32 = 2;

#[cfg(unix)]
extern "C" {
    fn tcgetattr(fd: i32, termios: &mut Termios);
    fn tcsetattr(fd: i32, int: i32, termios: &mut Termios);
    fn read(fd: i32, char: &mut u8, amount: i32);
}

#[cfg(target_os = "linux")]
fn init() {
    let fd = std::os::fd::AsRawFd::as_raw_fd(&std::io::stdout()); // most likely 0

    unsafe {
        let mut termios: Termios = std::mem::zeroed();

        tcgetattr(fd, &mut termios);

        termios.c_lflag &= !(ECHO | ICANON); // disables ECHO and cooked mode

        tcsetattr(fd, TCSAFLUSH, &mut termios);
    }
}

#[cfg(not(target_os = "linux"))]
fn init() {}

#[cfg(target_os = "linux")]
fn close() {
    let fd = std::os::fd::AsRawFd::as_raw_fd(&std::io::stdout());

    unsafe {
        let mut termios: Termios = std::mem::zeroed();

        tcgetattr(fd, &mut termios);

        termios.c_lflag |= ECHO | ICANON; // re-enables echo

        tcsetattr(fd, TCSAFLUSH, &mut termios);
    }
}

#[cfg(not(target_os = "linux"))]
fn close() {}

#[inline(always)]
pub fn inc_ptr(ptr: &mut usize) {
    if *ptr == 29999 {
        *ptr = 0;
    } else {
        *ptr += 1;
    }
}

#[inline(always)]
pub fn dec_ptr(ptr: &mut usize) {
    if *ptr == 0 {
        *ptr = 29999;
    } else {
        *ptr -= 1;
    }
}

#[inline(always)]
pub fn inc_val(stack: &mut [u8], ptr: &usize) {
    if stack[*ptr] == 255 {
        stack[*ptr] = 0;
    } else {
        stack[*ptr] += 1;
    }
}

#[inline(always)]
pub fn dec_val(stack: &mut [u8], ptr: &usize) {
    if stack[*ptr] == 0 {
        stack[*ptr] = 255;
    } else {
        stack[*ptr] -= 1;
    }
}

#[inline(always)]
pub fn read_byte(stack: &mut [u8], ptr: &usize) {
    let mut buf: [u8; 1] = [0u8];

    std::io::stdin().read_exact(&mut buf).unwrap();

    stack[*ptr] = *buf.first().unwrap_or(&0u8);
}

#[inline(always)]
pub fn print_byte(stack: &mut [u8], ptr: &usize) {
    print!("{}", stack[*ptr] as char);

    std::io::stdout().flush().unwrap();
}

#[inline(always)]
pub fn print_user_stack(user_stack: &mut [u8]) {
    for val in user_stack {
        print!("{}", *val as char);
    }

    std::io::stdout().flush().unwrap();
}

#[inline(always)]
pub fn append_to_us(stack: &mut [u8], ptr: &usize, us: &mut Vec<u8>) {
    us.push(stack[*ptr]);
}

#[inline(always)]
pub fn remove_newest_from_us(user_stack: &mut Vec<u8>) {
    user_stack.pop();
}

#[inline(always)]
pub fn clear_stack(user_stack: &mut Vec<u8>) {
    user_stack.clear();
}

#[inline(always)]
pub fn write_len(stack: &mut [u8], ptr: &usize, user_stack: &mut [u8]) {
    stack[*ptr] = user_stack.len() as u8;
}

fn main() {
    init();

    let mut stack: Vec<u8> = vec![0; 30000];

    let mut ptr: usize = 0;

    let mut user_stack: Vec<u8> = Vec::new();
}
