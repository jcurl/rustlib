use std::fmt;

/// The executable type of the ELF file.
///
/// # Example
///
/// Create the enum via the generic [ExecutableType::from] method. The
/// conversion will always work.
///
/// ```rust
/// use readelf::ExecutableType;
///
/// let e = ExecutableType::from(1);
/// println!("{:?}", e);
/// ```
///
/// You can convert the enum back to the value for the ELF file
///
/// ```rust
/// use readelf::ExecutableType;
///
/// let e = ExecutableType::from(1);
/// let v: u16 = e.into();
/// println!("EF_REL has value {}", v);
/// ```
///
/// # Handling Unknown Types
///
/// If an unknown executable type is found in the ELF file, the value is given
/// the variant `Unknown`. If in the future this value is defined, it will lead
/// a new enum variant, and no longer being part of the `Unknown` variant. To
/// keep your software forward compatible, you should not match to the `Unknown`
/// variant, and instead capture all that is not known and convert to an
/// integer, then checking the value. For example
///
/// ```rust
/// use readelf::ExecutableType;
/// let e = ExecutableType::from(10);
///
/// match e {
///   ExecutableType::Executable => println!("Executable"),
///   ExecutableType::Dynamic => println!("Dynamic"),
///   _ => {
///     // Note, we don't match Unknown(v) here.
///     let v = u16::from(e);
///     if v == 4 {
///       println!("Core");
///     } else {
///       println!("Unknown {}", v);
///     }
///   }
/// }
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum ExecutableType {
    /// Unknown executable type.
    None = 0,

    /// Relocatable file.
    Relocatable = 1,

    /// Executable file.
    Executable = 2,

    /// Shared object.
    Dynamic = 3,

    /// Core file.
    Core = 4,

    /// Operating specific type.
    ///
    /// This may be a value defined by the operating system vendor, or be
    /// processor specific, usually in the range of
    /// [ExecutableType::LOOS]..=[ExecutableType::HIOS] or
    /// [ExecutableType::LOPROC]..=[ExecutableType::HIPROC].
    Unknown(u16),
}

impl ExecutableType {
    /// Reserved range, lower value for operating system specific executable types.
    pub const LOOS: u16 = 0xFE00;

    /// Reserved range, higher value for operating system specific executable types.
    pub const HIOS: u16 = 0xFEFF;

    /// Reserved range, lower value for processor specific executable types.
    pub const LOPROC: u16 = 0xFF00;

    /// Reserved range, high value for processor specific executable types.
    pub const HIPROC: u16 = 0xFFFF;
}

impl From<u16> for ExecutableType {
    fn from(v: u16) -> ExecutableType {
        match v {
            0 => ExecutableType::None,
            1 => ExecutableType::Relocatable,
            2 => ExecutableType::Executable,
            3 => ExecutableType::Dynamic,
            4 => ExecutableType::Core,
            _ => ExecutableType::Unknown(v),
        }
    }
}

impl From<ExecutableType> for u16 {
    fn from(v: ExecutableType) -> u16 {
        match v {
            ExecutableType::None => 0,
            ExecutableType::Relocatable => 1,
            ExecutableType::Executable => 2,
            ExecutableType::Dynamic => 3,
            ExecutableType::Core => 4,
            ExecutableType::Unknown(v) => v,
        }
    }
}

impl fmt::Display for ExecutableType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutableType::None => write!(f, "None"),
            ExecutableType::Relocatable => write!(f, "Relocatable"),
            ExecutableType::Executable => write!(f, "Executable"),
            ExecutableType::Dynamic => write!(f, "Shared"),
            ExecutableType::Core => write!(f, "Core"),
            ExecutableType::Unknown(v) => write!(f, "Type 0x{v:0>4X}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ExecutableType;

    #[test]
    fn from_value() {
        assert_eq!(ExecutableType::from(0), ExecutableType::None);
        assert_eq!(ExecutableType::from(1), ExecutableType::Relocatable);
        assert_eq!(ExecutableType::from(2), ExecutableType::Executable);
        assert_eq!(ExecutableType::from(3), ExecutableType::Dynamic);
        assert_eq!(ExecutableType::from(4), ExecutableType::Core);
        assert_eq!(ExecutableType::from(5), ExecutableType::Unknown(5));
        assert_eq!(ExecutableType::from(0xFF), ExecutableType::Unknown(0xFF));
        assert_eq!(
            ExecutableType::from(0xFFFF),
            ExecutableType::Unknown(0xFFFF)
        );
    }

    #[test]
    fn from_enum() {
        assert_eq!(u16::from(ExecutableType::None), 0);
        assert_eq!(u16::from(ExecutableType::Relocatable), 1);
        assert_eq!(u16::from(ExecutableType::Executable), 2);
        assert_eq!(u16::from(ExecutableType::Dynamic), 3);
        assert_eq!(u16::from(ExecutableType::Core), 4);
        assert_eq!(u16::from(ExecutableType::Unknown(5)), 5);
        assert_eq!(u16::from(ExecutableType::Unknown(0xFF)), 0xFF);
        assert_eq!(u16::from(ExecutableType::Unknown(0xFFFF)), 0xFFFF);
    }

    #[test]
    fn executable_type_to_string() {
        assert_eq!(ExecutableType::None.to_string(), "None");
        assert_eq!(ExecutableType::Relocatable.to_string(), "Relocatable");
        assert_eq!(ExecutableType::Executable.to_string(), "Executable");
        assert_eq!(ExecutableType::Dynamic.to_string(), "Shared");
        assert_eq!(ExecutableType::Core.to_string(), "Core");
        assert_eq!(ExecutableType::Unknown(5).to_string(), "Type 0x0005");
        assert_eq!(ExecutableType::Unknown(0xFF).to_string(), "Type 0x00FF");
        assert_eq!(ExecutableType::Unknown(0xFFFF).to_string(), "Type 0xFFFF");
    }
}
