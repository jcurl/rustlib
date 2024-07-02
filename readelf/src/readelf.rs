use crate::binparser;
use crate::{Class, Endian, ExecutableType, Machine, OsAbi};
use std::fmt;
use std::path::Path;

/// Properties of an ELF file when loaded into memory.
///
/// The methods for this class read the source lazily. It will only access the
/// elements in the file when they're needed.
pub struct ReadElf<'elf> {
    /// The bitness (class) of the ELF file, used to interpret the ELF file
    /// layout.
    ///
    /// This value represents `e_ident[EI_CLASS]`.
    pub class: Class,

    /// The Endianness of the ELF file when interpreting the contents.
    ///
    /// This value represents `e_ident[EI_DATA]`.
    pub data: Endian,

    /// The ELF version, which is expected to always be `1`.
    ///
    /// This value represents `e_ident[EI_VERSION] or e_version`.
    pub version: u32,

    /// The OS ABI specification for the ELF file, interpreted by the OS.
    ///
    /// This value represents `e_ident[EI_OSABI]`.
    pub osabi: OsAbi,

    /// The ABI version.
    ///
    /// Further specifies the ABI version. Its interpretation depends on the
    /// target ABI. Linux kernel (after at least 2.6) has no definition of it.
    /// Glibc 2.12+ in case of [ReadElf.osabi] being [OsAbi::LINUX] treats this
    /// field as ABI version of the dynamic linker.
    ///
    /// This value represents `e_ident[EI_ABIVERSION]`.
    pub abi_version: u8,

    /// The object file type.
    ///
    /// This value represents `e_type`.
    pub exec_type: ExecutableType,

    /// The machine instruction set architecture.
    ///
    /// This value represents `e_machine`.
    pub machine: Machine,

    /// The entry point where the process starts executing.
    ///
    /// This value represents `e_entry`.
    pub entry: u64,

    /// Flags of the ELF file dependent on the target architecture.
    ///
    /// This value represents `e_flags`.
    pub flags: u32,

    // Private fields
    /// Contains the size of this header, normally 64 Bytes for 64-bit and 52
    /// Bytes for 32-bit format.
    ///
    /// This value represents `e_ehsize`.
    file_header_size: u16,

    /// Points to the start of the program header table.
    ///
    /// It usually follows the file header immediately following this one,
    /// making the offset 0x34 or 0x40 for 32- and 64-bit ELF executables,
    /// respectively.
    ///
    /// This value represents `e_phoff`.
    program_header_offset: u64,

    /// Contains the size of a program header table entry.
    ///
    /// This value represents `e_phentsize`.
    program_header_size: u16,

    /// Contains the number of entries in the program header table.
    ///
    /// This value represents `e_phnum`.
    program_header_count: u16,

    /// Points to the start of the section header table.
    ///
    /// This value represents `e_shoff`.
    section_header_offset: u64,

    /// Contains the size of a section header table entry.
    ///
    /// This value represents `e_shentsize`.
    section_header_size: u16,

    /// Contains the number of entries in the section header table.
    ///
    /// This value represents `e_shnum`.
    section_header_count: u16,

    /// Contains index of the section header table entry that contains the
    /// section names.
    ///
    /// This value repesents `e_shstrndx`.
    string_section_index: u16,

    parser: Box<dyn binparser::BinParser + 'elf>,
}

impl<'elf> fmt::Debug for ReadElf<'elf> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Must implement this manually, as the `#[derive(Debug)]` doesn't work
        // because of the field `parser`. When we had this implementation on
        // BinParser:
        //
        // impl fmt::Debug for dyn BinParser {
        //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //         f.debug_struct("BinParser").finish()
        //     }
        // }
        //
        // we get the error
        //
        // error: lifetime may not live long enough
        //    --> src/readelf.rs:106:5
        //     |
        // 6   | #[derive(Debug)]
        //     |          ----- in this derive macro expansion
        // 7   | pub struct ReadElf<'elf> {
        //     |                    ---- lifetime `'elf` defined here
        // ...
        // 106 |     parser: Box<dyn binparser::BinParser + 'elf>,
        //     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cast requires that `'elf` must outlive `'static`
        //     |
        //     = note: this error originates in the derive macro `Debug` (in Nightly builds, run with -Z macro-backtrace for more info)

        f.debug_struct("ReadElf")
            .field("class", &self.class)
            .field("data", &self.data)
            .field("version", &self.version)
            .field("osabi", &self.osabi.to_string())
            .field("abi_version", &self.abi_version)
            .field("exec_type", &self.exec_type.to_string())
            .field("machine", &self.machine.to_string())
            .field("entry", &format_args!("0x{0:0>16x}", &self.entry))
            .field("flags", &format_args!("0x{0:0>8x}", &self.flags))
            .finish()
    }
}

impl<'elf> ReadElf<'elf> {
    /// A convenience function to get the offset in an ELF struct.
    ///
    /// Get a 32-bit offset, or a 64-bit offset into the ELF struct depending on
    /// the [Class].
    const fn offset(c: Class, o32: u64, o64: u64) -> u64 {
        match c {
            Class::Elf32 => o32,
            Class::Elf64 => o64,
        }
    }

