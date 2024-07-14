use crate::binparser::Buffer;
use crate::{ReadElf, SectionHeader, SectionType};

#[derive(Debug)]
pub(super) struct StringSection<'elf> {
    buffer: Buffer<'elf>,
}

impl<'elf> StringSection<'elf> {
    /// Create a string section from the ELF file using the default string index.
    pub(super) fn new(elf: &'elf ReadElf<'elf>) -> Option<StringSection<'elf>> {
        let index = elf.string_section_index;
        Self::from_index(elf, index)
    }

    /// Create a string section from the ELF file using the specified section index.
    pub(super) fn from_index(elf: &'elf ReadElf<'elf>, index: u16) -> Option<StringSection<'elf>> {
        if index > elf.section_header_count {
            None
        } else {
            // Can't reference `elf.section_headers().index(index)` as that calls
            // this function which results in a cyclic dependency and ultimately a
            // stack overflow.
            let section = SectionHeader::new(elf, index, None)?;
            if section.file_size > usize::MAX as u64
                || u32::from(section.section_type) != u32::from(SectionType::StrTab)
            {
                None
            } else {
                let buffer: Buffer<'elf> = elf
                    .parser
                    .get_map(section.file_offset, section.file_size as usize)?;
                Some(StringSection { buffer })
            }
        }
    }

    /// Get the string at the offset `index`.
    ///
    /// A string section is a collection of NUL terminated strings. Get the
    /// string at offset `index`, which is expected to be UTF-8 encoded.
    pub(super) fn to_string(&self, index: u32) -> Option<String> {
        let i = index as usize;
        let bytes = &self.buffer.buffer()[i..];
        let cstr = std::ffi::CStr::from_bytes_until_nul(bytes).ok()?;
        let str = cstr.to_str().ok()?;
        Some(str.to_owned())
    }
}
