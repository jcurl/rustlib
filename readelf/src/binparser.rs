use super::{Class, Endian};

mod slice;
pub(crate) use slice::Slice;

mod vecbuffer;
pub(crate) use vecbuffer::VecBuffer;

mod file;
pub(crate) use file::File;

/// BinParser has common methods to get values from an ELF file.
///
/// Get values from the ELF file, depending on the header of the ELF file.
pub(crate) trait BinParser {
    /// Get a single byte at the offset given.
    fn get_u8(&self, offset: u64) -> Option<u8>;

    /// Get a 16-bit value at the offset given.
    ///
    /// The bytes are swapped as necessary depending on [Endian] of the ELF
    /// file.
    ///
    /// # Returns
    ///
    /// If the `offset` is out of range, then `None` is returned.
    fn get_u16(&self, offset: u64, e: Endian) -> Option<u16>;

    /// Get a 32-bit value at the offset given.
    ///
    /// The bytes are swapped as necessary depending on [Endian] of the ELF
    /// file.
    ///
    /// # Returns
    ///
    /// If the `offset` is out of range, then `None` is returned.
    fn get_u32(&self, offset: u64, e: Endian) -> Option<u32>;

    /// Get a 64-bit value at the offset given.
    ///
    /// The bytes are swapped as necessary depending on [Endian] of the ELF
    /// file.
    ///
    /// # Returns
    ///
    /// If the `offset` is out of range, then `None` is returned.
    fn get_u64(&self, offset: u64, e: Endian) -> Option<u64>;

    /// Get a "native" bit value at the offset given.
    ///
    /// The bytes are swapped as necessary depending on [Endian] of the ELF
    /// file. For [Class::Elf32] a 32-bit value is read, for [Class::Elf64] a
    /// 64-bit value is read.
    ///
    /// # Returns
    ///
    /// If the `offset` is out of range, then `None` is returned.
    fn get_usize(&self, offset: u64, e: Endian, c: Class) -> Option<u64> {
        match c {
            Class::Elf32 => self.get_u32(offset, e).map(|v| v as u64),
            Class::Elf64 => self.get_u64(offset, e),
        }
    }
}
