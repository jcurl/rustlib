use super::{BinParser, Buffer, Endian, Slice};
use std::vec::Vec;

pub(crate) struct VecBuffer {
    buffer: Vec<u8>,
}

impl VecBuffer {
    pub(crate) fn new(buffer: Vec<u8>) -> VecBuffer {
        VecBuffer { buffer }
    }
}

impl BinParser for VecBuffer {
    fn get_u8(&self, offset: u64) -> Option<u8> {
        let slice = Slice::new(self.buffer.as_slice());
        slice.get_u8(offset)
    }

    fn get_u16(&self, offset: u64, e: Endian) -> Option<u16> {
        let slice = Slice::new(self.buffer.as_slice());
        slice.get_u16(offset, e)
    }

    fn get_u32(&self, offset: u64, e: Endian) -> Option<u32> {
        let slice = Slice::new(self.buffer.as_slice());
        slice.get_u32(offset, e)
    }

    fn get_u64(&self, offset: u64, e: Endian) -> Option<u64> {
        let slice = Slice::new(self.buffer.as_slice());
        slice.get_u64(offset, e)
    }

    fn get_map(&self, offset: u64, len: usize) -> Option<Buffer<'_>> {
        let start = offset as usize;
        let end = offset as usize + len;
        let buffer = self.buffer.as_slice().get(start..end);
        buffer.map(Buffer::AsRef)
    }
}

#[cfg(test)]
mod tests {
    use crate::binparser::VecBuffer;

    use super::{BinParser, Endian};

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
        let buffer = VecBuffer::new(Vec::from(TEST_BUFFER));

        assert_eq!(buffer.get_u8(0), Some(1));
        assert_eq!(buffer.get_u8(9), Some(10));
        assert_eq!(buffer.get_u8(10), None);
        assert_eq!(buffer.get_u8(u64::MAX), None);

        let buffer_0 = VecBuffer::new(Vec::from(TEST_BUFFER_0));
        assert_eq!(buffer_0.get_u8(0), None);
        assert_eq!(buffer_0.get_u8(1), None);
        assert_eq!(buffer_0.get_u8(u64::MAX), None);

