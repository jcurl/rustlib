//! Test suite reading in various different ELF file formats.
//!
//! Files for testing are found in the `resources/tests` folder.

use readelf::*;
use std::fs::File;
use std::io::BufReader;

mod common;
use common::builder::{ElfBuilder, ElfBuilder32, ElfBuilder64};
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
fn elf_header_precondition() {
    let buff = get_header_64();
    let slice = buff.as_slice();

    // Check that our test actually works. Later when we test that it fails,
    // it's assumed it worked prior.
    let elf_file = ReadElf::from_slice(slice);
    assert!(elf_file.is_some());
}

#[test]
fn elf_header_no_magic() {
    let mut buff = get_header_64();
    buff[0] = 0;

    let elf_file = ReadElf::from_vec(buff);
    assert!(elf_file.is_none());
}

#[test]
fn elf_header_invalid_class() {
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
fn elf_header_invalid_data() {
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
fn elf_header_invalid_version_ident() {
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
fn elf_header_invalid_version() {
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
fn elf_header_invalid_version_both() {
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
fn elf_header_all_osabi() {
    for i in 0..=255_u8 {
        let mut buff = get_header_64();

        // All values of osabi should work.
        buff[7] = i;

        let elf_file = ReadElf::from_vec(buff);
        assert!(elf_file.is_some());
    }
}

#[test]
fn elf_header_all_abi_version() {
    for i in 0..=255_u8 {
        let mut buff = get_header_64();

        // All values of abi_version should work.
        buff[8] = i;

        let elf_file = ReadElf::from_vec(buff);
        assert!(elf_file.is_some());
    }
}

#[test]
fn elf_header_all_type() {
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
fn elf_header_all_machine() {
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

#[test]
fn segments_elf32_little_program_header_only() {
    let mut elf_builder = ElfBuilder32::new(Endian::Little);
    elf_builder
        .set_os_abi(OsAbi::from(OsAbi::NONE))
        .set_abi_version(0)
        .set_executable_type(ExecutableType::Executable)
        .set_machine(Machine::from(Machine::X86_64))
        .set_entry(0x1000)
        .set_flags(0x00000000);
    let elf = ReadElf::from_slice(elf_builder.buffer()).unwrap();
    assert_eq!(elf.osabi, OsAbi::from(OsAbi::NONE));
    assert_eq!(elf.abi_version, 0);
    assert_eq!(elf.exec_type, ExecutableType::Executable);
    assert_eq!(elf.machine, Machine::from(Machine::X86_64));
    assert_eq!(elf.entry, 0x1000);
    assert_eq!(elf.flags, 0x00000000);
    assert!(elf.program_headers().is_empty());
    assert_eq!(elf.program_headers().len(), 0);
}

#[test]
fn segments_elf32_big_program_header_only() {
    let mut elf_builder = ElfBuilder32::new(Endian::Big);
    elf_builder
        .set_os_abi(OsAbi::from(OsAbi::NONE))
        .set_abi_version(0)
        .set_executable_type(ExecutableType::Executable)
        .set_machine(Machine::from(Machine::X86_64))
        .set_entry(0x1000)
        .set_flags(0x00000000);
    let elf = ReadElf::from_slice(elf_builder.buffer()).unwrap();
    assert_eq!(elf.osabi, OsAbi::from(OsAbi::NONE));
    assert_eq!(elf.abi_version, 0);
    assert_eq!(elf.exec_type, ExecutableType::Executable);
    assert_eq!(elf.machine, Machine::from(Machine::X86_64));
    assert_eq!(elf.entry, 0x1000);
    assert_eq!(elf.flags, 0x00000000);
    assert!(elf.program_headers().is_empty());
    assert_eq!(elf.program_headers().len(), 0);
}

#[test]
fn segments_elf64_little_program_header_only() {
    let mut elf_builder = ElfBuilder64::new(Endian::Little);
    elf_builder
        .set_os_abi(OsAbi::from(OsAbi::NONE))
        .set_abi_version(0)
        .set_executable_type(ExecutableType::Executable)
        .set_machine(Machine::from(Machine::X86_64))
        .set_entry(0x1000)
        .set_flags(0x00000000);
    let elf = ReadElf::from_slice(elf_builder.buffer()).unwrap();
    assert_eq!(elf.osabi, OsAbi::from(OsAbi::NONE));
    assert_eq!(elf.abi_version, 0);
    assert_eq!(elf.exec_type, ExecutableType::Executable);
    assert_eq!(elf.machine, Machine::from(Machine::X86_64));
    assert_eq!(elf.entry, 0x1000);
    assert_eq!(elf.flags, 0x00000000);
    assert!(elf.program_headers().is_empty());
    assert_eq!(elf.program_headers().len(), 0);
}

#[test]
fn segments_elf64_big_program_header_only() {
    let mut elf_builder = ElfBuilder64::new(Endian::Big);
    elf_builder
        .set_os_abi(OsAbi::from(OsAbi::NONE))
        .set_abi_version(0)
        .set_executable_type(ExecutableType::Executable)
        .set_machine(Machine::from(Machine::X86_64))
        .set_entry(0x1000)
        .set_flags(0x00000000);
    let elf = ReadElf::from_slice(elf_builder.buffer()).unwrap();
    assert_eq!(elf.osabi, OsAbi::from(OsAbi::NONE));
    assert_eq!(elf.abi_version, 0);
    assert_eq!(elf.exec_type, ExecutableType::Executable);
    assert_eq!(elf.machine, Machine::from(Machine::X86_64));
    assert_eq!(elf.entry, 0x1000);
    assert_eq!(elf.flags, 0x00000000);
    assert!(elf.program_headers().is_empty());
    assert_eq!(elf.program_headers().len(), 0);
}

#[test]
fn segments_elf32_little_null() {
    let mut elf_builder = ElfBuilder32::new(Endian::Little);
    elf_builder
        .set_os_abi(OsAbi::from(OsAbi::NONE))
        .set_abi_version(0)
        .set_executable_type(ExecutableType::Executable)
        .set_machine(Machine::from(Machine::X86_64))
        .set_entry(0x1000)
        .set_flags(0x00000000);
    elf_builder.add_segment(&ProgramHeader {
        segment_type: SegmentType::Null,
        flags: SegmentFlags::from(SegmentFlags::R),
        file_offset: 0x1000,
        virtual_address: 0x2000,
        physical_address: 0x3000,
        file_size: 0x4000,
        memory_size: 0x5000,
        alignment: 0x10,
    });
    let elf = ReadElf::from_slice(elf_builder.buffer()).unwrap();
    assert!(!elf.program_headers().is_empty());
    assert_eq!(elf.program_headers().len(), 1);

    let segments: Vec<ProgramHeader> = elf.program_headers().collect();
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].segment_type, SegmentType::Null);
    assert_eq!(segments[0].flags, SegmentFlags::from(SegmentFlags::R));
    assert_eq!(segments[0].file_offset, 0x1000);
    assert_eq!(segments[0].virtual_address, 0x2000);
    assert_eq!(segments[0].physical_address, 0x3000);
    assert_eq!(segments[0].file_size, 0x4000);
    assert_eq!(segments[0].memory_size, 0x5000);
    assert_eq!(segments[0].alignment, 0x10);
}

#[test]
fn segments_elf32_big_null() {
    let mut elf_builder = ElfBuilder32::new(Endian::Big);
    elf_builder
        .set_os_abi(OsAbi::from(OsAbi::NONE))
        .set_abi_version(0)
        .set_executable_type(ExecutableType::Executable)
        .set_machine(Machine::from(Machine::X86_64))
        .set_entry(0x1000)
        .set_flags(0x00000000);
    elf_builder.add_segment(&ProgramHeader {
        segment_type: SegmentType::Null,
        flags: SegmentFlags::from(SegmentFlags::R),
        file_offset: 0x1000,
        virtual_address: 0x2000,
        physical_address: 0x3000,
        file_size: 0x4000,
        memory_size: 0x5000,
        alignment: 0x10,
    });
    let elf = ReadElf::from_slice(elf_builder.buffer()).unwrap();
    assert!(!elf.program_headers().is_empty());
    assert_eq!(elf.program_headers().len(), 1);

    let segments: Vec<ProgramHeader> = elf.program_headers().collect();
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].segment_type, SegmentType::Null);
    assert_eq!(segments[0].flags, SegmentFlags::from(SegmentFlags::R));
    assert_eq!(segments[0].file_offset, 0x1000);
    assert_eq!(segments[0].virtual_address, 0x2000);
    assert_eq!(segments[0].physical_address, 0x3000);
    assert_eq!(segments[0].file_size, 0x4000);
    assert_eq!(segments[0].memory_size, 0x5000);
    assert_eq!(segments[0].alignment, 0x10);
}

#[test]
fn segments_elf64_little_null() {
    let mut elf_builder = ElfBuilder64::new(Endian::Little);
    elf_builder
        .set_os_abi(OsAbi::from(OsAbi::NONE))
        .set_abi_version(0)
        .set_executable_type(ExecutableType::Executable)
        .set_machine(Machine::from(Machine::X86_64))
        .set_entry(0x1000)
        .set_flags(0x00000000);
    elf_builder.add_segment(&ProgramHeader {
        segment_type: SegmentType::Null,
        flags: SegmentFlags::from(SegmentFlags::R),
        file_offset: 0x1000,
        virtual_address: 0x2000,
        physical_address: 0x3000,
        file_size: 0x4000,
        memory_size: 0x5000,
        alignment: 0x10,
    });
    let elf = ReadElf::from_slice(elf_builder.buffer()).unwrap();
    assert!(!elf.program_headers().is_empty());
    assert_eq!(elf.program_headers().len(), 1);

    let segments: Vec<ProgramHeader> = elf.program_headers().collect();
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].segment_type, SegmentType::Null);
    assert_eq!(segments[0].flags, SegmentFlags::from(SegmentFlags::R));
    assert_eq!(segments[0].file_offset, 0x1000);
    assert_eq!(segments[0].virtual_address, 0x2000);
    assert_eq!(segments[0].physical_address, 0x3000);
    assert_eq!(segments[0].file_size, 0x4000);
    assert_eq!(segments[0].memory_size, 0x5000);
    assert_eq!(segments[0].alignment, 0x10);
}

#[test]
fn segments_elf64_big_null() {
    let mut elf_builder = ElfBuilder64::new(Endian::Big);
    elf_builder
        .set_os_abi(OsAbi::from(OsAbi::NONE))
        .set_abi_version(0)
        .set_executable_type(ExecutableType::Executable)
        .set_machine(Machine::from(Machine::X86_64))
        .set_entry(0x1000)
        .set_flags(0x00000000);
    elf_builder.add_segment(&ProgramHeader {
        segment_type: SegmentType::Null,
        flags: SegmentFlags::from(SegmentFlags::R),
        file_offset: 0x1000,
        virtual_address: 0x2000,
        physical_address: 0x3000,
        file_size: 0x4000,
        memory_size: 0x5000,
        alignment: 0x10,
    });
    let elf = ReadElf::from_slice(elf_builder.buffer()).unwrap();
    assert!(!elf.program_headers().is_empty());
    assert_eq!(elf.program_headers().len(), 1);

    let segments: Vec<ProgramHeader> = elf.program_headers().collect();
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].segment_type, SegmentType::Null);
    assert_eq!(segments[0].flags, SegmentFlags::from(SegmentFlags::R));
    assert_eq!(segments[0].file_offset, 0x1000);
    assert_eq!(segments[0].virtual_address, 0x2000);
    assert_eq!(segments[0].physical_address, 0x3000);
    assert_eq!(segments[0].file_size, 0x4000);
    assert_eq!(segments[0].memory_size, 0x5000);
    assert_eq!(segments[0].alignment, 0x10);
}

