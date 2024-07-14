use readelf::{Endian, ExecutableType, Machine, OsAbi, ProgramHeader, SectionHeader};

mod elfbuilder32;
pub use elfbuilder32::ElfBuilder32;

mod elfbuilder64;
pub use elfbuilder64::ElfBuilder64;

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

    fn write_u8(&mut self, offset: usize, value: u8);
    fn write_u16(&mut self, offset: usize, value: u16);
    fn write_u32(&mut self, offset: usize, value: u32);
    fn write_u64(&mut self, offset: usize, value: u64);

    fn buffer(&'b self) -> &'b [u8];

    fn add_segment(&mut self, segment: &ProgramHeader) -> bool;
    fn add_section(&mut self, section: &SectionHeader) -> bool;
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