        let buffer_1 = VecBuffer::new(Vec::from(TEST_BUFFER_1));
        assert_eq!(buffer_1.get_u8(0), Some(1));
        assert_eq!(buffer_1.get_u8(1), None);
        assert_eq!(buffer_1.get_u8(u64::MAX), None);
    }

    #[test]
    fn test_get_u16() {
        let buffer = VecBuffer::new(Vec::from(TEST_BUFFER));

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

        let buffer_0 = VecBuffer::new(Vec::from(TEST_BUFFER_0));
        assert_eq!(buffer_0.get_u16(0, Endian::Big), None);
        assert_eq!(buffer_0.get_u16(0, Endian::Little), None);
        assert_eq!(buffer_0.get_u16(1, Endian::Big), None);
        assert_eq!(buffer_0.get_u16(1, Endian::Little), None);
        assert_eq!(buffer_0.get_u16(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_0.get_u16(u64::MAX, Endian::Little), None);

        let buffer_1 = VecBuffer::new(Vec::from(TEST_BUFFER_1));
        assert_eq!(buffer_1.get_u16(0, Endian::Big), None);
        assert_eq!(buffer_1.get_u16(0, Endian::Little), None);
        assert_eq!(buffer_1.get_u16(1, Endian::Big), None);
        assert_eq!(buffer_1.get_u16(1, Endian::Little), None);
        assert_eq!(buffer_1.get_u16(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_1.get_u16(u64::MAX, Endian::Little), None);

        let buffer_2 = VecBuffer::new(Vec::from(TEST_BUFFER_2));
        assert_eq!(buffer_2.get_u16(0, Endian::Big), Some(0x0102));
        assert_eq!(buffer_2.get_u16(0, Endian::Little), Some(0x0201));
        assert_eq!(buffer_2.get_u16(1, Endian::Big), None);
        assert_eq!(buffer_2.get_u16(1, Endian::Little), None);
        assert_eq!(buffer_2.get_u16(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_2.get_u16(u64::MAX, Endian::Little), None);
    }

    #[test]
    fn test_get_u32() {
        let buffer = VecBuffer::new(Vec::from(TEST_BUFFER));

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

        let buffer_0 = VecBuffer::new(Vec::from(TEST_BUFFER_0));
        assert_eq!(buffer_0.get_u32(0, Endian::Big), None);
        assert_eq!(buffer_0.get_u32(0, Endian::Little), None);
        assert_eq!(buffer_0.get_u32(1, Endian::Big), None);
        assert_eq!(buffer_0.get_u32(1, Endian::Little), None);
        assert_eq!(buffer_0.get_u32(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_0.get_u32(u64::MAX, Endian::Little), None);

        let buffer_1 = VecBuffer::new(Vec::from(TEST_BUFFER_1));
        assert_eq!(buffer_1.get_u32(0, Endian::Big), None);
        assert_eq!(buffer_1.get_u32(0, Endian::Little), None);
        assert_eq!(buffer_1.get_u32(1, Endian::Big), None);
        assert_eq!(buffer_1.get_u32(1, Endian::Little), None);
        assert_eq!(buffer_1.get_u32(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_1.get_u32(u64::MAX, Endian::Little), None);

        let buffer_2 = VecBuffer::new(Vec::from(TEST_BUFFER_2));
        assert_eq!(buffer_2.get_u32(0, Endian::Big), None);
        assert_eq!(buffer_2.get_u32(0, Endian::Little), None);
        assert_eq!(buffer_2.get_u32(2, Endian::Big), None);
        assert_eq!(buffer_2.get_u32(2, Endian::Little), None);
        assert_eq!(buffer_2.get_u32(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_2.get_u32(u64::MAX, Endian::Little), None);

        let buffer_3 = VecBuffer::new(Vec::from(TEST_BUFFER_3));
        assert_eq!(buffer_3.get_u32(0, Endian::Big), None);
        assert_eq!(buffer_3.get_u32(0, Endian::Little), None);
        assert_eq!(buffer_3.get_u32(2, Endian::Big), None);
        assert_eq!(buffer_3.get_u32(2, Endian::Little), None);
        assert_eq!(buffer_3.get_u32(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_3.get_u32(u64::MAX, Endian::Little), None);

        let buffer_4 = VecBuffer::new(Vec::from(TEST_BUFFER_4));
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
        let buffer = VecBuffer::new(Vec::from(TEST_BUFFER));

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

        let buffer_0 = VecBuffer::new(Vec::from(TEST_BUFFER_0));
        assert_eq!(buffer_0.get_u64(0, Endian::Big), None);
        assert_eq!(buffer_0.get_u64(0, Endian::Little), None);
        assert_eq!(buffer_0.get_u64(1, Endian::Big), None);
        assert_eq!(buffer_0.get_u64(1, Endian::Little), None);
        assert_eq!(buffer_0.get_u64(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_0.get_u64(u64::MAX, Endian::Little), None);

        let buffer_1 = VecBuffer::new(Vec::from(TEST_BUFFER_1));
        assert_eq!(buffer_1.get_u64(0, Endian::Big), None);
        assert_eq!(buffer_1.get_u64(0, Endian::Little), None);
        assert_eq!(buffer_1.get_u64(1, Endian::Big), None);
        assert_eq!(buffer_1.get_u64(1, Endian::Little), None);
        assert_eq!(buffer_1.get_u64(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_1.get_u64(u64::MAX, Endian::Little), None);

        let buffer_2 = VecBuffer::new(Vec::from(TEST_BUFFER_2));
        assert_eq!(buffer_2.get_u64(0, Endian::Big), None);
        assert_eq!(buffer_2.get_u64(0, Endian::Little), None);
        assert_eq!(buffer_2.get_u64(2, Endian::Big), None);
        assert_eq!(buffer_2.get_u64(2, Endian::Little), None);
        assert_eq!(buffer_2.get_u64(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_2.get_u64(u64::MAX, Endian::Little), None);

        let buffer_3 = VecBuffer::new(Vec::from(TEST_BUFFER_3));
        assert_eq!(buffer_3.get_u64(0, Endian::Big), None);
        assert_eq!(buffer_3.get_u64(0, Endian::Little), None);
        assert_eq!(buffer_3.get_u64(2, Endian::Big), None);
        assert_eq!(buffer_3.get_u64(2, Endian::Little), None);
        assert_eq!(buffer_3.get_u64(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_3.get_u64(u64::MAX, Endian::Little), None);

        let buffer_4 = VecBuffer::new(Vec::from(TEST_BUFFER_4));
        assert_eq!(buffer_4.get_u64(0, Endian::Big), None);
        assert_eq!(buffer_4.get_u64(0, Endian::Little), None);
        assert_eq!(buffer_4.get_u64(4, Endian::Big), None);
        assert_eq!(buffer_4.get_u64(4, Endian::Little), None);
        assert_eq!(buffer_4.get_u64(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_4.get_u64(u64::MAX, Endian::Little), None);

        let buffer_7 = VecBuffer::new(Vec::from(TEST_BUFFER_7));
        assert_eq!(buffer_7.get_u64(0, Endian::Big), None);
        assert_eq!(buffer_7.get_u64(0, Endian::Little), None);
        assert_eq!(buffer_7.get_u64(8, Endian::Big), None);
        assert_eq!(buffer_7.get_u64(8, Endian::Little), None);
        assert_eq!(buffer_7.get_u64(u64::MAX, Endian::Big), None);
        assert_eq!(buffer_7.get_u64(u64::MAX, Endian::Little), None);

        let buffer_8 = VecBuffer::new(Vec::from(TEST_BUFFER_8));
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

    #[test]
    fn test_get_map() {
        let buffer = VecBuffer::new(Vec::from(TEST_BUFFER));

        let buffer_1 = buffer.get_map(4, 4).unwrap();
        assert!(buffer_1.is_ref());
        let slice_1 = buffer_1.buffer();
        assert_eq!(slice_1.len(), 4);
        assert_eq!(slice_1[0], 5);
        assert_eq!(slice_1[1], 6);
        assert_eq!(slice_1[2], 7);
        assert_eq!(slice_1[3], 8);

        let buffer_2 = buffer.get_map(0, 10).unwrap();
        assert!(buffer_2.is_ref());
        let slice_2 = buffer_2.buffer();
        assert_eq!(slice_2.len(), 10);
        assert_eq!(slice_2[0], 1);
        assert_eq!(slice_2[1], 2);
        assert_eq!(slice_2[2], 3);
        assert_eq!(slice_2[3], 4);

        let buffer_3 = buffer.get_map(6, 4).unwrap();
        assert!(buffer_3.is_ref());
        let slice_3 = buffer_3.buffer();
        assert_eq!(slice_3.len(), 4);
        assert_eq!(slice_3[0], 7);
        assert_eq!(slice_3[1], 8);
        assert_eq!(slice_3[2], 9);
        assert_eq!(slice_3[3], 10);
    }

    #[test]
    fn test_get_map_partial() {
        let buffer = VecBuffer::new(Vec::from(TEST_BUFFER));

        assert!(buffer.get_map(0, 11).is_none());
        assert!(buffer.get_map(4, 10).is_none());
        assert!(buffer.get_map(4, 7).is_none());
        assert!(buffer.get_map(10, 1).is_none());
    }
}
