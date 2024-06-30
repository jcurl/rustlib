use super::{BinParser, Endian};

pub(crate) struct File {}

impl File {
    pub(crate) fn new() -> File {
        File {}
    }
}

impl BinParser for File {
    fn get_u8(&self, _offset: u64) -> Option<u8> {
        None
    }

    fn get_u16(&self, _offset: u64, _e: Endian) -> Option<u16> {
        None
    }

    fn get_u32(&self, _offset: u64, _e: Endian) -> Option<u32> {
        None
    }

    fn get_u64(&self, _offset: u64, _e: Endian) -> Option<u64> {
        None
    }
}
