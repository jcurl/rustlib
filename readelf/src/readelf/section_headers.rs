use crate::{Class, ReadElf, SectionHeader};

use super::string_section::StringSection;

/// An iterator for all program headers in the ELF file.
#[derive(Debug)]
pub struct SectionHeaders<'elf> {
    elf: &'elf ReadElf<'elf>,
    strings: Option<StringSection<'elf>>,
    index: u16,
}

impl<'elf> SectionHeaders<'elf> {
    /// Create a new iterator for ELF segments.
    pub(super) fn new(elf: &'elf ReadElf<'elf>) -> SectionHeaders<'elf> {
        let min_shentsize = match elf.class {
            Class::Elf32 => 40_u16,
            Class::Elf64 => 64_u16,
        };

        let i = if elf.section_header_size < min_shentsize {
            // Don't iterate, by indicating we're at the end.
            elf.section_header_count
        } else {
            0
        };

        let strings = StringSection::new(elf);

        SectionHeaders {
            elf,
            strings,
            index: i,
        }
    }

    /// Get the expected number of section header sections.
    ///
    /// This is always the value of `e_shnum`, regardless if there that many
    /// readable sections or not.
    pub fn len(&self) -> usize {
        self.elf.section_header_count as usize
    }

    /// Check if there are no sections in the ELF file.
    ///
    /// This is always checked against `e_shnum`, regardless if there are that
    /// many readable sections or not.
    pub fn is_empty(&self) -> bool {
        self.elf.section_header_count == 0
    }

    /// Get the [SectionHeader] at the specified index.
    ///
    /// Index directly into the ELF file to get the associated [SectionHeader].
    /// The value of `index` must be in the range of 0 to
    /// [SectionHeaders::len()]. If it isn't, then [Option::None] is returned.
    /// If the index is in the range, a value of [Option::None] may still be
    /// returned in case there is a problem with the file (corruption, or
    /// truncated).
    ///
    /// Note that the `std::ops::Index` trait is not implemented, as the
    /// [SectionHeader] is created lazily from the ELF file.
    pub fn index(&self, index: usize) -> Option<SectionHeader> {
        if index >= self.elf.section_header_count as usize {
            None
        } else {
            // Can't truncate, because it must be smaller than
            // `program_header_count`.
            SectionHeader::new(self.elf, index as u16, self.strings.as_ref())
        }
    }
}

impl<'elf> Iterator for SectionHeaders<'elf> {
    type Item = SectionHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.elf.section_header_count {
            return None;
        }

        let section = SectionHeader::new(self.elf, self.index, self.strings.as_ref());
        match section {
            Some(_) => {
                self.index += 1;
            }
            None => {
                self.index = self.elf.section_header_count;
            }
        };
        section
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.elf.section_header_count as usize))
    }
}
