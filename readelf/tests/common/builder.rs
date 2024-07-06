use readelf::{Endian, ExecutableType, Machine, OsAbi, ProgramHeader};

// Some methods are provided for completeness, even if they're not used (e.g.
// `write_u*()`).
#[allow(dead_code)]
pub trait ElfBuilder<'b> {
    fn set_os_abi(&mut self, abi: OsAbi) -> &mut Self;
    fn set_abi_version(&mut self, abi_version: u8) -> &mut Self;
    fn set_executable_type(&mut self, exec_type: ExecutableType) -> &mut Self;
    fn set_machine(&mut self, machine: Machine) -> &mut Self;
    fn set_entry(&mut self, entry: u64) -> &mut Self;
    fn set_flags(&mut self, flags: u32) -> &mut Self;
    fn add_segment(&mut self, segment: &ProgramHeader) -> bool;
    fn buffer(&'b self) -> &'b [u8];

    fn write_u8(&mut self, offset: usize, value: u8);
    fn write_u16(&mut self, offset: usize, value: u16);
    fn write_u32(&mut self, offset: usize, value: u32);
    fn write_u64(&mut self, offset: usize, value: u64);
}

fn write_u8(buffer: &mut [u8], value: u8) {
    if buffer.len() < std::mem::size_of::<u8>() {
        panic!("Buffer too small")
    };
    buffer[0] = value;
}

fn write_u16(buffer: &mut [u8], value: u16, endian: Endian) {
    if buffer.len() < std::mem::size_of::<u16>() {
        panic!("Buffer too small")
    };

    // Some sugar to automatically write the correct size.
    let wrbuff = if buffer.len() == std::mem::size_of::<u16>() {
        buffer
    } else {
        &mut buffer[0..2]
    };

    match endian {
        Endian::Little => wrbuff.copy_from_slice(&value.to_le_bytes()),
        Endian::Big => wrbuff.copy_from_slice(&value.to_be_bytes()),
    };
}

fn write_u32(buffer: &mut [u8], value: u32, endian: Endian) {
    if buffer.len() < std::mem::size_of::<u32>() {
        panic!("Buffer too small")
    };

    // Some sugar to automatically write the correct size.
    let wrbuff = if buffer.len() == std::mem::size_of::<u32>() {
        buffer
    } else {
        &mut buffer[0..4]
    };

    match endian {
        Endian::Little => wrbuff.copy_from_slice(&value.to_le_bytes()),
        Endian::Big => wrbuff.copy_from_slice(&value.to_be_bytes()),
    };
}

fn write_u64(buffer: &mut [u8], value: u64, endian: Endian) {
    if buffer.len() < std::mem::size_of::<u64>() {
        panic!("Buffer too small")
    };

    // Some sugar to automatically write the correct size.
    let wrbuff = if buffer.len() == std::mem::size_of::<u64>() {
        buffer
    } else {
        &mut buffer[0..8]
    };

    match endian {
        Endian::Little => wrbuff.copy_from_slice(&value.to_le_bytes()),
        Endian::Big => wrbuff.copy_from_slice(&value.to_be_bytes()),
    };
}

pub struct ElfBuilder32 {
    buffer: [u8; 8192],
    endian: Endian,
    segment_index: usize,
}

impl ElfBuilder32 {
    const E_PHOFF: usize = 0x34;
    const E_PHENTSIZE: usize = 0x20;

    pub fn new(endian: Endian) -> ElfBuilder32 {
        let mut elf = ElfBuilder32 {
            buffer: [0; 8192],
            endian,
            segment_index: 0,
        };

        write_u32(&mut elf.buffer[0..4], 0x7f454c46, Endian::Big);
        write_u8(&mut elf.buffer[4..5], 0x01);
        match endian {
            Endian::Big => write_u8(&mut elf.buffer[5..6], 0x02),
            Endian::Little => write_u8(&mut elf.buffer[5..6], 0x01),
        }
        write_u8(&mut elf.buffer[6..7], 0x01);
        write_u32(&mut elf.buffer[20..24], 0x01, endian);

        elf
    }
}

