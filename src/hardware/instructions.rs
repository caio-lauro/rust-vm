#[derive(Debug, Clone, Copy)]
pub enum Opcodes {
    Branch = 0,
    Add,
    Load,
    Store,
    JumpRegister,
    And,
    LoadRegister,
    StoreRegister,
    ReturnFromInterrupt,
    Not,
    LoadIndirect,
    StoreIndirect,
    Jump,
    Reserved,
    LoadEffectiveAddress,
    Trap,
}

impl From<u16> for Opcodes {
    fn from(value: u16) -> Self {
        use Opcodes::*;
        match value {
            0 => Branch,
            1 => Add,
            2 => Load,
            3 => Store,
            4 => JumpRegister,
            5 => And,
            6 => LoadRegister,
            7 => StoreRegister,
            8 => ReturnFromInterrupt,
            9 => Not,
            10 => LoadIndirect,
            11 => StoreIndirect,
            12 => Jump,
            13 => Reserved,
            14 => LoadEffectiveAddress,
            15 => Trap,
            _ => panic!("value too big to convert to an opcode"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TrapVector {
    GetChar = 0x20,
    Out = 0x21,
    Puts = 0x22,
    In = 0x23,
    Putsp = 0x24,
    Halt = 0x25,
}

impl From<u16> for TrapVector {
    fn from(value: u16) -> Self {
        use TrapVector::*;
        match value {
            0x20 => GetChar,
            0x21 => Out,
            0x22 => Puts,
            0x23 => In,
            0x24 => Putsp,
            0x25 => Halt,
            _ => panic!("value cannot be converted to trap vector"),
        }
    }
}