#[test]
fn segments_phoff_umax() {
    let mut elf_builder = ElfBuilder64::new(Endian::Big);
    elf_builder
        .set_os_abi(OsAbi::from(OsAbi::NONE))
        .set_abi_version(0)
        .set_executable_type(ExecutableType::Executable)
        .set_machine(Machine::from(Machine::X86_64))
        .set_entry(0x1000)
        .set_flags(0x00000000);
    elf_builder.add_segment(&ProgramHeader {
        segment_type: SegmentType::Null,
        flags: SegmentFlags::from(SegmentFlags::R),
        file_offset: 0x1000,
        virtual_address: 0x2000,
        physical_address: 0x3000,
        file_size: 0x4000,
        memory_size: 0x5000,
        alignment: 0x10,
    });

    elf_builder.write_u64(0x20, u64::MAX);
    let elf = ReadElf::from_slice(elf_builder.buffer()).unwrap();

    // This gets the value of `e_phnum`, but hasn't checked the table for actual
    // contents.
    assert_eq!(elf.program_headers().len(), 1);

    // Only when we go to lazy execute, we'll find that the headers aren't there.
    let segments: Vec<ProgramHeader> = elf.program_headers().collect();
    assert!(segments.is_empty())
}

#[test]
fn segments_elf32_phentsize_too_small() {
    let mut elf_builder = ElfBuilder64::new(Endian::Big);
    elf_builder
        .set_os_abi(OsAbi::from(OsAbi::NONE))
        .set_abi_version(0)
        .set_executable_type(ExecutableType::Executable)
        .set_machine(Machine::from(Machine::X86_64))
        .set_entry(0x1000)
        .set_flags(0x00000000);
    elf_builder.add_segment(&ProgramHeader {
        segment_type: SegmentType::Null,
        flags: SegmentFlags::from(SegmentFlags::R),
        file_offset: 0x1000,
        virtual_address: 0x2000,
        physical_address: 0x3000,
        file_size: 0x4000,
        memory_size: 0x5000,
        alignment: 0x10,
    });

    // Set `e_phentsize` to one less than the size of the structure.
    elf_builder.write_u16(0x36, 31);
    let elf = ReadElf::from_slice(elf_builder.buffer()).unwrap();

    // This gets the value of `e_phnum`, but hasn't checked the table for actual
    // contents.
    assert_eq!(elf.program_headers().len(), 1);

    // Only when we go to lazy execute, we'll find that the headers aren't there.
    let segments: Vec<ProgramHeader> = elf.program_headers().collect();
    assert!(segments.is_empty())
}

