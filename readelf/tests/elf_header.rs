//! Test suite reading in various different ELF file formats.
//!
//! Files for testing are found in the `resources/tests` folder.

use readelf::ReadElf;
use std::fs::File;
use std::io::BufReader;

mod common;
use common::config::{self, ElfHeaders};

#[test]
fn read_elf_headers() {
    let config_path = config::test_resource_path("test_files.json");
    let config_file = File::open(config_path).unwrap();
    let config_read = BufReader::new(config_file);
    let config: ElfHeaders = serde_json::from_reader(config_read).unwrap();

    for elf in config.elf_headers.iter() {
        println!("Testing: {}", &elf.path);
        let elf_file = config::load_elf_file_vec(&elf.path);

        assert_eq!(elf_file.version, elf.version, "Version mismatch");
        assert_eq!(elf_file.class, elf.class.0, "Class mismatch");
        assert_eq!(elf_file.data, elf.data.0, "Endian mismatch");
        assert_eq!(elf_file.osabi.to_string(), elf.osabi, "OS ABI mismatch");
        assert_eq!(
            elf_file.abi_version, elf.abi_version,
            "ABI Version mismatch"
        );
        assert_eq!(elf_file.exec_type, elf.exec_type.0, "ExecType mismatch");
        assert_eq!(
            elf_file.machine.to_string(),
            elf.machine,
            "Machine mismatch"
        );
        assert_eq!(
            elf_file.flags, elf.flags,
            "Flags mismatch {:x} {:x}",
            elf_file.flags, elf.flags
        );

        // Load using a file for all the variants and check that they're the
        // same as above.
        let elf_file2 = config::load_elf_file(&elf.path);
        assert_eq!(elf_file2.version, elf_file.version);
        assert_eq!(elf_file2.class, elf_file.class);
        assert_eq!(elf_file2.data, elf_file.data);
        assert_eq!(elf_file2.osabi, elf_file.osabi);
        assert_eq!(elf_file2.version, elf_file.version);
        assert_eq!(elf_file2.exec_type, elf_file.exec_type);
        assert_eq!(elf_file2.flags, elf_file.flags);
    }
}

fn get_header_32() -> Vec<u8> {
    let buff: Vec<u8> = vec![
        0x7f, 0x45, 0x4c, 0x46, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x02, 0x00, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x8e, 0x55, 0x06, 0x08, 0x34, 0x00,
        0x00, 0x00, 0xc8, 0x49, 0x13, 0x00, 0x00, 0x00, 0x00, 0x00, 0x34, 0x00, 0x20, 0x00, 0x09,
        0x00, 0x28, 0x00, 0x1e, 0x00, 0x1d, 0x00, 0x06, 0x00, 0x00, 0x00, 0x34, 0x00, 0x00, 0x00,
        0x34, 0x80, 0x04, 0x08,
    ];
    buff
}

fn get_header_64() -> Vec<u8> {
    let buff: Vec<u8> = vec![
        0x7f, 0x45, 0x4c, 0x46, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x02, 0x00, 0xb7, 0x00, 0x01, 0x00, 0x00, 0x00, 0x58, 0x5c, 0x42, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd8, 0x79, 0x0f, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x38, 0x00, 0x08, 0x00, 0x40, 0x00,
        0x1c, 0x00, 0x1b, 0x00,
    ];
    buff
}

#[test]
fn get_header_precondition() {
    let buff = get_header_64();
    let slice = buff.as_slice();

    // Check that our test actually works. Later when we test that it fails,
    // it's assumed it worked prior.
    let elf_file = ReadElf::from_slice(slice);
    assert!(elf_file.is_some());
}

#[test]
fn no_magic_elf_header() {
    let mut buff = get_header_64();
    buff[0] = 0;

    let elf_file = ReadElf::from_vec(buff);
    assert!(elf_file.is_none());
}

#[test]
fn invalid_class() {
    for i in 0..=255_u8 {
        if i == 0 || i > 2 {
            let mut buff = get_header_64();

            // Class (32-bit/64-bit) is offset 4.
            buff[4] = i;

            let elf_file = ReadElf::from_vec(buff);
            assert!(elf_file.is_none());
        }
    }
}

