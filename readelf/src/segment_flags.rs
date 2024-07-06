use std::fmt;

/// The flags associated with a segment in the program header.
///
/// To create an instance of [SegmentFlags], use the `from` method. You can use
/// one of the constants define, or any [u32].
///
/// # Example
/// ```rust
/// use readelf::SegmentFlags;
///
/// let f = SegmentFlags::from(SegmentFlags::R + SegmentFlags::X);
/// println!("{:?}", f.to_string());
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SegmentFlags {
    flags: u32,
}

impl SegmentFlags {
    /// No flags are set.
    pub const NONE: u32 = 0;

    /// Executable segment.
    pub const X: u32 = 1;

    /// Writable segment.
    pub const W: u32 = 2;

    /// Readable segment.
    pub const R: u32 = 4;

    /// Get the byte representation of the OS ABI in the ELF file.
    #[must_use]
    pub fn flags(&self) -> u32 {
        self.flags
    }
}

impl From<u32> for SegmentFlags {
    #[must_use]
    fn from(v: u32) -> Self {
        SegmentFlags { flags: v }
    }
}

impl From<SegmentFlags> for u32 {
    #[must_use]
    fn from(v: SegmentFlags) -> Self {
        v.flags
    }
}

fn append(s: &mut String, v: &str) {
    if !s.is_empty() {
        s.push_str(" | ");
    }
    s.push_str(v);
}

impl fmt::Display for SegmentFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::default();
        let mut flag = self.flags;

        if self.flags == 0 {
            write!(f, "NONE")
        } else {
            if self.flags & SegmentFlags::X != 0 {
                append(&mut result, "PF_X");
                flag ^= SegmentFlags::X;
            }
            if self.flags & SegmentFlags::W != 0 {
                append(&mut result, "PF_W");
                flag ^= SegmentFlags::W;
            }
            if self.flags & SegmentFlags::R != 0 {
                append(&mut result, "PF_R");
                flag ^= SegmentFlags::R;
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
    use super::SegmentFlags;

    #[test]
    fn flags_string() {
        assert_eq!(SegmentFlags::from(SegmentFlags::NONE).to_string(), "NONE");
        assert_eq!(SegmentFlags::from(SegmentFlags::X).to_string(), "PF_X");
        assert_eq!(SegmentFlags::from(SegmentFlags::W).to_string(), "PF_W");
        assert_eq!(SegmentFlags::from(SegmentFlags::R).to_string(), "PF_R");
        assert_eq!(SegmentFlags::from(3).to_string(), "PF_X | PF_W");
        assert_eq!(SegmentFlags::from(5).to_string(), "PF_X | PF_R");
        assert_eq!(SegmentFlags::from(6).to_string(), "PF_W | PF_R");
        assert_eq!(SegmentFlags::from(7).to_string(), "PF_X | PF_W | PF_R");
        assert_eq!(SegmentFlags::from(8).to_string(), "0x8");
        assert_eq!(SegmentFlags::from(9).to_string(), "PF_X | 0x8");
        assert_eq!(
            SegmentFlags::from(15).to_string(),
            "PF_X | PF_W | PF_R | 0x8"
        );
        assert_eq!(
            SegmentFlags::from(0xFF07).to_string(),
            "PF_X | PF_W | PF_R | 0xFF00"
        );
    }

    #[test]
    fn from_integer() {
        let flags = SegmentFlags::from(SegmentFlags::X);

        let v: u32 = flags.into();
        assert_eq!(v, SegmentFlags::X);

        assert_eq!(flags.flags(), SegmentFlags::X);
    }
}
