use std::fmt;

/// The Endianness defined in the ELF file.
///
/// # Example
///
/// Create the enum via the generic [Endian::try_from] method.
///
/// ```rust
/// use readelf::Endian;
///
/// let e = Endian::try_from(1).unwrap();
/// println!("{:?}", e);
/// ```
///
/// You can convert the enum back to the value for the ELF file
///
/// ```rust
/// use readelf::Endian;
///
/// let e = Endian::try_from(1).unwrap();
/// let v: u8 = e.into();
/// println!("Little Endian has value {}", v);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Endian {
    /// Little Endian format, where the first byte is the lowest order byte.
    Little = 1,

    /// Big Endian format, where the first byte is the highest order byte.
    Big = 2,
}

impl TryFrom<u8> for Endian {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Endian::Little),
            2 => Ok(Endian::Big),
            _ => Err(()),
        }
    }
}

impl From<Endian> for u8 {
    fn from(v: Endian) -> u8 {
        v as u8
    }
}

impl fmt::Display for Endian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Endian::Little => write!(f, "Little Endian"),
            Endian::Big => write!(f, "Big Endian"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Endian;

    #[test]
    fn try_from_little_endian() {
        let v = Endian::try_from(1).unwrap();
        assert_eq!(v, Endian::Little);

        let u: u8 = v.into();
        assert_eq!(u, 1);
    }

    #[test]
    fn try_from_big_endian() {
        let v = Endian::try_from(2).unwrap();
        assert_eq!(v, Endian::Big);

        let u: u8 = v.into();
        assert_eq!(u, 2);
    }

    #[test]
    fn try_from_unknown_endian() {
        let v = Endian::try_from(0);
        assert!(v.is_err());
    }

    #[test]
    fn endian_to_string() {
        let b = Endian::Big;
        assert_eq!(b.to_string(), "Big Endian");

        let l = Endian::Little;
        assert_eq!(l.to_string(), "Little Endian");
    }
}
