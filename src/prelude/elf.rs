use super::{Endianness, utils::*, Program, Addr};
use std::fs::File;
use std::path::Path;
use std::io::Write;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ELFClass {
    X86,
    X86_64
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ELFType {
     None,
     Relocatable,
     Exectuable,
     SharedObject,
     Core
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ELFProgramHeaderType {
    Null,
    Loadable,
    Dynamic,
    Interpereter,
    Auxiliary,
    ProgramHeaderTable,
    TLS,
}

pub struct ELFHeader {
    pub class: ELFClass, 
    pub endianness: Endianness,
    pub elftype: ELFType,
    pub instruction_set: u16,
    pub entry_point: u64,
    pub program_table: u64,
}

pub struct ELFProgramHeader {
    pub class: ELFClass,
    pub p_type: ELFProgramHeaderType, 
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_filesz: u64,
}

pub struct ELF {
    pub class: ELFClass,
    pub header: ELFHeader,
    pub program_header: ELFProgramHeader,
    pub program: Program,
}

impl ELFHeader {
    pub fn new_x86(entry_point: u32) -> ELFHeader {
        ELFHeader {
            class: ELFClass::X86,
            endianness: Endianness::Little,
            elftype: ELFType::Exectuable,
            instruction_set: 0x03, // x86
            entry_point: entry_point as u64,
            program_table: 0x34,
        }
    }

    pub fn len(&self) -> usize {
        match self.class {
            ELFClass::X86 => 0x34,
            ELFClass::X86_64 => 0x40,
        }
    }

    pub fn as_vec(&self) -> Vec<u8> {
        let mut dump = Vec::new();

        // e_idnt[EI_MAG]: ELF magic number
        dump.push(0x7F);
        dump.push(0x45);
        dump.push(0x4C);
        dump.push(0x46);

        // e_ident[EI_CLASS]: 1 = 32 bit, 2 = 64 bit
        dump.push(match self.class {
            ELFClass::X86 => 0x01,
            ELFClass::X86_64 => 0x02,
        });

        // e_ident[EI_DATA]: 1 = little endian, 2 = big endian. Starts affecting data at 0x10
        dump.push(match self.endianness {
            Endianness::Little => 0x01,
            Endianness::Big => 0x02,
        });

        // e_ident[EI_VERSION]: needs to be 1
        dump.push(0x01);

        // e_ident[EI_OSABI]: Target operating system ABI. We use System V (0).
        dump.push(0x00);

        // e_ident[EI_ABIVERSION]: Specifies ABI version. Linux kernel ignores this, set to 0.
        dump.push(0x00);

        // e_ident[EI_PAD]: Reserved padding (seven bytes), currently unused. Set to 0.
        for _ in 0..7 {
            dump.push(0x00);
        }

        // e_type: Word identifying object file type.
        let bytes = dump_word(match self.elftype {
            ELFType::None => 0x00,
            ELFType::Relocatable => 0x01,
            ELFType::Exectuable => 0x02,
            ELFType::SharedObject => 0x03,
            ELFType::Core => 0x04,
        }, self.endianness);
        dump.push(bytes[0]);
        dump.push(bytes[1]);

        // e_machine: Specifies target instruction set architecture.
        let bytes = dump_word(self.instruction_set, self.endianness);
        dump.push(bytes[0]);
        dump.push(bytes[1]);

        // e_version: dword specifying ELF version. Set to 1. 
        let bytes = dump_dword(1, self.endianness);
        for i in 0..4 { dump.push(bytes[i]); }

        // e_entry: Memory address of entry point where the process starts executing. This field is
        // either 32 or 64 bits long depending on e_ident[EI_CLASS].
        match self.class {
            ELFClass::X86 => {
                let bytes = dump_dword(self.entry_point as u32, self.endianness);
                for i in 0..4 { dump.push(bytes[i]); }
            }
            ELFClass::X86_64 => {
                let bytes = dump_qword(self.entry_point, self.endianness);
                for i in 0..8 { dump.push(bytes[i]); }
            }
        }

        // e_phoff: Memory address of the program header table. Typically 0x34 or 0x40 for end of the ELF header.
        match self.class {
            ELFClass::X86 => {
                let bytes = dump_dword(self.program_table as u32, self.endianness);
                for i in 0..4 { dump.push(bytes[i]); }
            }
            ELFClass::X86_64 => {
                let bytes = dump_qword(self.program_table, self.endianness);
                for i in 0..8 { dump.push(bytes[i]); }
            }
        }

        // e_shoff: Memory address to the start of the section header table. 
        // TODO: Something other than 0.
        match self.class {
            ELFClass::X86 => {
                for _ in 0..4 { dump.push(0); }
            }
            ELFClass::X86_64 => {
                for _ in 0..8 { dump.push(0); }
            }
        }

        // e_flags: Unsure of this field, it depends on target architcture. TODO: Research this.
        for _ in 0..4 { dump.push(0); }

        // e_ehsize : Size of ELF header.
        let bytes = dump_word(self.len() as u16, self.endianness);
        dump.push(bytes[0]); dump.push(bytes[1]);

        // e_phentsize : Contains the size of the program header table entry.
        let bytes = dump_word(match self.class {
            ELFClass::X86 => 0x20,
            ELFClass::X86_64 => 0x38,
        }, self.endianness);
        dump.push(bytes[0]); dump.push(bytes[1]);

        // e_phnum: Number of entries in the program header table. We set this to 1.
        let bytes = dump_word(1, self.endianness);
        dump.push(bytes[0]); dump.push(bytes[1]);

        // e_shentsize: Contains the size of the section header table entry.
        let bytes = dump_word(match self.class {
            ELFClass::X86 => 0x28,
            ELFClass::X86_64 => 0x40,
        }, self.endianness);
        dump.push(bytes[0]); dump.push(bytes[1]);

        // e_shnum: Number of entries in the section header table. We set this to 0.
        let bytes = dump_word(0, self.endianness);
        dump.push(bytes[0]); dump.push(bytes[1]);

        // e_shstrndx: Contains the index of the section header table etnry that contains the
        // section names. We temporarily set this to 0.
        let bytes = dump_word(0, self.endianness);
        dump.push(bytes[0]); dump.push(bytes[1]);

        dump
    }
}

impl ELFProgramHeader {
    pub fn len(&self) -> usize {
        match self.class {
            ELFClass::X86 => 0x20,
            ELFClass::X86_64 => 0x38,
        }
    }
    
    pub fn as_vec(&self, endianness: Endianness) -> Vec<u8> {
        let mut dump = Vec::new();

        // p_type: Identifies the type of the segment.
        let bytes = dump_dword(match self.p_type {
            ELFProgramHeaderType::Null => 0,
            ELFProgramHeaderType::Loadable => 1,
            ELFProgramHeaderType::Dynamic => 2,
            ELFProgramHeaderType::Interpereter => 3,
            ELFProgramHeaderType::Auxiliary => 4,
            ELFProgramHeaderType::ProgramHeaderTable => 6,
            ELFProgramHeaderType::TLS => 7,
        }, endianness);
        for i in 0..4 { dump.push(bytes[i]); }

        // p_flags: 0x07 = exectuable + readable + readable. (64-bit only).
        if self.class == ELFClass::X86_64 {
            let bytes = dump_dword(0x07, endianness);
            for i in 0..4 { dump.push(bytes[i]); }
        }

        // p_offset: offset of the segment in the file image.
        match self.class {
            ELFClass::X86 => {
                let bytes = dump_dword(self.p_offset as u32, endianness);
                for i in 0..4 { dump.push(bytes[i]); }
            }
            ELFClass::X86_64 => {
                let bytes = dump_qword(self.p_offset, endianness);
                for i in 0..8 { dump.push(bytes[i]); }
            }
        }

        // p_vaddr: virtual address of the segment in memory.
        match self.class {
            ELFClass::X86 => {
                let bytes = dump_dword(self.p_vaddr as u32, endianness);
                for i in 0..4 { dump.push(bytes[i]); }
            }
            ELFClass::X86_64 => {
                let bytes = dump_qword(self.p_vaddr, endianness);
                for i in 0..8 { dump.push(bytes[i]); }
            }
        }

        // p_paddr: for systems with physical address, we this to zero
        match self.class {
            ELFClass::X86 => { for i in 0..4 { dump.push(0); } }
            ELFClass::X86_64 => { for i in 0..8 { dump.push(0); } }
        }

        // p_filesz: size in bytes of the segment in the file image.
        match self.class {
            ELFClass::X86 => {
                let bytes = dump_dword(self.p_filesz as u32, endianness);
                for i in 0..4 { dump.push(bytes[i]); }
            }
            ELFClass::X86_64 => {
                let bytes = dump_qword(self.p_filesz, endianness);
                for i in 0..8 { dump.push(bytes[i]); }
            }
        }

        // p_memsz: size in bytes of the segment in memory. we set p_memsz = p_filesz
        match self.class {
            ELFClass::X86 => {
                let bytes = dump_dword(0x40000000, endianness);
                for i in 0..4 { dump.push(bytes[i]); }
            }
            ELFClass::X86_64 => {
                let bytes = dump_qword(self.p_filesz, endianness);
                for i in 0..8 { dump.push(bytes[i]); }
            }
        }

        // p_flags: 0x05 = exectuable + writeable + readable. (32-bit only).
        if self.class == ELFClass::X86 {
            let bytes = dump_dword(0x07, endianness);
            for i in 0..4 { dump.push(bytes[i]); }
        }

        // p_align: Unsure of this, source sets it to 0x1000. (TODO: figure this out)
        match self.class {
            ELFClass::X86 => {
                let bytes = dump_dword(0x1000, endianness);
                for i in 0..4 { dump.push(bytes[i]); }
            }
            ELFClass::X86_64 => {
                let bytes = dump_qword(0x1000, endianness);
                for i in 0..8 { dump.push(bytes[i]); }
            }
        }

        dump
    }
}

impl ELF {
    pub fn new_x86(program: Program) -> ELF {
        let mut offset = Addr { addr: 0, vaddr: 0x08048000 }; // TODO: Figure out what this address is.
        let mut header = ELFHeader::new_x86(0);
        let mut program_header = ELFProgramHeader {
            class: ELFClass::X86,
            p_type: ELFProgramHeaderType::Loadable,
            p_offset: 0,
            p_vaddr: offset.vaddr,
            p_filesz: program.len() as u64,
        };

        offset += header.len() as u64 + program_header.len() as u64;
        header.entry_point = offset.vaddr + program.entry_point.addr; 
        program_header.p_offset = header.len() as u64 + program_header.len() as u64;
        program_header.p_vaddr += header.len() as u64 + program_header.len() as u64;

        let mut elf = ELF {
            class: ELFClass::X86,
            header,
            program_header,           
            program,
        };

        elf.program.offset = offset;
        elf
    }
    
    /// Saves the ELF binary to disk.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(&self.header.as_vec())?;
        file.write_all(&self.program_header.as_vec(self.header.endianness))?;
        file.write_all(&self.program.as_vec())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elf_header() {
        let result = vec![
            0x7F, 0x45, 0x4C, 0x46, // 04 e_ident[EI_MAG]
            0x01,                   // 05 e_ident[EI_CLASS]
            0x01,                   // 06 e_ident[EI_DATA]
            0x01,                   // 07 e_ident[EI_VERSION]
            0x00,                   // 08 e_ident[EI_OSABI]
            0x00,                   // 09 e_ident[EI_ABIVERSION]
            0x00, 0x00, 0x00,       // 0C e_ident[EI_PAD]
            0x00, 0x00, 0x00, 0x00, // 10 e_ident[EI_PAD]
            0x02, 0x00,             // 12 e_type
            0x03, 0x00,             // 14 e_machine 
            0x01, 0x00, 0x00, 0x00, // 18 e_version
            0x54, 0x80, 0x04, 0x08, // 1C e_entry
            0x34, 0x00, 0x00, 0x00, // 20 e_phof
            0x00, 0x00, 0x00, 0x00, // 24 e_shoff
            0x00, 0x00, 0x00, 0x00, // 28 e_flags
            0x34, 0x00,             // 2A e_ehsize
            0x20, 0x00,             // 2C e_phentsize
            0x01, 0x00,             // 2E e_phnum
            0x28, 0x00,             // 30 e_shentsize
            0x00, 0x00,             // 32 e_shnum
            0x00, 0x00,             // 34 e_shstrndx
        ];

        let header = ELFHeader::new_x86(0x08048054);

        assert_eq!(header.len(), 0x34);
        assert_eq!(header.as_vec(), result);
    }

    #[test]
    fn elf_program_header() {
        let result = vec![
            0x01, 0x00, 0x00, 0x00,  // 38 p_type
            0x54, 0x00, 0x00, 0x00,  // 3C p_offset
            0x54, 0x80, 0x04, 0x08,  // 40 p_vaddr
            0x00, 0x00, 0x00, 0x00,  // 44 p_paddr
            0x0C, 0x00, 0x00, 0x00,  // 48 p_filesz
            0x0C, 0x00, 0x00, 0x00,  // 4C p_memsz
            0x05, 0x00, 0x00, 0x00,  // 50 p_flags
            0x00, 0x10, 0x00, 0x00,  // 50 p_align
        ];

        let ph = ELFProgramHeader {
            class: ELFClass::X86,
            p_type: ELFProgramHeaderType::Loadable,
            p_offset: 0x54,
            p_vaddr: 0x08048054,
            p_filesz: 0x0C,
        };

        assert_eq!(ph.len(), 0x20);
        assert_eq!(ph.as_vec(Endianness::Little), result);
    }
}
