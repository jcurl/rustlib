use std::fmt;

/// The flags associated with a section in the program header.
///
/// To create an instance of [SectionFlags], use the `from` method. You can use
/// one of the constants define, or any [u32].
///
/// # Example
/// ```rust
/// use readelf::SectionFlags;
///
/// let f = SectionFlags::from(SectionFlags::WRITE + SectionFlags::ALLOC);
/// println!("{:?}", f.to_string());
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SectionFlags {
    flags: u64,
}

impl SectionFlags {
    /// No flags are set.
    pub const NONE: u64 = 0;

    /// Writable.
    pub const WRITE: u64 = 0x00000001;

    /// Occupies memory during execution.
    pub const ALLOC: u64 = 0x00000002;

    /// Executable.
    pub const EXECINSTR: u64 = 0x00000004;

    /// Might be merged.
    pub const MERGE: u64 = 0x00000010;

    /// Contains null-terminated strings.
    pub const STRINGS: u64 = 0x00000020;

    /// 'sh_info' contains SHT index.
    pub const INFO_LINK: u64 = 0x00000040;

    /// Preserve order after combining.
    pub const LINK_ORDER: u64 = 0x00000080;

    /// Non-standard OS specific handling required.
    pub const OS_NONCONFORMING: u64 = 0x00000100;

    /// Section is member of a group.
    pub const GROUP: u64 = 0x00000200;

    /// Section holds thread-local data.
    pub const TLS: u64 = 0x00000400;

    /// OS specific mask.
    pub const MASKOS: u64 = 0x0FF00000;

    /// Processor specific mask.
    pub const MASKPROC: u64 = 0xF0000000;

    /// Get the byte representation of the OS ABI in the ELF file.
    #[must_use]
    pub const fn flags(&self) -> u64 {
        self.flags
    }
}

impl From<u64> for SectionFlags {
    #[must_use]
    fn from(v: u64) -> Self {
        SectionFlags { flags: v }
    }
}

impl From<SectionFlags> for u64 {
    #[must_use]
    fn from(v: SectionFlags) -> Self {
        v.flags
    }
}

fn append(s: &mut String, v: &str) {
    if !s.is_empty() {
        s.push_str(" | ");
    }
    s.push_str(v);
}

impl fmt::Display for SectionFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::default();
        let mut flag = self.flags;

        if self.flags == 0 {
            write!(f, "NONE")
        } else {
            if self.flags & SectionFlags::WRITE != 0 {
                append(&mut result, "SHF_WRITE");
                flag ^= SectionFlags::WRITE;
            }
            if self.flags & SectionFlags::ALLOC != 0 {
                append(&mut result, "SHF_ALLOC");
                flag ^= SectionFlags::ALLOC;
            }
            if self.flags & SectionFlags::EXECINSTR != 0 {
                append(&mut result, "SHF_EXECINSTR");
                flag ^= SectionFlags::EXECINSTR;
            }
            if self.flags & SectionFlags::MERGE != 0 {
                append(&mut result, "SHF_MERGE");
                flag ^= SectionFlags::MERGE;
            }
            if self.flags & SectionFlags::STRINGS != 0 {
                append(&mut result, "SHF_STRINGS");
                flag ^= SectionFlags::STRINGS;
            }
            if self.flags & SectionFlags::INFO_LINK != 0 {
                append(&mut result, "SHF_INFO_LINK");
                flag ^= SectionFlags::INFO_LINK;
            }
            if self.flags & SectionFlags::LINK_ORDER != 0 {
                append(&mut result, "SHF_LINK_ORDER");
                flag ^= SectionFlags::LINK_ORDER;
            }
            if self.flags & SectionFlags::OS_NONCONFORMING != 0 {
                append(&mut result, "SHF_OS_NONCONFORMING");
                flag ^= SectionFlags::OS_NONCONFORMING;
            }
            if self.flags & SectionFlags::GROUP != 0 {
                append(&mut result, "SHF_GROUP");
                flag ^= SectionFlags::GROUP;
            }
            if self.flags & SectionFlags::TLS != 0 {
                append(&mut result, "SHF_TLS");
                flag ^= SectionFlags::TLS;
            }
            if self.flags & SectionFlags::MASKOS != 0 {
                append(
                    &mut result,
                    &format!(
                        "SHF_MASKOS({:02X})",
                        (self.flags & SectionFlags::MASKOS) >> 20
                    ),
                );
                flag &= !SectionFlags::MASKOS;
            }
            if self.flags & SectionFlags::MASKPROC != 0 {
                append(
                    &mut result,
                    &format!(
                        "SHF_MASKPROC({:X})",
                        (self.flags & SectionFlags::MASKPROC) >> 28
                    ),
                );
                flag &= !SectionFlags::MASKPROC;
            }
            if flag != 0 {
                append(&mut result, format!("0x{:X}", flag).as_str());
            }

            write!(f, "{}", result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SectionFlags;

    #[test]
    fn flags_string() {
        assert_eq!(SectionFlags::from(SectionFlags::NONE).to_string(), "NONE");
        assert_eq!(
            SectionFlags::from(SectionFlags::WRITE).to_string(),
            "SHF_WRITE"
        );
        assert_eq!(
            SectionFlags::from(SectionFlags::ALLOC).to_string(),
            "SHF_ALLOC"
        );
        assert_eq!(
            SectionFlags::from(SectionFlags::EXECINSTR).to_string(),
            "SHF_EXECINSTR"
        );
        assert_eq!(
            SectionFlags::from(SectionFlags::MERGE).to_string(),
            "SHF_MERGE"
        );
        assert_eq!(
            SectionFlags::from(SectionFlags::STRINGS).to_string(),
            "SHF_STRINGS"
        );
        assert_eq!(
            SectionFlags::from(SectionFlags::INFO_LINK).to_string(),
            "SHF_INFO_LINK"
        );
        assert_eq!(
            SectionFlags::from(SectionFlags::LINK_ORDER).to_string(),
            "SHF_LINK_ORDER"
        );
        assert_eq!(
            SectionFlags::from(SectionFlags::OS_NONCONFORMING).to_string(),
            "SHF_OS_NONCONFORMING"
        );
        assert_eq!(
            SectionFlags::from(SectionFlags::GROUP).to_string(),
            "SHF_GROUP"
        );
        assert_eq!(SectionFlags::from(SectionFlags::TLS).to_string(), "SHF_TLS");
        assert_eq!(SectionFlags::from(3).to_string(), "SHF_WRITE | SHF_ALLOC");
        assert_eq!(SectionFlags::from(8).to_string(), "0x8");
        assert_eq!(SectionFlags::from(9).to_string(), "SHF_WRITE | 0x8");
        assert_eq!(
            SectionFlags::from(0x109).to_string(),
            "SHF_WRITE | SHF_OS_NONCONFORMING | 0x8"
        );
        assert_eq!(
            SectionFlags::from(0xFFFFFFFF).to_string(),
            "SHF_WRITE | SHF_ALLOC | SHF_EXECINSTR | SHF_MERGE | SHF_STRINGS | SHF_INFO_LINK | SHF_LINK_ORDER | SHF_OS_NONCONFORMING | SHF_GROUP | SHF_TLS | SHF_MASKOS(FF) | SHF_MASKPROC(F) | 0xFF808"
        );
    }

    #[test]
    fn from_integer() {
        let flags = SectionFlags::from(SectionFlags::WRITE);

        let v: u64 = flags.into();
        assert_eq!(v, SectionFlags::WRITE);

        assert_eq!(flags.flags(), SectionFlags::WRITE);
    }
}