#[test]
fn invalid_data() {
    for i in 0..=255_u8 {
        if i == 0 || i > 2 {
            let mut buff = get_header_64();

            // Data (endianness) is offset 5.
            buff[5] = i;

            let elf_file = ReadElf::from_vec(buff);
            assert!(elf_file.is_none());
        }
    }
}

#[test]
fn invalid_version_ident() {
    for i in 0..=255_u8 {
        if i != 1 {
            let mut buff = get_header_64();

            // Version is offset 6.
            buff[6] = i;

            let elf_file = ReadElf::from_vec(buff);
            assert!(elf_file.is_none());
        }
    }
}

#[test]
fn invalid_version() {
    for p in vec![20, 23].into_iter() {
        for i in 0..=255_u8 {
            if i != 1 {
                let mut buff = get_header_64();

                buff[20] = 0; // Ensure that when p != 20 test case works
                buff[p] = i;

                let elf_file = ReadElf::from_vec(buff);
                assert!(elf_file.is_none(), "Offset:{} Value:{}", p, i);
            }
        }
    }
}

#[test]
fn invalid_version_both() {
    for i in 0..=255_u8 {
        if i != 1 {
            let mut buff = get_header_64();

            // Version (endianness) is offset 6.
            buff[6] = i;
            buff[20] = i;

            let elf_file = ReadElf::from_vec(buff);
            assert!(elf_file.is_none());
        }
    }
}

#[test]
fn all_osabi() {
    for i in 0..=255_u8 {
        let mut buff = get_header_64();

        // All values of osabi should work.
        buff[7] = i;

        let elf_file = ReadElf::from_vec(buff);
        assert!(elf_file.is_some());
    }
}

#[test]
fn all_abi_version() {
    for i in 0..=255_u8 {
        let mut buff = get_header_64();

        // All values of abi_version should work.
        buff[8] = i;

        let elf_file = ReadElf::from_vec(buff);
        assert!(elf_file.is_some());
    }
}

#[test]
fn all_type() {
    for i in 0..=65535_u16 {
        let mut buff = get_header_64();

        // Data (endianness) is offset 5.
        buff[16] = (i & 0xFF) as u8;
        buff[17] = (i >> 8) as u8;

        let elf_file = ReadElf::from_vec(buff);
        assert!(elf_file.is_some(), "Type:{:x}", i);
    }
}

#[test]
fn all_machine() {
    for i in 0..=65535_u16 {
        let mut buff = get_header_64();

        // All values of abi_version should work.
        buff[18] = (i & 0xFF) as u8;
        buff[19] = (i >> 8) as u8;

        let elf_file = ReadElf::from_vec(buff);
        assert!(elf_file.is_some());
    }
}

#[test]
fn zero_length_file() {
    let buff: Vec<u8> = vec![];
    let elf_file = ReadElf::from_vec(buff);
    assert!(elf_file.is_none());
}

#[test]
fn zero_file_small() {
    let buff: Vec<u8> = vec![0, 0, 0, 0];
    for i in 0..4 {
        let elf_file = ReadElf::from_slice(&buff.as_slice()[0..i]);
        assert!(elf_file.is_none(), "Valid file with length {}", i);
    }
}

#[test]
fn very_small_file_32() {
    let buff = get_header_32();
    for i in 0..52 {
        let elf_file = ReadElf::from_slice(&buff.as_slice()[0..i]);
        assert!(elf_file.is_none(), "Valid file with length {}", i);
    }
}

#[test]
fn very_small_file_64() {
    let buff = get_header_64();
    for i in 0..64 {
        let elf_file = ReadElf::from_slice(&buff.as_slice()[0..i]);
        assert!(elf_file.is_none(), "Valid file with length {}", i);
    }
}

#[test]
fn file_nonexistent() {
    let elf_file = ReadElf::open("nonexistent.elf");
    assert!(elf_file.is_none());
}
