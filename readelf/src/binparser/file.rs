use super::{BinParser, Endian};
use std::cell::RefCell;
use std::io::*;
use std::path::Path;

pub(crate) struct File {
    elf: RefCell<std::fs::File>, // A file must be mutable.
}

impl File {
    pub(crate) fn open<P: AsRef<Path>>(path: P) -> Option<File> {
        let elf_file = std::fs::File::open(path).ok()?;
        Some(File {
            elf: RefCell::new(elf_file),
        })
    }
}

impl BinParser for File {
    #[inline(always)]
    fn get_u8(&self, offset: u64) -> Option<u8> {
        let mut elf = self.elf.borrow_mut();
        elf.seek(SeekFrom::Start(offset)).ok()?;

        let mut buff = [0; 1];
        elf.read_exact(&mut buff).ok()?;

        Some(buff[0])
    }

    #[inline(always)]
    fn get_u16(&self, offset: u64, e: Endian) -> Option<u16> {
        let mut elf = self.elf.borrow_mut();
        elf.seek(SeekFrom::Start(offset)).ok()?;

        let mut buff = [0; 2];
        elf.read_exact(&mut buff).ok()?;

        match e {
            Endian::Little => Some(u16::from_le_bytes(buff)),
            Endian::Big => Some(u16::from_be_bytes(buff)),
        }
    }

    #[inline(always)]
    fn get_u32(&self, offset: u64, e: Endian) -> Option<u32> {
        let mut elf = self.elf.borrow_mut();
        elf.seek(SeekFrom::Start(offset)).ok()?;

        let mut buff = [0; 4];
        elf.read_exact(&mut buff).ok()?;

        match e {
            Endian::Little => Some(u32::from_le_bytes(buff)),
            Endian::Big => Some(u32::from_be_bytes(buff)),
        }
    }

    #[inline(always)]
    fn get_u64(&self, offset: u64, e: Endian) -> Option<u64> {
        let mut elf = self.elf.borrow_mut();
        elf.seek(SeekFrom::Start(offset)).ok()?;

        let mut buff = [0; 8];
        elf.read_exact(&mut buff).ok()?;

        match e {
            Endian::Little => Some(u64::from_le_bytes(buff)),
            Endian::Big => Some(u64::from_be_bytes(buff)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BinParser, Endian, File};
    use std::path::PathBuf;

    fn test_resource_path(path: &str) -> PathBuf {
        let paths = path.split('/');
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources");
        d.push("tests");

        for path in paths {
            d.push(path);
        }
        d
    }

    #[test]
    fn test_get_u8() {
        let buffer = File::open(test_resource_path("elf/debian-9.13.0-i386-netinst/bash")).unwrap();
        assert_eq!(buffer.get_u8(0), Some(0x7f));
        assert_eq!(buffer.get_u8(1), Some(0x45));
        assert_eq!(buffer.get_u8(2), Some(0x4c));
        assert_eq!(buffer.get_u8(3), Some(0x46));
        assert_eq!(buffer.get_u8(63), Some(0x08));
        assert_eq!(buffer.get_u8(64), None); // File is exactly 64 bytes large
        assert_eq!(buffer.get_u8(u64::MAX), None);
    }

    #[test]
    fn test_get_u16() {
        let buffer = File::open(test_resource_path("elf/debian-9.13.0-i386-netinst/bash")).unwrap();
        assert_eq!(buffer.get_u16(0, Endian::Little), Some(0x457f));
        assert_eq!(buffer.get_u16(1, Endian::Little), Some(0x4c45));
        assert_eq!(buffer.get_u16(62, Endian::Little), Some(0x0804));
        assert_eq!(buffer.get_u16(63, Endian::Little), None);
        assert_eq!(buffer.get_u16(64, Endian::Little), None);
        assert_eq!(buffer.get_u16(u64::MAX, Endian::Little), None);
        assert_eq!(buffer.get_u16(u64::MAX - 1, Endian::Little), None);

        assert_eq!(buffer.get_u16(0, Endian::Big), Some(0x7f45));
        assert_eq!(buffer.get_u16(1, Endian::Big), Some(0x454c));
        assert_eq!(buffer.get_u16(62, Endian::Big), Some(0x0408));
        assert_eq!(buffer.get_u16(63, Endian::Big), None);
        assert_eq!(buffer.get_u16(64, Endian::Big), None);
        assert_eq!(buffer.get_u16(u64::MAX, Endian::Big), None);
        assert_eq!(buffer.get_u16(u64::MAX - 1, Endian::Big), None);
    }

    #[test]
    fn test_get_u32() {
        let buffer = File::open(test_resource_path("elf/debian-9.13.0-i386-netinst/bash")).unwrap();
        assert_eq!(buffer.get_u32(0, Endian::Little), Some(0x464c457f));
        assert_eq!(buffer.get_u32(1, Endian::Little), Some(0x01464c45));
        assert_eq!(buffer.get_u32(60, Endian::Little), Some(0x08048034));
        assert_eq!(buffer.get_u32(61, Endian::Little), None);
        assert_eq!(buffer.get_u32(64, Endian::Little), None);
        assert_eq!(buffer.get_u32(u64::MAX, Endian::Little), None);
        assert_eq!(buffer.get_u32(u64::MAX - 1, Endian::Little), None);

        assert_eq!(buffer.get_u32(0, Endian::Big), Some(0x7f454c46));
        assert_eq!(buffer.get_u32(1, Endian::Big), Some(0x454c4601));
        assert_eq!(buffer.get_u32(60, Endian::Big), Some(0x34800408));
        assert_eq!(buffer.get_u32(61, Endian::Big), None);
        assert_eq!(buffer.get_u32(64, Endian::Big), None);
        assert_eq!(buffer.get_u32(u64::MAX, Endian::Big), None);
        assert_eq!(buffer.get_u32(u64::MAX - 1, Endian::Big), None);
    }

    #[test]
    fn test_get_u64() {
        let buffer = File::open(test_resource_path("elf/debian-9.13.0-i386-netinst/bash")).unwrap();
        assert_eq!(buffer.get_u64(0, Endian::Little), Some(0x00010101464c457f));
        assert_eq!(buffer.get_u64(1, Endian::Little), Some(0x0000010101464c45));
        assert_eq!(buffer.get_u64(56, Endian::Little), Some(0x0804803400000034));
        assert_eq!(buffer.get_u64(57, Endian::Little), None);
        assert_eq!(buffer.get_u64(64, Endian::Little), None);
        assert_eq!(buffer.get_u64(u64::MAX, Endian::Little), None);
        assert_eq!(buffer.get_u64(u64::MAX - 1, Endian::Little), None);

        assert_eq!(buffer.get_u64(0, Endian::Big), Some(0x7f454c4601010100));
        assert_eq!(buffer.get_u64(1, Endian::Big), Some(0x454c460101010000));
        assert_eq!(buffer.get_u64(56, Endian::Big), Some(0x3400000034800408));
        assert_eq!(buffer.get_u64(57, Endian::Big), None);
        assert_eq!(buffer.get_u64(64, Endian::Big), None);
        assert_eq!(buffer.get_u64(u64::MAX, Endian::Big), None);
        assert_eq!(buffer.get_u64(u64::MAX - 1, Endian::Big), None);
    }
}
