use super::{BinParser, Endian, Slice};
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
    #[inline(always)]
    fn get_u8(&self, offset: u64) -> Option<u8> {
        let slice = Slice::new(self.buffer.as_slice());
        slice.get_u8(offset)
    }

    #[inline(always)]
    fn get_u16(&self, offset: u64, e: Endian) -> Option<u16> {
        let slice = Slice::new(self.buffer.as_slice());
        slice.get_u16(offset, e)
    }

    #[inline(always)]
    fn get_u32(&self, offset: u64, e: Endian) -> Option<u32> {
        let slice = Slice::new(self.buffer.as_slice());
        slice.get_u32(offset, e)
    }

    #[inline(always)]
    fn get_u64(&self, offset: u64, e: Endian) -> Option<u64> {
        let slice = Slice::new(self.buffer.as_slice());
        slice.get_u64(offset, e)
    }
}