impl<'b> ElfBuilder<'b> for ElfBuilder32 {
    fn set_os_abi(&mut self, abi: OsAbi) -> &mut Self {
        write_u8(&mut self.buffer[7..8], u8::from(abi));
        self
    }

    fn set_abi_version(&mut self, abi_version: u8) -> &mut Self {
        write_u8(&mut self.buffer[8..9], abi_version);
        self
    }

    fn set_executable_type(&mut self, exec_type: ExecutableType) -> &mut Self {
        write_u16(&mut self.buffer[16..18], u16::from(exec_type), self.endian);
        self
    }

    fn set_machine(&mut self, machine: Machine) -> &mut Self {
        write_u16(&mut self.buffer[18..20], u16::from(machine), self.endian);
        self
    }

    fn set_entry(&mut self, entry: u64) -> &mut Self {
        write_u32(&mut self.buffer[24..28], entry as u32, self.endian);
        self
    }

    fn set_flags(&mut self, flags: u32) -> &mut Self {
        write_u32(&mut self.buffer[36..40], flags, self.endian);
        self
    }

    fn add_segment(&mut self, segment: &ProgramHeader) -> bool {
        if self.segment_index > 16 {
            false
        } else {
            // Array is in the range of 0x34 .. 0x0400. The value is always in
            // range.
            let segment_base =
                ElfBuilder32::E_PHOFF + self.segment_index * ElfBuilder32::E_PHENTSIZE;

            write_u32(
                &mut self.buffer[segment_base..],
                u32::from(segment.segment_type),
                self.endian,
            );
            write_u32(
                &mut self.buffer[segment_base + 4..],
                segment.file_offset as u32,
                self.endian,
            );
            write_u32(
                &mut self.buffer[segment_base + 8..],
                segment.virtual_address as u32,
                self.endian,
            );
            write_u32(
                &mut self.buffer[segment_base + 12..],
                segment.physical_address as u32,
                self.endian,
            );
            write_u32(
                &mut self.buffer[segment_base + 16..],
                segment.file_size as u32,
                self.endian,
            );
            write_u32(
                &mut self.buffer[segment_base + 20..],
                segment.memory_size as u32,
                self.endian,
            );
            write_u32(
                &mut self.buffer[segment_base + 24..],
                u32::from(segment.flags),
                self.endian,
            );
            write_u32(
                &mut self.buffer[segment_base + 28..],
                segment.alignment as u32,
                self.endian,
            );

            self.segment_index += 1;
            write_u32(
                &mut self.buffer[28..32],
                ElfBuilder32::E_PHOFF as u32,
                self.endian,
            );
            write_u16(
                &mut self.buffer[42..44],
                ElfBuilder32::E_PHENTSIZE as u16,
                self.endian,
            );
            write_u16(
                &mut self.buffer[44..46],
                self.segment_index as u16,
                self.endian,
            );
            true
        }
    }

    fn buffer(&'b self) -> &'b [u8] {
        &self.buffer
    }

    fn write_u8(&mut self, offset: usize, value: u8) {
        super::builder::write_u8(&mut self.buffer[offset..], value);
    }

    fn write_u16(&mut self, offset: usize, value: u16) {
        super::builder::write_u16(&mut self.buffer[offset..], value, self.endian);
    }

    fn write_u32(&mut self, offset: usize, value: u32) {
        super::builder::write_u32(&mut self.buffer[offset..], value, self.endian);
    }

    fn write_u64(&mut self, offset: usize, value: u64) {
        super::builder::write_u64(&mut self.buffer[offset..], value, self.endian);
    }
}

pub struct ElfBuilder64 {
    buffer: [u8; 8192],
    endian: Endian,
    segment_index: usize,
}

impl ElfBuilder64 {
    const E_PHOFF: usize = 0x40;
    const E_PHENTSIZE: usize = 0x38;

