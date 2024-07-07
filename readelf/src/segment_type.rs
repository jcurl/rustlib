use std::fmt;

/// The segment type of a program header in the ELF file.
///
/// # Example
///
/// Create the enum via the generic [SegmentType::from] method. The
/// conversion will always work.
///
/// ```rust
/// use readelf::SegmentType;
///
/// let e = SegmentType::from(1);
/// println!("{:?}", e);
/// ```
///
/// You can convert the enum back to the value for the ELF file
///
/// ```rust
/// use readelf::SegmentType;
///
/// let e = SegmentType::from(1);
/// let v: u32 = e.into();
/// println!("PT_LOAD has value {}", v);
/// ```
///
/// # Handling Unknown Types
///
/// If an unknown segment type is found in the ELF file, the value is given
/// the variant `Unknown`. If in the future this value is defined, it will lead
/// a new enum variant, and no longer being part of the `Unknown` variant. To
/// keep your software forward compatible, you should not match to the `Unknown`
/// variant, and instead capture all that is not known and convert to an
/// integer, then checking the value. For example
///
/// ```rust
/// use readelf::SegmentType;
/// let s = SegmentType::from(10);
///
/// match s {
///   SegmentType::Null => println!("Null"),
///   SegmentType::Dynamic => println!("Dynamic"),
///   _ => {
///     // Note, we don't match Unknown(v) here.
///     let v = u32::from(s);
///     if v == 5 {
///       println!("Shared Library Header (Reserved)");
///     } else {
///       println!("Unknown {}", v);
///     }
///   }
/// }
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum SegmentType {
    /// Program header table entry is unused.
    Null = 0,

    /// Loadable segment.
    Load = 1,

    /// Dynamic linking information.
    Dynamic = 2,

    /// Interpreter information.
    Interpreter = 3,

    /// Auxiliary information.
    Note = 4,

    /// Reserved.
    ShLib = 5,

    /// Segment containing program header table itself.
    ///
    /// This points to the ELF header given by `e_phoff`.
    ProgramHeader = 6,

    /// Thread-Local storage template.
    ThreadLocalStorage = 7,

    /// Unknown segment type.
    ///
    /// Don't ever match this type, instead convert to a [u32] and then check
    /// the value:
    ///
    /// ```rust
    /// # use readelf::SegmentType;
    /// let s = u32::from(SegmentType::Unknown(8));
    /// match s {
    ///     5 => { println!("ShLib"); }
    ///     _ => { println!("Other: {}", s); }
    /// }
    /// ```
    ///
    /// This is to allow new values of [SegmentType] to be added, and if your
    /// code used the [SegmentType::Unknown] variant, it would no longer match
    /// if it were defined in a newer library.
    Unknown(u32),
}

impl From<u32> for SegmentType {
    fn from(v: u32) -> SegmentType {
        match v {
            0 => SegmentType::Null,
            1 => SegmentType::Load,
            2 => SegmentType::Dynamic,
            3 => SegmentType::Interpreter,
            4 => SegmentType::Note,
            5 => SegmentType::ShLib,
            6 => SegmentType::ProgramHeader,
            7 => SegmentType::ThreadLocalStorage,
            _ => SegmentType::Unknown(v),
        }
    }
}

impl From<SegmentType> for u32 {
    fn from(v: SegmentType) -> u32 {
        match v {
            SegmentType::Null => 0,
            SegmentType::Load => 1,
            SegmentType::Dynamic => 2,
            SegmentType::Interpreter => 3,
            SegmentType::Note => 4,
            SegmentType::ShLib => 5,
            SegmentType::ProgramHeader => 6,
            SegmentType::ThreadLocalStorage => 7,
            SegmentType::Unknown(v) => v,
        }
    }
}

impl fmt::Display for SegmentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Don't explicitly use the variant, so Unknown(x) will map to the
        // correct name.
        let v = u32::from(*self);
        match v {
            0 => write!(f, "Null"),
            1 => write!(f, "Loadable Segment"),
            2 => write!(f, "Dynamic Linking"),
            3 => write!(f, "Interpreter"),
            4 => write!(f, "Note"),
            6 => write!(f, "Program Header"),
            7 => write!(f, "Thread Local Storage"),
            _ => write!(f, "Segment 0x{:0>8X}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SegmentType;

    #[test]
    fn from_value() {
        assert_eq!(SegmentType::from(0), SegmentType::Null);
        assert_eq!(SegmentType::from(1), SegmentType::Load);
        assert_eq!(SegmentType::from(2), SegmentType::Dynamic);
        assert_eq!(SegmentType::from(3), SegmentType::Interpreter);
        assert_eq!(SegmentType::from(4), SegmentType::Note);
        assert_eq!(SegmentType::from(5), SegmentType::ShLib);
        assert_eq!(SegmentType::from(6), SegmentType::ProgramHeader);
        assert_eq!(SegmentType::from(7), SegmentType::ThreadLocalStorage);
        assert_eq!(SegmentType::from(8), SegmentType::Unknown(8));
        assert_eq!(SegmentType::from(0xFF), SegmentType::Unknown(0xFF));
        assert_eq!(SegmentType::from(0xFFFF), SegmentType::Unknown(0xFFFF));
        assert_eq!(
            SegmentType::from(0xFFFFFFFF),
            SegmentType::Unknown(0xFFFFFFFF)
        );
    }

    #[test]
    fn from_enum() {
        assert_eq!(u32::from(SegmentType::Null), 0);
        assert_eq!(u32::from(SegmentType::Load), 1);
        assert_eq!(u32::from(SegmentType::Dynamic), 2);
        assert_eq!(u32::from(SegmentType::Interpreter), 3);
        assert_eq!(u32::from(SegmentType::Note), 4);
        assert_eq!(u32::from(SegmentType::ShLib), 5);
        assert_eq!(u32::from(SegmentType::Unknown(5)), 5);
        assert_eq!(u32::from(SegmentType::ProgramHeader), 6);
        assert_eq!(u32::from(SegmentType::ThreadLocalStorage), 7);
        assert_eq!(u32::from(SegmentType::Unknown(0xFFFF)), 0xFFFF);
        assert_eq!(u32::from(SegmentType::Unknown(0xFFFFFFFF)), 0xFFFFFFFF);
    }

    #[test]
    fn segment_type_to_string() {
        assert_eq!(SegmentType::Null.to_string(), "Null");
        assert_eq!(SegmentType::Unknown(0).to_string(), "Null");
        assert_eq!(SegmentType::Load.to_string(), "Loadable Segment");
        assert_eq!(SegmentType::Dynamic.to_string(), "Dynamic Linking");
        assert_eq!(SegmentType::Interpreter.to_string(), "Interpreter");
        assert_eq!(SegmentType::Note.to_string(), "Note");
        assert_eq!(SegmentType::ProgramHeader.to_string(), "Program Header");
        assert_eq!(
            SegmentType::ThreadLocalStorage.to_string(),
            "Thread Local Storage"
        );
        assert_eq!(SegmentType::ShLib.to_string(), "Segment 0x00000005");
        assert_eq!(SegmentType::Unknown(0xFF).to_string(), "Segment 0x000000FF");
        assert_eq!(
            SegmentType::Unknown(0xFFFF).to_string(),
            "Segment 0x0000FFFF"
        );
        assert_eq!(
            SegmentType::Unknown(0x8000FFFF).to_string(),
            "Segment 0x8000FFFF"
        );
    }
}
