use std::fmt;

/// The Class of the ELF file.
///
/// The class of the ELF file defines the layout of the ELF structure on disk.
/// It is either 32-bit or 64-bit.
///
/// # Example
///
/// Create the enum via the generic [Class::try_from] method.
///
/// ```rust
/// use readelf::Class;
///
/// let e = Class::try_from(1).unwrap();
/// println!("{:?}", e);
/// ```
///
/// You can convert the enum back to the value for the ELF file.
///
/// ```rust
/// use readelf::Class;
///
/// let e = Class::try_from(1).unwrap();
/// let v: u8 = e.into();
/// println!("32-bit has value {}", v);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Class {
    /// 32-bit format.
    ///
    /// The original format of ELF files, where pointers in the ELF files to
    /// sections are represented using 32-bit values.
    Elf32 = 1,

    /// 64-bit format.
    ///
    /// ELF files where memory entry points, and offsets within the ELF file are
    /// represented as 64-bit values.
    Elf64 = 2,
}

impl TryFrom<u8> for Class {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Class::Elf32),
            2 => Ok(Class::Elf64),
            _ => Err(()),
        }
    }
}

impl From<Class> for u8 {
    fn from(v: Class) -> u8 {
        v as u8
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Class::Elf32 => write!(f, "32-bit ELF"),
            Class::Elf64 => write!(f, "64-bit ELF"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Class;

    #[test]
    fn try_from_32bit() {
        let v = Class::try_from(1).unwrap();
        assert_eq!(v, Class::Elf32);

        let u: u8 = v.into();
        assert_eq!(u, 1);
    }

    #[test]
    fn try_from_64bit() {
        let v = Class::try_from(2).unwrap();
        assert_eq!(v, Class::Elf64);

        let u: u8 = v.into();
        assert_eq!(u, 2);
    }

    #[test]
    fn try_from_unknown_class() {
        let v = Class::try_from(0);
        assert!(v.is_err());
    }

    #[test]
    fn class_to_string() {
        let b = Class::Elf64;
        assert_eq!(b.to_string(), "64-bit ELF");

        let l = Class::Elf32;
        assert_eq!(l.to_string(), "32-bit ELF");
    }
}