    pub fn new(endian: Endian) -> ElfBuilder64 {
        let mut elf = ElfBuilder64 {
            buffer: [0; 8192],
            endian,
            segment_index: 0,
        };

        write_u32(&mut elf.buffer[0..4], 0x7f454c46, Endian::Big);
        write_u8(&mut elf.buffer[4..5], 0x02);
        match endian {
            Endian::Big => write_u8(&mut elf.buffer[5..6], 0x02),
            Endian::Little => write_u8(&mut elf.buffer[5..6], 0x01),
        }
        write_u8(&mut elf.buffer[6..7], 0x01);
        write_u32(&mut elf.buffer[20..24], 0x01, endian);

        elf
    }
}

impl<'b> ElfBuilder<'b> for ElfBuilder64 {
    fn set_os_abi(&mut self, abi: OsAbi) -> &mut Self {
        write_u8(&mut self.buffer[7..8], u8::from(abi));
        self
    }

    fn set_abi_version(&mut self, abi_version: u8) -> &mut Self {
        write_u8(&mut self.buffer[8..9], abi_version);
        self
    }

    fn set_executable_type(&mut self, exec_type: ExecutableType) -> &mut Self {
        write_u16(&mut self.buffer[16..18], u16::from(exec_type), self.endian);
        self
    }

    fn set_machine(&mut self, machine: Machine) -> &mut Self {
        write_u16(&mut self.buffer[18..20], u16::from(machine), self.endian);
        self
    }

    fn set_entry(&mut self, entry: u64) -> &mut Self {
        write_u64(&mut self.buffer[24..32], entry, self.endian);
        self
    }

    fn set_flags(&mut self, flags: u32) -> &mut Self {
        write_u32(&mut self.buffer[48..52], flags, self.endian);
        self
    }

    fn add_segment(&mut self, segment: &ProgramHeader) -> bool {
        if self.segment_index > 16 {
            false
        } else {
            // Array is in the range of 0x34 .. 0x0400. The value is always in
            // range.
            let segment_base =
                ElfBuilder64::E_PHOFF + self.segment_index * ElfBuilder64::E_PHENTSIZE;

            write_u32(
                &mut self.buffer[segment_base..],
                u32::from(segment.segment_type),
                self.endian,
            );
            write_u32(
                &mut self.buffer[segment_base + 4..],
                u32::from(segment.flags),
                self.endian,
            );
            write_u64(
                &mut self.buffer[segment_base + 8..],
                segment.file_offset,
                self.endian,
            );
            write_u64(
                &mut self.buffer[segment_base + 16..],
                segment.virtual_address,
                self.endian,
            );
            write_u64(
                &mut self.buffer[segment_base + 24..],
                segment.physical_address,
                self.endian,
            );
            write_u64(
                &mut self.buffer[segment_base + 32..],
                segment.file_size,
                self.endian,
            );
            write_u64(
                &mut self.buffer[segment_base + 40..],
                segment.memory_size,
                self.endian,
            );
            write_u64(
                &mut self.buffer[segment_base + 48..],
                segment.alignment,
                self.endian,
            );

            self.segment_index += 1;
            write_u64(
                &mut self.buffer[32..40],
                ElfBuilder64::E_PHOFF as u64,
                self.endian,
            );
            write_u16(
                &mut self.buffer[54..56],
                ElfBuilder64::E_PHENTSIZE as u16,
                self.endian,
            );
            write_u16(
                &mut self.buffer[56..58],
                self.segment_index as u16,
                self.endian,
            );
            true
        }
    }

