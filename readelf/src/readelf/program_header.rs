use crate::{Class, ReadElf, SegmentFlags, SegmentType};

/// Describes a segment on how an OS creates a process image.
#[derive(Debug)]
pub struct ProgramHeader {
    /// Identifies the type of the segment.
    pub segment_type: SegmentType,

    /// Segment dependent flags.
    pub flags: SegmentFlags,

    /// Offset of the segment in the file image.
    pub file_offset: u64,

    /// Virtual address of the segment in memory.
    pub virtual_address: u64,

    /// Physical address of the segment.
    ///
    /// Reserved for the segment's physical address on systems where a physical
    /// address is relevant.
    pub physical_address: u64,

    /// Size in bytes of the segment in the file image. May be zero.
    pub file_size: u64,

    /// Size in bytes of the segment in memory. May be zero.
    pub memory_size: u64,

    /// Integral power of two for segment alignment in memory.
    pub alignment: u64,
}

impl ProgramHeader {
    pub(super) fn new<'elf>(elf: &'elf ReadElf<'elf>, index: u16) -> Option<ProgramHeader> {
        // Ensure that the base offset of the segment doesn't overflow. The
        // `BinParser` will always check that the offsets are within the file.
        // The `ProgramHeaders`` class has already checked the validity of the
        // `program_header_size` field meets the minimum required.
        //
        // Note, here `offset` can't overflow. Multiplication of two 16-bit
        // values results in a 32-bit value.
        let offset = (index as u64) * (elf.program_header_size as u64);
        let base = elf.program_header_offset.checked_add(offset)?;
        if base > u64::MAX - elf.program_header_size as u64 {
            return None;
        }

        match elf.class {
            Class::Elf32 => Some(ProgramHeader {
                segment_type: SegmentType::from(elf.parser.get_u32(base, elf.data)?),
                flags: SegmentFlags::from(elf.parser.get_u32(base + 24, elf.data)?),
                file_offset: elf.parser.get_u32(base + 4, elf.data)? as u64,
                virtual_address: elf.parser.get_u32(base + 8, elf.data)? as u64,
                physical_address: elf.parser.get_u32(base + 12, elf.data)? as u64,
                file_size: elf.parser.get_u32(base + 16, elf.data)? as u64,
                memory_size: elf.parser.get_u32(base + 20, elf.data)? as u64,
                alignment: elf.parser.get_u32(base + 28, elf.data)? as u64,
            }),
            Class::Elf64 => Some(ProgramHeader {
                segment_type: SegmentType::from(elf.parser.get_u32(base, elf.data)?),
                flags: SegmentFlags::from(elf.parser.get_u32(base + 4, elf.data)?),
                file_offset: elf.parser.get_u64(base + 8, elf.data)?,
                virtual_address: elf.parser.get_u64(base + 16, elf.data)?,
                physical_address: elf.parser.get_u64(base + 24, elf.data)?,
                file_size: elf.parser.get_u64(base + 32, elf.data)?,
                memory_size: elf.parser.get_u64(base + 40, elf.data)?,
                alignment: elf.parser.get_u64(base + 48, elf.data)?,
            }),
        }
    }

    /// Check if the alignment is correct for a loadable segment.
    ///
    /// Loadable process segments must have congruent values for
    /// [ProgramHeader::virtual_address] and [ProgramHeader::file_offset],
    /// modulo their page size. Values of 0 or 1 for [ProgramHeader::alignment]
    /// mean no alignment is required. Otherwise, [ProgramHeader::alignment]
    /// must be a positive, integral power of 2, and
    /// [ProgramHeader::virtual_address] should equal
    /// [ProgramHeader::file_offset] modulo [ProgramHeader::alignment].
    ///
    /// This checks that the difference between the
    /// [ProgramHeader::virtual_address] and the [ProgramHeader::file_offset]
    /// should be a multiple of [ProgramHeader::alignment].
    pub fn is_aligned(&self) -> bool {
        self.alignment == 0
            || self.alignment == 1
            || self.alignment.is_power_of_two()
                && (self.virtual_address.wrapping_sub(self.file_offset) % self.alignment) == 0
    }
}
