use std::fmt;

/// The ABI for the target Operating System.
///
/// To create an instance of [OsAbi], use the `from` method. You can use one of
/// the constants define, or any [u8].
///
/// Refer to your Operating System documentation on the exact value that it
/// supports.
///
/// # Example
/// ```rust
/// use readelf::OsAbi;
///
/// let o = OsAbi::from(OsAbi::NONE);
/// println!("{:?}", o);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OsAbi {
    os_abi: u8,
}

impl OsAbi {
    /// Unspecified.
    pub const NONE: u8 = 0;

    /// System V.
    pub const SYSV: u8 = 0;

    /// HP-UX.
    pub const HPUX: u8 = 1;

    /// NetBSD.
    pub const NETBSD: u8 = 2;

    /// GNU Linux.
    pub const LINUX: u8 = 3;

    /// GNU Hurd.
    pub const GNUHURD: u8 = 4;

    /// Solaris.
    pub const SOLARIS: u8 = 6;

    /// IBM AIX (Monterey).
    pub const AIX: u8 = 7;

    /// IRIX.
    pub const IRIX: u8 = 8;

    /// FreeBSD.
    pub const FREEBSD: u8 = 9;

    /// Tru64.
    pub const TRU64: u8 = 10;

    /// Novell Modesto.
    pub const MODESTO: u8 = 11;

    /// OpenBSD.
    pub const OPENBSD: u8 = 12;

    /// OpenVMS.
    pub const OPENVMS: u8 = 13;

    /// NonStop Kernel.
    pub const NSK: u8 = 14;

    /// AROS Research Operating System.
    pub const AROS: u8 = 15;

    /// FenixOS.
    pub const FENIXOS: u8 = 16;

    /// Nuxi CloudABI.
    pub const CLOUDABI: u8 = 17;

    /// OpenVOS.
    pub const OPENVOS: u8 = 18;

    /// AMD Mesa3D Runtime.
    pub const AMDGPU_MESA3D: u8 = 66;

    /// ARM.
    ///
    /// According to [binutils
    /// source](https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=elfcpp/elfcpp.h;h=f2fe7330f7c0cd60ecfba5bdc7d77091d896da93;hb=HEAD#l146)
    /// this is a GNU extension.
    pub const ARM: u8 = 97;

    /// Standolne (embedded) application.
    pub const STANDALONE: u8 = 255;

    /// Get the byte representation of the OS ABI in the ELF file.
    #[must_use]
    pub fn os_abi(&self) -> u8 {
        self.os_abi
    }
}

impl From<u8> for OsAbi {
    #[must_use]
    fn from(v: u8) -> Self {
        OsAbi { os_abi: v }
    }
}

impl From<OsAbi> for u8 {
    #[must_use]
    fn from(v: OsAbi) -> Self {
        v.os_abi
    }
}

impl fmt::Display for OsAbi {
    /// Format the OS ABI into a printable string.
    ///
    /// Values that are generally uncommon or ambiguous are not converted to a
    /// name. Values that are well known are converted to their well known
    /// string. Values that are unknown, or not well-known are converted to `ABI
    /// 0xVV`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.os_abi {
            OsAbi::SYSV => write!(f, "SysV / Not Specified"),
            OsAbi::HPUX => write!(f, "HP-UX"),
            OsAbi::NETBSD => write!(f, "NetBSD"),
            OsAbi::LINUX => write!(f, "Linux"),
            OsAbi::GNUHURD => write!(f, "GNU Hurd"),
            OsAbi::SOLARIS => write!(f, "Solaris"),
            OsAbi::AIX => write!(f, "AIX"),
            OsAbi::IRIX => write!(f, "Irix"),
            OsAbi::FREEBSD => write!(f, "FreeBSD"),
            OsAbi::TRU64 => write!(f, "Tru64"),
            OsAbi::MODESTO => write!(f, "Novell Modesto"),
            OsAbi::OPENBSD => write!(f, "OpenBSD"),
            OsAbi::OPENVMS => write!(f, "OpenVMS"),
            OsAbi::NSK => write!(f, "NonStop Kernel"),
            OsAbi::AROS => write!(f, "Amiga Research Operating System"),
            OsAbi::FENIXOS => write!(f, "FenixOS"),
            OsAbi::CLOUDABI => write!(f, "Nuxi CloudABI"),
            OsAbi::OPENVOS => write!(f, "OpenVOS"),
            OsAbi::ARM => write!(f, "ARM"),
            OsAbi::STANDALONE => write!(f, "Standalone (embedded)"),
            _ => write!(f, "ABI 0x{:0>2X}", self.os_abi),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OsAbi;

    #[test]
    fn abi_string() {
        assert_eq!(OsAbi::from(OsAbi::NONE).to_string(), "SysV / Not Specified");
        assert_eq!(OsAbi::from(OsAbi::SYSV).to_string(), "SysV / Not Specified");
        assert_eq!(OsAbi::from(OsAbi::HPUX).to_string(), "HP-UX");
        assert_eq!(OsAbi::from(OsAbi::NETBSD).to_string(), "NetBSD");
        assert_eq!(OsAbi::from(OsAbi::LINUX).to_string(), "Linux");
        assert_eq!(OsAbi::from(OsAbi::GNUHURD).to_string(), "GNU Hurd");
        assert_eq!(OsAbi::from(OsAbi::SOLARIS).to_string(), "Solaris");
        assert_eq!(OsAbi::from(OsAbi::AIX).to_string(), "AIX");
        assert_eq!(OsAbi::from(OsAbi::IRIX).to_string(), "Irix");
        assert_eq!(OsAbi::from(OsAbi::FREEBSD).to_string(), "FreeBSD");
        assert_eq!(OsAbi::from(OsAbi::TRU64).to_string(), "Tru64");
        assert_eq!(OsAbi::from(OsAbi::MODESTO).to_string(), "Novell Modesto");
        assert_eq!(OsAbi::from(OsAbi::OPENBSD).to_string(), "OpenBSD");
        assert_eq!(OsAbi::from(OsAbi::OPENVMS).to_string(), "OpenVMS");
        assert_eq!(OsAbi::from(OsAbi::NSK).to_string(), "NonStop Kernel");
        assert_eq!(
            OsAbi::from(OsAbi::AROS).to_string(),
            "Amiga Research Operating System"
        );
        assert_eq!(OsAbi::from(OsAbi::FENIXOS).to_string(), "FenixOS");
        assert_eq!(OsAbi::from(OsAbi::CLOUDABI).to_string(), "Nuxi CloudABI");
        assert_eq!(OsAbi::from(OsAbi::OPENVOS).to_string(), "OpenVOS");
        assert_eq!(OsAbi::from(OsAbi::ARM).to_string(), "ARM");
        assert_eq!(
            OsAbi::from(OsAbi::STANDALONE).to_string(),
            "Standalone (embedded)"
        );
        assert_eq!(OsAbi::from(5).to_string(), "ABI 0x05");
        assert_eq!(OsAbi::from(64).to_string(), "ABI 0x40");
        assert_eq!(OsAbi::from(65).to_string(), "ABI 0x41");
        assert_eq!(OsAbi::from(66).to_string(), "ABI 0x42");
        assert_eq!(OsAbi::from(254).to_string(), "ABI 0xFE");
    }

    #[test]
    fn from_integer() {
        let abi = OsAbi::from(OsAbi::LINUX);

        let v: u8 = abi.into();
        assert_eq!(v, OsAbi::LINUX);

        assert_eq!(abi.os_abi(), OsAbi::LINUX);
    }
}
