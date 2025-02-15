use super::Endianness;

pub fn dump_word(word: u16, endian: Endianness) -> [u8; 2] {
   let word_le = word.to_le(); 
   let hi = ((word & 0xFF00) >> 8) as u8;
   let lo = (word & 0x00FF) as u8;

   match endian {
        Endianness::Little => [lo, hi],
        Endianness::Big => [hi, lo],
   }
}

pub fn dump_dword(word: u32, endian: Endianness) -> [u8; 4] {
   let word_le = word.to_le(); 
   let b0 = ((word & 0xFF000000) >> 24) as u8;
   let b1 = ((word & 0x00FF0000) >> 16) as u8;
   let b2 = ((word & 0x0000FF00) >> 8) as u8;
   let b3 = (word & 0x000000FF) as u8;

   match endian {
        Endianness::Little => [b3, b2, b1, b0],
        Endianness::Big => [b0, b1, b2, b3],
   }
}

pub fn dump_qword(word: u64, endian: Endianness) -> [u8; 8] {
   let word_le = word.to_le(); 
   let b0 = ((word & (0xFF << 56)) >> 56) as u8;
   let b1 = ((word & (0xFF << 48)) >> 48) as u8;
   let b2 = ((word & (0xFF << 40)) >> 40) as u8;
   let b3 = ((word & (0xFF << 32)) >> 32) as u8;
   let b4 = ((word & (0xFF << 24)) >> 24) as u8;
   let b5 = ((word & (0xFF << 16)) >> 16) as u8;
   let b6 = ((word & (0xFF << 8)) >> 8) as u8;
   let b7 = (word & 0xFF) as u8;

   match endian {
        Endianness::Little => [b7, b6, b5, b4, b3, b2, b1, b0],
        Endianness::Big => [b0, b1, b2, b3, b4, b5, b6, b7],
   }
}