#[test]
fn segments_elf64_phentsize_too_small() {
    let mut elf_builder = ElfBuilder64::new(Endian::Big);
    elf_builder
        .set_os_abi(OsAbi::from(OsAbi::NONE))
        .set_abi_version(0)
        .set_executable_type(ExecutableType::Executable)
        .set_machine(Machine::from(Machine::X86_64))
        .set_entry(0x1000)
        .set_flags(0x00000000);
    elf_builder.add_segment(&ProgramHeader {
        segment_type: SegmentType::Null,
        flags: SegmentFlags::from(SegmentFlags::R),
        file_offset: 0x1000,
        virtual_address: 0x2000,
        physical_address: 0x3000,
        file_size: 0x4000,
        memory_size: 0x5000,
        alignment: 0x10,
    });

    // Set `e_phentsize` to one less than the size of the structure.
    elf_builder.write_u16(0x36, 55);
    let elf = ReadElf::from_slice(elf_builder.buffer()).unwrap();

    // This gets the value of `e_phnum`, but hasn't checked the table for actual
    // contents.
    assert_eq!(elf.program_headers().len(), 1);

    // Only when we go to lazy execute, we'll find that the headers aren't there.
    let segments: Vec<ProgramHeader> = elf.program_headers().collect();
    assert!(segments.is_empty())
}

