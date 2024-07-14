use std::fmt;

/// The section type of a program header in the ELF file.
///
/// # Example
///
/// Create the enum via the generic [SectionType::from] method. The
/// conversion will always work.
///
/// ```rust
/// use readelf::SectionType;
///
/// let e = SectionType::from(5);
/// println!("{:?}", e);
/// ```
///
/// You can convert the enum back to the value for the ELF file
///
/// ```rust
/// use readelf::SectionType;
///
/// let e = SectionType::from(5);
/// let v: u32 = e.into();
/// println!("SHT_RELA has value {}", v);
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
/// use readelf::SectionType;
/// let s = SectionType::from(10);
///
/// match s {
///   SectionType::Null => println!("Null"),
///   SectionType::Dynamic => println!("Dynamic"),
///   _ => {
///     // Note, we don't match Unknown(v) here.
///     let v = u32::from(s);
///     if v == 5 {
///       println!("Shared Library Section (Reserved)");
///     } else {
///       println!("Unknown {}", v);
///     }
///   }
/// }
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum SectionType {
    /// Section header table entry is unused.
    Null = 0,

    /// Program data.
    ProgBits = 1,

    /// Symbol table.
    SymTab = 2,

    /// String table.
    StrTab = 3,

    /// Relcation entries with addends.
    RelA = 4,

    /// Symbol hash table.
    Hash = 5,

    /// Dynamic linking information.
    Dynamic = 6,

    /// Notes.
    Note = 7,

    /// Program space with no data (BSS).
    NoBits = 8,

    /// Relocation entries, no addends.
    Rel = 9,

    /// Reserved.
    ShLib = 10,

    /// Dynamic linker symbol table.
    DynSym = 11,

    /// Array of constructors.
    InitArray = 14,

    /// Array of destructors.
    FiniArray = 15,

    /// Array of pre-constructors.
    PreInitArray = 16,

    /// Section group.
    Group = 17,

    /// Extended section indices.
    SymTabIndex = 18,

    /// Unknown section type.
    ///
    /// Don't ever match this type, instead convert to a [u32] and then check
    /// the value:
    ///
    /// ```rust
    /// # use readelf::SectionType;
    /// let s = u32::from(SectionType::Unknown(8));
    /// match s {
    ///     10 => { println!("ShLib"); }
    ///     _ => { println!("Other: {}", s); }
    /// }
    /// ```
    ///
    /// This is to allow new values of [SectionType] to be added, and if your
    /// code used the [SectionType::Unknown] variant, it would no longer match
    /// if it were defined in a newer library.
    Unknown(u32),
}

impl From<u32> for SectionType {
    fn from(v: u32) -> SectionType {
        match v {
            0 => SectionType::Null,
            1 => SectionType::ProgBits,
            2 => SectionType::SymTab,
            3 => SectionType::StrTab,
            4 => SectionType::RelA,
            5 => SectionType::Hash,
            6 => SectionType::Dynamic,
            7 => SectionType::Note,
            8 => SectionType::NoBits,
            9 => SectionType::Rel,
            10 => SectionType::ShLib,
            11 => SectionType::DynSym,
            14 => SectionType::InitArray,
            15 => SectionType::FiniArray,
            16 => SectionType::PreInitArray,
            17 => SectionType::Group,
            18 => SectionType::SymTabIndex,
            _ => SectionType::Unknown(v),
        }
    }
}

impl From<SectionType> for u32 {
    fn from(v: SectionType) -> u32 {
        match v {
            SectionType::Null => 0,
            SectionType::ProgBits => 1,
            SectionType::SymTab => 2,
            SectionType::StrTab => 3,
            SectionType::RelA => 4,
            SectionType::Hash => 5,
            SectionType::Dynamic => 6,
            SectionType::Note => 7,
            SectionType::NoBits => 8,
            SectionType::Rel => 9,
            SectionType::ShLib => 10,
            SectionType::DynSym => 11,
            SectionType::InitArray => 14,
            SectionType::FiniArray => 15,
            SectionType::PreInitArray => 16,
            SectionType::Group => 17,
            SectionType::SymTabIndex => 18,
            SectionType::Unknown(v) => v,
        }
    }
}

