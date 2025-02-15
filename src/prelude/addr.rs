use std::ops::{Add, AddAssign};

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Addr {
    pub addr: u64,
    pub vaddr: u64,
}

impl Add<u64> for Addr {
    type Output = Self;

    fn add(self, other: u64) -> Self {
        Self {
            addr: self.addr + other,
            vaddr: self.vaddr + other,
        }
    }
}

impl Add<u32> for Addr {
    type Output = Self;

    fn add(self, other: u32) -> Self {
        self.add(other as u64)
    }
}

impl AddAssign<u64> for Addr {
    fn add_assign(&mut self, other: u64) { 
        self.addr += other;
        self.vaddr += other;
    }
}

impl AddAssign<u32> for Addr {
    fn add_assign(&mut self, other: u32) { 
        self.add_assign(other as u64);
    }
}
