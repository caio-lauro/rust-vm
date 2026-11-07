use std::unreachable;

use crate::hardware::Register::CondFlags;

#[derive(Debug)]
pub struct Memory {
    memory: Vec<u16>,
    size: usize,
    capacity: usize,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            memory: Vec::from([0; 1 << 16]),
            size: 0,
            capacity: 1 << 16,
        }
    }

    pub fn read(&self, address: u16) -> u16 {
        todo!();
    }

    pub fn write(&self, address: u16, value: u16) {
        todo!();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Register {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    CondFlags,
    Count,
}

impl From<u16> for Register {
    fn from(value: u16) -> Self {
        match value.into() {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::R6,
            7 => Register::R7,
            8 => Register::PC,
            9 => Register::CondFlags,
            _ => panic!("value too big to convert to register"),
        }
    }
}

impl Register {
    pub fn usize(self) -> usize {
        match self {
            Register::R0 => 0,
            Register::R1 => 1,
            Register::R2 => 2,
            Register::R3 => 3,
            Register::R4 => 4,
            Register::R5 => 5,
            Register::R6 => 6,
            Register::R7 => 7,
            Register::PC => 8,
            Register::CondFlags => 9,
            _ => unreachable!(),
        }
    }

    pub fn u16(self) -> u16 {
        Self::usize(self) as u16
    }
}

pub static PC_START: u16 = 0x3000;

#[derive(Debug)]
pub struct Registers {
    registers: Vec<u16>,
}

impl Registers {
    pub fn new() -> Self {
        let mut registers = Vec::from([0; Register::Count as usize]);

        registers[Register::CondFlags.usize()] = super::Flags::Zro.value();

        registers[Register::PC.usize()] = PC_START;

        Self { registers }
    }

    pub fn increment_pc(&mut self) -> u16 {
        self.increment_pc_by(1)
    }

    pub fn increment_pc_by(&mut self, value: u16) -> u16 {
        self.registers[Register::PC.usize()] += value;
        self.registers[Register::PC.usize()]
    }

    pub fn get(&self, register: u16) -> u16 {
        self.registers[register as usize]
    }

    pub fn cond(&self) -> u16 {
        self.registers[Register::CondFlags.usize()]
    }

    pub fn update(&mut self, register: u16, value: u16) {
        self.registers[register as usize] = value;
        self.update_flags(register);
    }

    fn update_flags(&mut self, register: u16) {
        use super::Flags::*;

        let reg = self.registers[register as usize];
        let cond_reg = &mut self.registers[Register::CondFlags.usize()];
        if reg == 0 {
            *cond_reg = Zro.value();
        } else if (reg >> 15) == 1 {
            *cond_reg = Neg.value();
        } else {
            *cond_reg = Pos.value();
        }
    }
}