impl fmt::Display for SectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Don't explicitly use the variant, so Unknown(x) will map to the
        // correct name.
        let v = u32::from(*self);
        match v {
            0 => write!(f, "Null"),
            1 => write!(f, "Program data"),
            2 => write!(f, "Symbol table"),
            3 => write!(f, "String table"),
            4 => write!(f, "Relocation Addends"),
            5 => write!(f, "Symbol hash table"),
            6 => write!(f, "Dynamic linking"),
            7 => write!(f, "Notes"),
            8 => write!(f, "Program no data"),
            9 => write!(f, "Relocation"),
            11 => write!(f, "Dyn. linker symtab"),
            14 => write!(f, "Constructors"),
            15 => write!(f, "Destructors"),
            16 => write!(f, "Pre-constructors"),
            17 => write!(f, "Section group"),
            18 => write!(f, "Ext. section indices"),
            _ => write!(f, "Section 0x{:0>8X}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SectionType;

    #[test]
    fn from_value() {
        assert_eq!(SectionType::from(0), SectionType::Null);
        assert_eq!(SectionType::from(1), SectionType::ProgBits);
        assert_eq!(SectionType::from(2), SectionType::SymTab);
        assert_eq!(SectionType::from(3), SectionType::StrTab);
        assert_eq!(SectionType::from(4), SectionType::RelA);
        assert_eq!(SectionType::from(5), SectionType::Hash);
        assert_eq!(SectionType::from(6), SectionType::Dynamic);
        assert_eq!(SectionType::from(7), SectionType::Note);
        assert_eq!(SectionType::from(8), SectionType::NoBits);
        assert_eq!(SectionType::from(9), SectionType::Rel);
        assert_eq!(SectionType::from(10), SectionType::ShLib);
        assert_eq!(SectionType::from(11), SectionType::DynSym);
        assert_eq!(SectionType::from(12), SectionType::Unknown(12));
        assert_eq!(SectionType::from(13), SectionType::Unknown(13));
        assert_eq!(SectionType::from(14), SectionType::InitArray);
        assert_eq!(SectionType::from(15), SectionType::FiniArray);
        assert_eq!(SectionType::from(16), SectionType::PreInitArray);
        assert_eq!(SectionType::from(17), SectionType::Group);
        assert_eq!(SectionType::from(18), SectionType::SymTabIndex);
        assert_eq!(SectionType::from(19), SectionType::Unknown(19));
        assert_eq!(SectionType::from(0xFF), SectionType::Unknown(0xFF));
        assert_eq!(SectionType::from(0xFFFF), SectionType::Unknown(0xFFFF));
        assert_eq!(
            SectionType::from(0xFFFFFFFF),
            SectionType::Unknown(0xFFFFFFFF)
        );
    }

    #[test]
    fn from_enum() {
        assert_eq!(u32::from(SectionType::Null), 0);
        assert_eq!(u32::from(SectionType::ProgBits), 1);
        assert_eq!(u32::from(SectionType::SymTab), 2);
        assert_eq!(u32::from(SectionType::StrTab), 3);
        assert_eq!(u32::from(SectionType::RelA), 4);
        assert_eq!(u32::from(SectionType::Hash), 5);
        assert_eq!(u32::from(SectionType::Dynamic), 6);
        assert_eq!(u32::from(SectionType::Note), 7);
        assert_eq!(u32::from(SectionType::NoBits), 8);
        assert_eq!(u32::from(SectionType::Rel), 9);
        assert_eq!(u32::from(SectionType::ShLib), 10);
        assert_eq!(u32::from(SectionType::DynSym), 11);
        assert_eq!(u32::from(SectionType::Unknown(12)), 12);
        assert_eq!(u32::from(SectionType::Unknown(13)), 13);
        assert_eq!(u32::from(SectionType::InitArray), 14);
        assert_eq!(u32::from(SectionType::FiniArray), 15);
        assert_eq!(u32::from(SectionType::PreInitArray), 16);
        assert_eq!(u32::from(SectionType::Group), 17);
        assert_eq!(u32::from(SectionType::SymTabIndex), 18);
        assert_eq!(u32::from(SectionType::Unknown(0xFFFF)), 0xFFFF);
        assert_eq!(u32::from(SectionType::Unknown(0xFFFFFFFF)), 0xFFFFFFFF);
    }

    #[test]
    fn segment_type_to_string() {
        assert_eq!(SectionType::Null.to_string(), "Null");
        assert_eq!(SectionType::Unknown(0).to_string(), "Null");
        assert_eq!(SectionType::ProgBits.to_string(), "Program data");
        assert_eq!(SectionType::SymTab.to_string(), "Symbol table");
        assert_eq!(SectionType::StrTab.to_string(), "String table");
        assert_eq!(SectionType::RelA.to_string(), "Relocation Addends");
        assert_eq!(SectionType::Hash.to_string(), "Symbol hash table");
        assert_eq!(SectionType::Dynamic.to_string(), "Dynamic linking");
        assert_eq!(SectionType::Note.to_string(), "Notes");
        assert_eq!(SectionType::NoBits.to_string(), "Program no data");
        assert_eq!(SectionType::Rel.to_string(), "Relocation");
        assert_eq!(SectionType::ShLib.to_string(), "Section 0x0000000A");
        assert_eq!(SectionType::DynSym.to_string(), "Dyn. linker symtab");
        assert_eq!(SectionType::Unknown(12).to_string(), "Section 0x0000000C");
        assert_eq!(SectionType::Unknown(13).to_string(), "Section 0x0000000D");
        assert_eq!(SectionType::InitArray.to_string(), "Constructors");
        assert_eq!(SectionType::FiniArray.to_string(), "Destructors");
        assert_eq!(SectionType::PreInitArray.to_string(), "Pre-constructors");
        assert_eq!(SectionType::Group.to_string(), "Section group");
        assert_eq!(SectionType::SymTabIndex.to_string(), "Ext. section indices");
        assert_eq!(SectionType::Unknown(19).to_string(), "Section 0x00000013");
        assert_eq!(SectionType::Unknown(0xFF).to_string(), "Section 0x000000FF");
        assert_eq!(
            SectionType::Unknown(0xFFFF).to_string(),
            "Section 0x0000FFFF"
        );
        assert_eq!(
            SectionType::Unknown(0x8000FFFF).to_string(),
            "Section 0x8000FFFF"
        );
    }
}