#[test]
fn segments_aligned() {
    let mut elf_builder = ElfBuilder64::new(Endian::Big);
    elf_builder
        .set_os_abi(OsAbi::from(OsAbi::NONE))
        .set_abi_version(0)
        .set_executable_type(ExecutableType::Executable)
        .set_machine(Machine::from(Machine::X86_64))
        .set_entry(0x1000)
        .set_flags(0x00000000);
    elf_builder.add_segment(&ProgramHeader {
        segment_type: SegmentType::Null,
        flags: SegmentFlags::from(SegmentFlags::NONE),
        file_offset: 0,
        virtual_address: 0,
        physical_address: 0,
        file_size: 0,
        memory_size: 0,
        alignment: 0,
    });
    elf_builder.add_segment(&ProgramHeader {
        segment_type: SegmentType::Note,
        flags: SegmentFlags::from(SegmentFlags::R),
        file_offset: 0x12AE,
        virtual_address: 0x22AE,
        physical_address: 0x0000,
        file_size: 0x1000,
        memory_size: 0x1000,
        alignment: 0x1000,
    });

    let elf = ReadElf::from_slice(elf_builder.buffer()).unwrap();

    assert_eq!(elf.program_headers().len(), 2);
    let segments: Vec<ProgramHeader> = elf.program_headers().collect();
    assert!(segments[1].is_aligned());
}

#[test]
fn segments_not_aligned() {
    let mut elf_builder = ElfBuilder64::new(Endian::Big);
    elf_builder
        .set_os_abi(OsAbi::from(OsAbi::NONE))
        .set_abi_version(0)
        .set_executable_type(ExecutableType::Executable)
        .set_machine(Machine::from(Machine::X86_64))
        .set_entry(0x1000)
        .set_flags(0x00000000);
    elf_builder.add_segment(&ProgramHeader {
        segment_type: SegmentType::Null,
        flags: SegmentFlags::from(SegmentFlags::NONE),
        file_offset: 0,
        virtual_address: 0,
        physical_address: 0,
        file_size: 0,
        memory_size: 0,
        alignment: 0,
    });
    elf_builder.add_segment(&ProgramHeader {
        segment_type: SegmentType::Note,
        flags: SegmentFlags::from(SegmentFlags::R),
        file_offset: 0x12AE,
        virtual_address: 0x23AE,
        physical_address: 0x0000,
        file_size: 0x1000,
        memory_size: 0x1000,
        alignment: 0x1000,
    });

    let elf = ReadElf::from_slice(elf_builder.buffer()).unwrap();

    assert_eq!(elf.program_headers().len(), 2);
    let segments: Vec<ProgramHeader> = elf.program_headers().collect();
    assert!(!segments[1].is_aligned());
}
