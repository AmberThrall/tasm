pub mod elf;
mod utils;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Endianness {
    Little,
    Big,
}
