use super::StringSection;
use crate::{Class, ReadElf, SectionFlags, SectionType};

/// Describes a segment on how an OS creates a process image.
#[derive(Debug, PartialEq)]
pub struct SectionHeader {
    /// Identifies the type of the segment.
    pub name: Option<String>,

    /// The type of the section header.
    pub section_type: SectionType,

    /// Attributes of the section.
    pub flags: SectionFlags,

    /// Virtual address of the section in memory, for sections that are loaded.
    pub virtual_address: u64,

    /// Offset of the section in the file image.
    pub file_offset: u64,

    /// Size in bytes of the section in the file image. May be 0.
    pub file_size: u64,

    /// Section index of an associated section.
    ///
    /// This field is used for several purposes, depending on the type of the
    /// section.
    pub section_link: u32,

    /// Extra information about the section.
    ///
    /// This field is used for several purposes, depending on the type of the
    /// section.
    pub section_info: u32,

    /// Alignment of the section.
    ///
    /// This field must be a power of two.
    pub alignment: u64,

    /// Contains the size, in bytes, of each entry for sections that contains
    /// fixed size entries.
    ///
    /// If the section does not contain fixed size entries, this field may be
    /// zero.
    pub entry_size: u64,
}

impl SectionHeader {
    pub(super) fn new<'elf>(
        elf: &'elf ReadElf<'elf>,
        index: u16,
        strings: Option<&StringSection<'elf>>,
    ) -> Option<SectionHeader> {
        // Ensure that the base offset of the section doesn't overflow. The
        // `BinParser` will always check that the offsets are within the file.
        // The `SectionHeaders`` class has already checked the validity of the
        // `section_header_size` field meets the minimum required.
        //
        // Note, here `offset` can't overflow. Multiplication of two 16-bit
        // values results in a 32-bit value.
        let offset = (index as u64) * (elf.section_header_size as u64);
        let base = elf.section_header_offset.checked_add(offset)?;
        if base > u64::MAX - elf.section_header_size as u64 {
            return None;
        }

        let name = match strings {
            Some(section) => {
                let name_offset = elf.parser.get_u32(base, elf.data)?;
                section.to_string(name_offset)
            }
            None => None,
        };

        match elf.class {
            Class::Elf32 => Some(SectionHeader {
                name,
                section_type: SectionType::from(elf.parser.get_u32(base + 4, elf.data)?),
                flags: SectionFlags::from(elf.parser.get_u32(base + 8, elf.data)? as u64),
                virtual_address: elf.parser.get_u32(base + 12, elf.data)? as u64,
                file_offset: elf.parser.get_u32(base + 16, elf.data)? as u64,
                file_size: elf.parser.get_u32(base + 20, elf.data)? as u64,
                section_link: elf.parser.get_u32(base + 24, elf.data)?,
                section_info: elf.parser.get_u32(base + 28, elf.data)?,
                alignment: elf.parser.get_u32(base + 32, elf.data)? as u64,
                entry_size: elf.parser.get_u32(base + 36, elf.data)? as u64,
            }),
            Class::Elf64 => Some(SectionHeader {
                name,
                section_type: SectionType::from(elf.parser.get_u32(base + 4, elf.data)?),
                flags: SectionFlags::from(elf.parser.get_u64(base + 8, elf.data)?),
                virtual_address: elf.parser.get_u64(base + 16, elf.data)?,
                file_offset: elf.parser.get_u64(base + 24, elf.data)?,
                file_size: elf.parser.get_u64(base + 32, elf.data)?,
                section_link: elf.parser.get_u32(base + 40, elf.data)?,
                section_info: elf.parser.get_u32(base + 44, elf.data)?,
                alignment: elf.parser.get_u64(base + 48, elf.data)?,
                entry_size: elf.parser.get_u64(base + 56, elf.data)?,
            }),
        }
    }

    /// Check if the alignment is correct for a loadable section.
    pub fn is_aligned(&self) -> bool {
        self.alignment == 0 || self.alignment == 1 || self.alignment.is_power_of_two()
    }
}
