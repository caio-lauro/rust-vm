#[derive(Debug, Clone, Copy)]
pub enum Flags {
    Pos = 1 << 0,
    Zro = 1 << 1,
    Neg = 1 << 2,
}

impl Flags {
    pub fn value(self) -> u16 {
        self as u16
    }
}
