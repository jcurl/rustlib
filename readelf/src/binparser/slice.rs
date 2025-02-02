use super::{BinParser, Endian};

pub(crate) struct Slice<'elf> {
    buffer: &'elf [u8],
}

impl<'elf> Slice<'elf> {
    /// Create a buffer instance to read an ELF file.
    pub(crate) fn new(buffer: &'elf [u8]) -> Slice<'elf> {
        Slice { buffer }
    }
}

impl<'elf> BinParser for Slice<'elf> {
    #[inline(always)]
    fn get_u8(&self, offset: u64) -> Option<u8> {
        if self.buffer.len() < std::mem::size_of::<u8>() || offset >= self.buffer.len() as u64 {
            return None;
        }
        Some(self.buffer[offset as usize])
    }

    #[inline(always)]
    fn get_u16(&self, offset: u64, e: Endian) -> Option<u16> {
        if self.buffer.len() < std::mem::size_of::<u16>()
            || offset > (self.buffer.len() - std::mem::size_of::<u16>()) as u64
        {
            return None;
        };

        let i = offset as usize;
        let j = offset as usize + std::mem::size_of::<u16>();
        let slice = self.buffer[i..j].try_into().unwrap();
        match e {
            Endian::Little => Some(u16::from_le_bytes(slice)),
            Endian::Big => Some(u16::from_be_bytes(slice)),
        }
    }

    #[inline(always)]
    fn get_u32(&self, offset: u64, e: Endian) -> Option<u32> {
        if self.buffer.len() < std::mem::size_of::<u32>()
            || offset > (self.buffer.len() - std::mem::size_of::<u32>()) as u64
        {
            return None;
        };

        let i = offset as usize;
        let j = offset as usize + std::mem::size_of::<u32>();
        let slice = self.buffer[i..j].try_into().unwrap();
        match e {
            Endian::Little => Some(u32::from_le_bytes(slice)),
            Endian::Big => Some(u32::from_be_bytes(slice)),
        }
    }