    fn from_parser<T>(p: Box<T>) -> Option<ReadElf<'elf>>
    where
        T: binparser::BinParser + 'elf,
    {
        // The signature of the ELF must file be 0x7F ELF.
        if p.get_u8(0)? != 0x7F
            || p.get_u8(1)? != 0x45
            || p.get_u8(2)? != 0x4C
            || p.get_u8(3)? != 0x46
        {
            return None;
        }

        // The endianness is needed often when interpreting.
        let e = Endian::try_from(p.get_u8(5)?).ok()?;

        // We only support Version 1 when reading.
        if p.get_u8(6)? != 1 || p.get_u32(20, e)? != 1 {
            return None;
        }

        // The class tells us how to interpret the byte offsets.
        let c = Class::try_from(p.get_u8(4)?).ok()?;

        Some(ReadElf::<'elf> {
            class: c,
            data: e,
            version: 1,
            osabi: OsAbi::from(p.get_u8(7)?),
            abi_version: p.get_u8(8)?,
            exec_type: ExecutableType::from(p.get_u16(16, e)?),
            machine: Machine::from(p.get_u16(18, e)?),
            entry: p.get_usize(24, e, c)?,
            flags: p.get_u32(ReadElf::offset(c, 36, 48), e)?,
            file_header_size: p.get_u16(ReadElf::offset(c, 40, 52), e)?,
            program_header_offset: p.get_usize(ReadElf::offset(c, 28, 32), e, c)?,
            program_header_size: p.get_u16(ReadElf::offset(c, 42, 54), e)?,
            program_header_count: p.get_u16(ReadElf::offset(c, 44, 56), e)?,
            section_header_offset: p.get_usize(ReadElf::offset(c, 32, 40), e, c)?,
            section_header_size: p.get_u16(ReadElf::offset(c, 46, 58), e)?,
            section_header_count: p.get_u16(ReadElf::offset(c, 48, 60), e)?,
            string_section_index: p.get_u16(ReadElf::offset(c, 50, 62), e)?,
            parser: p,
        })
    }

    /// Interpret the ELF file from a buffer slice in memory.
    ///
    /// This method is useful if you have guarantees over the lifetime of the
    /// ELF file, that it lasts longer than the [ReadElf] object you get back.
    pub fn from_slice(buffer: &'elf [u8]) -> Option<ReadElf<'elf>> {
        let p = Box::new(binparser::Slice::<'elf>::new(buffer));
        Self::from_parser(p)
    }

    /// Interpret the ELF file from a buffer in memory.
    ///
    /// This method takes ownership of the buffer and encapsulates the buffer on
    /// the heap inside the [ReadElf] object.
    pub fn from_vec(buffer: Vec<u8>) -> Option<ReadElf<'elf>> {
        let p = Box::new(binparser::VecBuffer::new(buffer));
        Self::from_parser(p)
    }

    /// Interpret the ELF file from disk.
    ///
    /// This method opens the file on disk and uses seeks to access the file.
    /// This allows to open very large ELF files also on 32-bit systems.
    pub fn open<P: AsRef<Path>>(path: P) -> Option<ReadElf<'elf>> {
        let p = Box::new(binparser::File::open(path)?);
        Self::from_parser(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_resource_path(paths: &[&str]) -> PathBuf {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources");
        d.push("tests");

        for path in paths {
            d.push(path);
        }
        d
    }

    fn test_resource<'elf>(paths: &[&str]) -> ReadElf<'elf> {
        let path = test_resource_path(paths);
        let file_data = std::fs::read(path).unwrap();

        ReadElf::from_vec(file_data).unwrap()
    }

    #[test]
    fn powerpc_exe_bash_slice() {
        let path = test_resource_path(&["elf", "debian-8.11.0-powerpc-netinst", "bash"]);
        let file_data = std::fs::read(path).unwrap();
        let slice = file_data.as_slice();
        let r = ReadElf::from_slice(slice).unwrap();

        assert_eq!(r.class, Class::Elf32);
        assert_eq!(r.data, Endian::Big);
        assert_eq!(r.version, 1);
        assert_eq!(r.osabi.os_abi(), 0);
        assert_eq!(r.abi_version, 0);
        assert_eq!(r.exec_type, ExecutableType::Executable);
        assert_eq!(r.machine.machine(), Machine::PPC);
        assert_eq!(r.entry, 0x1001ABC8);
        assert_eq!(r.flags, 0x00000000);
    }

    #[test]
    fn powerpc_exe_bash_vec() {
        let r = test_resource(&["elf", "debian-8.11.0-powerpc-netinst", "bash"]);

        assert_eq!(r.class, Class::Elf32);
        assert_eq!(r.data, Endian::Big);
        assert_eq!(r.version, 1);
        assert_eq!(r.osabi.os_abi(), 0);
        assert_eq!(r.abi_version, 0);
        assert_eq!(r.exec_type, ExecutableType::Executable);
        assert_eq!(r.machine.machine(), Machine::PPC);
        assert_eq!(r.entry, 0x1001ABC8);
        assert_eq!(r.flags, 0x00000000);
    }

    #[test]
    fn powerpc_exe_bash_file() {
        let path = test_resource_path(&["elf", "debian-8.11.0-powerpc-netinst", "bash"]);
        let r = ReadElf::open(path).unwrap();

        assert_eq!(r.class, Class::Elf32);
        assert_eq!(r.data, Endian::Big);
        assert_eq!(r.version, 1);
        assert_eq!(r.osabi.os_abi(), 0);
        assert_eq!(r.abi_version, 0);
        assert_eq!(r.exec_type, ExecutableType::Executable);
        assert_eq!(r.machine.machine(), Machine::PPC);
        assert_eq!(r.entry, 0x1001ABC8);
        assert_eq!(r.flags, 0x00000000);
    }
}
