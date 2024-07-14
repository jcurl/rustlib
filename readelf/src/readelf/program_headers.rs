use crate::{Class, ProgramHeader, ReadElf};

/// An iterator for all program headers in the ELF file.
#[derive(Debug)]
pub struct ProgramHeaders<'elf> {
    elf: &'elf ReadElf<'elf>,
    index: u16,
}

impl<'elf> ProgramHeaders<'elf> {
    /// Create a new iterator for ELF segments.
    pub(super) fn new(elf: &'elf ReadElf<'elf>) -> ProgramHeaders<'elf> {
        let min_phentsize = match elf.class {
            Class::Elf32 => 32_u16,
            Class::Elf64 => 56_u16,
        };

        let i = if elf.program_header_size < min_phentsize {
            // Don't iterate, by indicating we're at the end.
            elf.program_header_count
        } else {
            0
        };

        ProgramHeaders { elf, index: i }
    }

    /// Get the expected number of program header segments.
    ///
    /// This is always the value of `e_phnum`, regardless if there that many
    /// readable segments or not.
    pub fn len(&self) -> usize {
        self.elf.program_header_count as usize
    }

    /// Check if there are no segments in the ELF file.
    ///
    /// This is always checked against `e_phnum`, regardless if there are that
    /// many readable segments or not.
    pub fn is_empty(&self) -> bool {
        self.elf.program_header_count == 0
    }

    /// Get the [ProgramHeader] at the specified index.
    ///
    /// Index directly into the ELF file to get the associated [ProgramHeader].
    /// The value of `index` must be in the range of 0 to
    /// [ProgramHeaders::len()]. If it isn't, then [Option::None] is returned.
    /// If the index is in the range, a value of [Option::None] may still be
    /// returned in case there is a problem with the file (corruption, or
    /// truncated).
    ///
    /// Note that the `std::ops::Index` trait is not implemented, as the
    /// [ProgramHeader] is created lazily from the ELF file.
    pub fn index(&self, index: usize) -> Option<ProgramHeader> {
        if index >= self.elf.program_header_count as usize {
            None
        } else {
            // Can't truncate, because it must be smaller than
            // `program_header_count`.
            ProgramHeader::new(self.elf, index as u16)
        }
    }
}

impl<'elf> Iterator for ProgramHeaders<'elf> {
    type Item = ProgramHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.elf.program_header_count {
            return None;
        }

        let segment = ProgramHeader::new(self.elf, self.index);
        match segment {
            Some(_) => {
                self.index += 1;
            }
            None => {
                self.index = self.elf.program_header_count;
            }
        };
        segment
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.elf.program_header_count as usize))
    }
}