    #[inline(always)]
    fn get_u64(&self, offset: u64, e: Endian) -> Option<u64> {
        if self.buffer.len() < std::mem::size_of::<u64>()
            || offset > (self.buffer.len() - std::mem::size_of::<u64>()) as u64
        {
            return None;
        };

        let i = offset as usize;
        let j = offset as usize + std::mem::size_of::<u64>();
        let slice = self.buffer[i..j].try_into().unwrap();
        match e {
            Endian::Little => Some(u64::from_le_bytes(slice)),
            Endian::Big => Some(u64::from_be_bytes(slice)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BinParser, Endian, Slice};

    static TEST_BUFFER: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    static TEST_BUFFER_0: [u8; 0] = [];
    static TEST_BUFFER_1: [u8; 1] = [1];
    static TEST_BUFFER_2: [u8; 2] = [1, 2];
    static TEST_BUFFER_3: [u8; 3] = [1, 2, 3];
    static TEST_BUFFER_4: [u8; 4] = [1, 2, 3, 4];
    static TEST_BUFFER_7: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];
    static TEST_BUFFER_8: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    #[test]
    fn test_get_u8() {
        let buffer = Slice::new(&TEST_BUFFER);

        assert_eq!(buffer.get_u8(0), Some(1));
        assert_eq!(buffer.get_u8(9), Some(10));
        assert_eq!(buffer.get_u8(10), None);
        assert_eq!(buffer.get_u8(u64::MAX), None);

        let buffer_0 = Slice::new(&TEST_BUFFER_0);
        assert_eq!(buffer_0.get_u8(0), None);
        assert_eq!(buffer_0.get_u8(1), None);
        assert_eq!(buffer_0.get_u8(u64::MAX), None);

        let buffer_1 = Slice::new(&TEST_BUFFER_1);
        assert_eq!(buffer_1.get_u8(0), Some(1));
        assert_eq!(buffer_1.get_u8(1), None);
        assert_eq!(buffer_1.get_u8(u64::MAX), None);
    }

    #[test]
    fn test_get_u16() {
        let buffer = Slice::new(&TEST_BUFFER);

        assert_eq!(buffer.get_u16(0, Endian::Big), Some(0x0102));
        assert_eq!(buffer.get_u16(0, Endian::Little), Some(0x0201));
        assert_eq!(buffer.get_u16(8, Endian::Big), Some(0x090A));
        assert_eq!(buffer.get_u16(8, Endian::Little), Some(0x0A09));
        assert_eq!(buffer.get_u16(9, Endian::Big), None);
        assert_eq!(buffer.get_u16(9, Endian::Little), None);
        assert_eq!(buffer.get_u16(10, Endian::Big), None);
        assert_eq!(buffer.get_u16(10, Endian::Little), None);
        assert_eq!(buffer.get_u16(u64::MAX, Endian::Big), None);
        assert_eq!(buffer.get_u16(u64::MAX, Endian::Little), None);

        let buffer_0 = Slice::new(&TEST_BUFFER_0);
        assert_eq!(buffer_0.get_u16(0, Endian::Big), None);
        assert_eq!(buffer_0.get_u16(0, Endian::Little), None);
        assert_eq!(buffer_0.get_u16(1, Endian::Big), None);
        assert_eq!(buffer_0.get_u16(1, Endian::Little), None);
        assert_eq!(buffer_0.get_u16(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_0.get_u16(u64::MAX, Endian::Little), None);

        let buffer_1 = Slice::new(&TEST_BUFFER_1);
        assert_eq!(buffer_1.get_u16(0, Endian::Big), None);
        assert_eq!(buffer_1.get_u16(0, Endian::Little), None);
        assert_eq!(buffer_1.get_u16(1, Endian::Big), None);
        assert_eq!(buffer_1.get_u16(1, Endian::Little), None);
        assert_eq!(buffer_1.get_u16(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_1.get_u16(u64::MAX, Endian::Little), None);

        let buffer_2 = Slice::new(&TEST_BUFFER_2);
        assert_eq!(buffer_2.get_u16(0, Endian::Big), Some(0x0102));
        assert_eq!(buffer_2.get_u16(0, Endian::Little), Some(0x0201));
        assert_eq!(buffer_2.get_u16(1, Endian::Big), None);
        assert_eq!(buffer_2.get_u16(1, Endian::Little), None);
        assert_eq!(buffer_2.get_u16(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_2.get_u16(u64::MAX, Endian::Little), None);
    }

    #[test]
    fn test_get_u32() {
        let buffer = Slice::new(&TEST_BUFFER);

        assert_eq!(buffer.get_u32(0, Endian::Big), Some(0x01020304));
        assert_eq!(buffer.get_u32(0, Endian::Little), Some(0x04030201));
        assert_eq!(buffer.get_u32(6, Endian::Big), Some(0x0708090A));
        assert_eq!(buffer.get_u32(6, Endian::Little), Some(0x0A090807));
        assert_eq!(buffer.get_u32(7, Endian::Big), None);
        assert_eq!(buffer.get_u32(7, Endian::Little), None);
        assert_eq!(buffer.get_u32(10, Endian::Big), None);
        assert_eq!(buffer.get_u32(10, Endian::Little), None);
        assert_eq!(buffer.get_u32(u64::MAX, Endian::Big), None);
        assert_eq!(buffer.get_u32(u64::MAX, Endian::Little), None);

        let buffer_0 = Slice::new(&TEST_BUFFER_0);
        assert_eq!(buffer_0.get_u32(0, Endian::Big), None);
        assert_eq!(buffer_0.get_u32(0, Endian::Little), None);
        assert_eq!(buffer_0.get_u32(1, Endian::Big), None);
        assert_eq!(buffer_0.get_u32(1, Endian::Little), None);
        assert_eq!(buffer_0.get_u32(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_0.get_u32(u64::MAX, Endian::Little), None);

        let buffer_1 = Slice::new(&TEST_BUFFER_1);
        assert_eq!(buffer_1.get_u32(0, Endian::Big), None);
        assert_eq!(buffer_1.get_u32(0, Endian::Little), None);
        assert_eq!(buffer_1.get_u32(1, Endian::Big), None);
        assert_eq!(buffer_1.get_u32(1, Endian::Little), None);
        assert_eq!(buffer_1.get_u32(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_1.get_u32(u64::MAX, Endian::Little), None);

        let buffer_2 = Slice::new(&TEST_BUFFER_2);
        assert_eq!(buffer_2.get_u32(0, Endian::Big), None);
        assert_eq!(buffer_2.get_u32(0, Endian::Little), None);
        assert_eq!(buffer_2.get_u32(2, Endian::Big), None);
        assert_eq!(buffer_2.get_u32(2, Endian::Little), None);
        assert_eq!(buffer_2.get_u32(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_2.get_u32(u64::MAX, Endian::Little), None);

        let buffer_3 = Slice::new(&TEST_BUFFER_3);
        assert_eq!(buffer_3.get_u32(0, Endian::Big), None);
        assert_eq!(buffer_3.get_u32(0, Endian::Little), None);
        assert_eq!(buffer_3.get_u32(2, Endian::Big), None);
        assert_eq!(buffer_3.get_u32(2, Endian::Little), None);
        assert_eq!(buffer_3.get_u32(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_3.get_u32(u64::MAX, Endian::Little), None);

        let buffer_4 = Slice::new(&TEST_BUFFER_4);
        assert_eq!(buffer_4.get_u32(0, Endian::Big), Some(0x01020304));
        assert_eq!(buffer_4.get_u32(0, Endian::Little), Some(0x04030201));
        assert_eq!(buffer_4.get_u32(1, Endian::Big), None);
        assert_eq!(buffer_4.get_u32(1, Endian::Little), None);
        assert_eq!(buffer_4.get_u32(4, Endian::Big), None);
        assert_eq!(buffer_4.get_u32(4, Endian::Little), None);
        assert_eq!(buffer_4.get_u32(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_4.get_u32(u64::MAX, Endian::Little), None);
    }

    #[test]
    fn test_get_u64() {
        let buffer = Slice::new(&TEST_BUFFER);

        assert_eq!(buffer.get_u64(0, Endian::Big), Some(0x0102030405060708));
        assert_eq!(buffer.get_u64(0, Endian::Little), Some(0x0807060504030201));
        assert_eq!(buffer.get_u64(2, Endian::Big), Some(0x030405060708090A));
        assert_eq!(buffer.get_u64(2, Endian::Little), Some(0x0A09080706050403));
        assert_eq!(buffer.get_u64(3, Endian::Big), None);
        assert_eq!(buffer.get_u64(3, Endian::Little), None);
        assert_eq!(buffer.get_u64(10, Endian::Big), None);
        assert_eq!(buffer.get_u64(10, Endian::Little), None);
        assert_eq!(buffer.get_u64(u64::MAX, Endian::Big), None);
        assert_eq!(buffer.get_u64(u64::MAX, Endian::Little), None);

        let buffer_0 = Slice::new(&TEST_BUFFER_0);
        assert_eq!(buffer_0.get_u64(0, Endian::Big), None);
        assert_eq!(buffer_0.get_u64(0, Endian::Little), None);
        assert_eq!(buffer_0.get_u64(1, Endian::Big), None);
        assert_eq!(buffer_0.get_u64(1, Endian::Little), None);
        assert_eq!(buffer_0.get_u64(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_0.get_u64(u64::MAX, Endian::Little), None);

        let buffer_1 = Slice::new(&TEST_BUFFER_1);
        assert_eq!(buffer_1.get_u64(0, Endian::Big), None);
        assert_eq!(buffer_1.get_u64(0, Endian::Little), None);
        assert_eq!(buffer_1.get_u64(1, Endian::Big), None);
        assert_eq!(buffer_1.get_u64(1, Endian::Little), None);
        assert_eq!(buffer_1.get_u64(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_1.get_u64(u64::MAX, Endian::Little), None);

        let buffer_2 = Slice::new(&TEST_BUFFER_2);
        assert_eq!(buffer_2.get_u64(0, Endian::Big), None);
        assert_eq!(buffer_2.get_u64(0, Endian::Little), None);
        assert_eq!(buffer_2.get_u64(2, Endian::Big), None);
        assert_eq!(buffer_2.get_u64(2, Endian::Little), None);
        assert_eq!(buffer_2.get_u64(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_2.get_u64(u64::MAX, Endian::Little), None);

        let buffer_3 = Slice::new(&TEST_BUFFER_3);
        assert_eq!(buffer_3.get_u64(0, Endian::Big), None);
        assert_eq!(buffer_3.get_u64(0, Endian::Little), None);
        assert_eq!(buffer_3.get_u64(2, Endian::Big), None);
        assert_eq!(buffer_3.get_u64(2, Endian::Little), None);
        assert_eq!(buffer_3.get_u64(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_3.get_u64(u64::MAX, Endian::Little), None);

        let buffer_4 = Slice::new(&TEST_BUFFER_3);
        assert_eq!(buffer_4.get_u64(0, Endian::Big), None);
        assert_eq!(buffer_4.get_u64(0, Endian::Little), None);
        assert_eq!(buffer_4.get_u64(4, Endian::Big), None);
        assert_eq!(buffer_4.get_u64(4, Endian::Little), None);
        assert_eq!(buffer_4.get_u64(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_4.get_u64(u64::MAX, Endian::Little), None);

        let buffer_7 = Slice::new(&TEST_BUFFER_7);
        assert_eq!(buffer_7.get_u64(0, Endian::Big), None);
        assert_eq!(buffer_7.get_u64(0, Endian::Little), None);
        assert_eq!(buffer_7.get_u64(8, Endian::Big), None);
        assert_eq!(buffer_7.get_u64(8, Endian::Little), None);
        assert_eq!(buffer_7.get_u64(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_7.get_u64(u64::MAX, Endian::Little), None);

        let buffer_8 = Slice::new(&TEST_BUFFER_8);
        assert_eq!(buffer_8.get_u64(0, Endian::Big), Some(0x0102030405060708));
        assert_eq!(
            buffer_8.get_u64(0, Endian::Little),
            Some(0x0807060504030201)
        );
        assert_eq!(buffer_8.get_u64(1, Endian::Big), None);
        assert_eq!(buffer_8.get_u64(1, Endian::Little), None);
        assert_eq!(buffer_8.get_u64(8, Endian::Big), None);
        assert_eq!(buffer_8.get_u64(8, Endian::Little), None);
        assert_eq!(buffer_8.get_u64(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_8.get_u64(u64::MAX, Endian::Little), None);
    }
}
