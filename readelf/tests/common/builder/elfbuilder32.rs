use super::*;
use readelf::{Endian, ExecutableType, Machine, OsAbi, ProgramHeader};

pub struct ElfBuilder32 {
    buffer: [u8; 8192],
    endian: Endian,
    segment_index: usize,
    section_index: usize,
}

impl ElfBuilder32 {
    const E_PHOFF: usize = 0x34;
    const E_PHENTSIZE: usize = 0x20;
    const E_PHNUM_MAX: usize =
        (ElfBuilder32::E_SHOFF - ElfBuilder32::E_PHOFF) / ElfBuilder32::E_PHENTSIZE;

    const E_SHOFF: usize = 0x0400;
    const E_SHENTSIZE: usize = 0x28;
    const E_SHNUM_MAX: usize =
        (ElfBuilder32::E_DATA - ElfBuilder32::E_SHOFF) / ElfBuilder32::E_SHENTSIZE;

    const E_DATA: usize = 0x0800;
    const E_DATA_LEN: usize = 0x1800;

    pub fn new(endian: Endian) -> ElfBuilder32 {
        let mut elf = ElfBuilder32 {
            buffer: [0; 8192],
            endian,
            segment_index: 0,
            section_index: 0,
        };

        write_u32(&mut elf.buffer[0..4], 0x7f454c46, Endian::Big);

        elf.write_u8(4, 0x01);
        match endian {
            Endian::Big => elf.write_u8(5, 0x02),
            Endian::Little => elf.write_u8(5, 0x01),
        }
        elf.write_u8(6, 0x01);
        elf.write_u32(20, 0x01);
        elf
    }
}

impl<'b> ElfBuilder<'b> for ElfBuilder32 {
    fn set_os_abi(&mut self, abi: OsAbi) -> &mut Self {
        self.write_u8(7, u8::from(abi));
        self
    }

    fn set_abi_version(&mut self, abi_version: u8) -> &mut Self {
        self.write_u8(8, abi_version);
        self
    }

    fn set_executable_type(&mut self, exec_type: ExecutableType) -> &mut Self {
        self.write_u16(16, u16::from(exec_type));
        self
    }

    fn set_machine(&mut self, machine: Machine) -> &mut Self {
        self.write_u16(18, u16::from(machine));
        self
    }

    fn set_entry(&mut self, entry: u64) -> &mut Self {
        self.write_u32(24, entry as u32);
        self
    }

    fn set_flags(&mut self, flags: u32) -> &mut Self {
        self.write_u32(36, flags);
        self
    }

    fn write_u8(&mut self, offset: usize, value: u8) {
        super::write_u8(&mut self.buffer[offset..], value);
    }

    fn write_u16(&mut self, offset: usize, value: u16) {
        super::write_u16(&mut self.buffer[offset..], value, self.endian);
    }

    fn write_u32(&mut self, offset: usize, value: u32) {
        super::write_u32(&mut self.buffer[offset..], value, self.endian);
    }

    fn write_u64(&mut self, offset: usize, value: u64) {
        super::write_u64(&mut self.buffer[offset..], value, self.endian);
    }

    fn buffer(&'b self) -> &'b [u8] {
        &self.buffer
    }

    fn add_segment(&mut self, segment: &ProgramHeader) -> bool {
        if self.segment_index >= ElfBuilder32::E_PHNUM_MAX {
            false
        } else {
            // Array is in the range of 0x34 .. 0x0400. The value is always in
            // range.
            let segment_base =
                ElfBuilder32::E_PHOFF + self.segment_index * ElfBuilder32::E_PHENTSIZE;

            self.write_u32(segment_base, u32::from(segment.segment_type));
            self.write_u32(segment_base + 4, segment.file_offset as u32);
            self.write_u32(segment_base + 8, segment.virtual_address as u32);
            self.write_u32(segment_base + 12, segment.physical_address as u32);
            self.write_u32(segment_base + 16, segment.file_size as u32);
            self.write_u32(segment_base + 20, segment.memory_size as u32);
            self.write_u32(segment_base + 24, u32::from(segment.flags));
            self.write_u32(segment_base + 28, segment.alignment as u32);

            self.segment_index += 1;
            self.write_u32(28, ElfBuilder32::E_PHOFF as u32);
            self.write_u16(42, ElfBuilder32::E_PHENTSIZE as u16);
            self.write_u16(44, self.segment_index as u16);
            true
        }
    }

    fn add_section(&mut self, section: &SectionHeader) -> bool {
        if self.segment_index >= ElfBuilder32::E_SHNUM_MAX {
            false
        } else {
            let section_base =
                ElfBuilder32::E_SHOFF + self.section_index * ElfBuilder32::E_SHENTSIZE;

            // TODO: Write the string
            self.write_u32(section_base, 0);
            self.write_u32(section_base + 4, u32::from(section.section_type));
            self.write_u32(section_base + 8, u64::from(section.flags) as u32);
            self.write_u32(section_base + 12, section.virtual_address as u32);
            self.write_u32(section_base + 16, section.file_offset as u32);
            self.write_u32(section_base + 20, section.file_size as u32);
            self.write_u32(section_base + 24, section.section_link);
            self.write_u32(section_base + 28, section.section_info);
            self.write_u32(section_base + 32, section.alignment as u32);
            self.write_u32(section_base + 36, section.entry_size as u32);
            true
        }
    }
}