    fn buffer(&'b self) -> &'b [u8] {
        &self.buffer
    }

    fn write_u8(&mut self, offset: usize, value: u8) {
        super::builder::write_u8(&mut self.buffer[offset..], value);
    }

    fn write_u16(&mut self, offset: usize, value: u16) {
        super::builder::write_u16(&mut self.buffer[offset..], value, self.endian);
    }

    fn write_u32(&mut self, offset: usize, value: u32) {
        super::builder::write_u32(&mut self.buffer[offset..], value, self.endian);
    }

    fn write_u64(&mut self, offset: usize, value: u64) {
        super::builder::write_u64(&mut self.buffer[offset..], value, self.endian);
    }
}

#[cfg(test)]
mod tests {
    // Internal test cases because it's easier to write than debug later for
    // silly mistakes. Testing the test code. We're not trying to be exhaustive
    // here, nor are we aiming for 100% coverage of our test code.

    use super::*;

    #[test]
    fn write_u8_ok() {
        let mut buffer = [0_u8; 1];
        write_u8(&mut buffer, 0x01);
        assert_eq!(buffer[0], 0x01);
    }

    #[test]
    #[should_panic]
    fn write_u8_panic() {
        let mut buffer = [0_u8; 0];
        write_u8(&mut buffer, 0x01);
    }

    #[test]
    fn write_u16_ok() {
        let mut buffer = [0_u8; 2];

        write_u16(&mut buffer, 0x1234, Endian::Big);
        assert_eq!(buffer[0], 0x12);
        assert_eq!(buffer[1], 0x34);

        write_u16(&mut buffer, 0x1234, Endian::Little);
        assert_eq!(buffer[0], 0x34);
        assert_eq!(buffer[1], 0x12);
    }

    #[test]
    #[should_panic]
    fn write_u16_panic() {
        let mut buffer = [1_u8; 0];
        write_u16(&mut buffer, 0x0102, Endian::Big);
    }

    #[test]
    fn write_u32_ok() {
        let mut buffer = [0_u8; 4];

        write_u32(&mut buffer, 0x12345678, Endian::Big);
        assert_eq!(buffer[0], 0x12);
        assert_eq!(buffer[1], 0x34);
        assert_eq!(buffer[2], 0x56);
        assert_eq!(buffer[3], 0x78);

        write_u32(&mut buffer, 0x12345678, Endian::Little);
        assert_eq!(buffer[0], 0x78);
        assert_eq!(buffer[1], 0x56);
        assert_eq!(buffer[2], 0x34);
        assert_eq!(buffer[3], 0x12);
    }

    #[test]
    #[should_panic]
    fn write_u32_panic() {
        let mut buffer = [3_u8; 0];
        write_u32(&mut buffer, 0x01020304, Endian::Big);
    }

    #[test]
    fn write_u64_ok() {
        let mut buffer = [0_u8; 8];

        write_u64(&mut buffer, 0x123456789abcdef0, Endian::Big);
        assert_eq!(buffer[0], 0x12);
        assert_eq!(buffer[1], 0x34);
        assert_eq!(buffer[2], 0x56);
        assert_eq!(buffer[3], 0x78);
        assert_eq!(buffer[4], 0x9a);
        assert_eq!(buffer[5], 0xbc);
        assert_eq!(buffer[6], 0xde);
        assert_eq!(buffer[7], 0xf0);

        write_u64(&mut buffer, 0x123456789abcdef0, Endian::Little);
        assert_eq!(buffer[0], 0xf0);
        assert_eq!(buffer[1], 0xde);
        assert_eq!(buffer[2], 0xbc);
        assert_eq!(buffer[3], 0x9a);
        assert_eq!(buffer[4], 0x78);
        assert_eq!(buffer[5], 0x56);
        assert_eq!(buffer[6], 0x34);
        assert_eq!(buffer[7], 0x12);
    }

    #[test]
    #[should_panic]
    fn write_u64_panic() {
        let mut buffer = [7_u8; 0];
        write_u64(&mut buffer, 0x0102030405060708, Endian::Big);
    }
}
