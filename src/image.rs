use std::fs::File;
use std::io::{BufReader, Read};

use crate::hardware::Memory;

pub fn read(path: &str, memory: &mut Memory) -> bool {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let mut reader = BufReader::new(file);

    let mut origin = [0_u8; 2];
    if reader.read_exact(&mut origin).is_err() {
        return false;
    }

    let mut addr = u16::from_be_bytes(origin);
    let mut word_bytes = [0_u8; 2];
    while reader.read_exact(&mut word_bytes).is_ok() {
        let word = u16::from_be_bytes(word_bytes);
        memory.write(addr, word);

        if addr == u16::MAX {
            break;
        }

        addr += 1;
    }

    true
}
