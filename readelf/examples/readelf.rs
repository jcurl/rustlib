use clap::{CommandFactory, Parser};
use std::process::ExitCode;

use readelf::*;

#[derive(Parser, Debug)]
#[command(disable_help_flag = true)]
struct Args {
    /// Show help.
    #[arg(short = '?', long = "help", default_value_t = false)]
    help: bool,

    /// Show the ELF header (usually first 52/64 bytes).
    #[arg(short = 'h', long = "file-header", default_value_t = false)]
    headers: bool,

    /// Show the program headers (segments).
    #[arg(short = 'l', long = "segments", default_value_t = false)]
    segments: bool,

    /// Show the section headers.
    #[arg(short = 'S', long = "sections", default_value_t = false)]
    sections: bool,

    /// A list of files that should be read.
    #[arg(trailing_var_arg = true)]
    files: Vec<String>,
}

fn main() -> ExitCode {
    let cli = Args::parse();
    if cli.help {
        let _ = Args::command().print_long_help();
        return ExitCode::SUCCESS;
    }

    for file in cli.files {
        let path = std::path::PathBuf::from(&file);
        let file_data = std::fs::read(path).unwrap();
        let slice = file_data.as_slice();

        let r = ReadElf::from_slice(slice).unwrap();
        println!("ELF File: {}", &file);

        if cli.headers {
            println!(" Header:");
            println!("  Class: {} ({})", r.class, u8::from(r.class));
            println!("  Data: {} ({})", r.data, u8::from(r.data));
            println!("  Version: {}", r.version);
            println!("  OS ABI: {} ({})", r.osabi, u8::from(r.osabi));
            println!("  ABI Version: {}", r.abi_version);
            println!("  Type: {} ({})", r.exec_type, u16::from(r.exec_type));
            println!("  Machine: {} ({})", r.machine, u16::from(r.machine));
            println!("  Entry: 0x{:0>8X}", r.entry);
            println!("  Flags: 0x{:0>8X}", r.flags);
        }

        if cli.segments {
            println!(
                "  Segments: {:<14} Flags    File Offset      Virt Address     Phys Address     File Size        Memory Size      Alignment",
                r.program_headers().len()
            );
            let mut i = 0_u16;
            for segment in r.program_headers() {
                let aligned = if segment.is_aligned() { 'A' } else { 'X' };
                println!(
                    "   {:>2} {:<20} {:<8} {:0>16X} {:0>16X} {:0>16X} {:0>16X} {:0>16X} {:0>8X} {}",
                    i,
                    segment.segment_type.to_string(),
                    segment_flags(&segment.flags),
                    segment.file_offset,
                    segment.virtual_address,
                    segment.physical_address,
                    segment.file_size,
                    segment.memory_size,
                    segment.alignment,
                    aligned
                );
                i += 1;
            }
        }

        if cli.sections {
            println!(
                "  Sections: {:<14} Type                 Flags            Virt Address     File Offset      File Size        SecLink  SecInfo  Alignment  EntSize",
                r.section_headers().len()
            );
            let mut i = 0_u16;
            for section in r.section_headers() {
                let aligned = if section.is_aligned() { 'A' } else { 'X' };
                println!(
                    "   {:>2} {:<20} {:<20} {:<16} {:0>16X} {:0>16X} {:0>16X} {:0>8X} {:0>8X} {:0>8X} {} {:0>8X}",
                    i,
                    section.name.unwrap_or("<none>".to_string()),
                    section.section_type.to_string(),
                    section_flags(&section.flags),
                    section.virtual_address,
                    section.file_offset,
                    section.file_size,
                    section.section_link,
                    section.section_info,
                    section.alignment,
                    aligned,
                    section.entry_size
                );
                i += 1;
            }
        }
    }
    ExitCode::SUCCESS
}

fn segment_flags(flags: &SegmentFlags) -> String {
    let mut result = String::default();
    let mut f = flags.flags();

    if f & SegmentFlags::R != 0 {
        result.push('R');
        f ^= SegmentFlags::R;
    } else {
        result.push('-')
    }
    if f & SegmentFlags::W != 0 {
        result.push('W');
        f ^= SegmentFlags::W;
    } else {
        result.push('-')
    }
    if f & SegmentFlags::X != 0 {
        result.push('E');
        f ^= SegmentFlags::X;
    } else {
        result.push('-')
    }

    if f != 0 {
        result.push_str(format!("0x{:X}", f).as_str());
    }

    result
}

fn section_flags(flags: &SectionFlags) -> String {
    let mut result = String::default();
    let mut f = flags.flags();

    if f & SectionFlags::WRITE != 0 {
        result.push('W');
        f ^= SectionFlags::WRITE;
    } else {
        result.push('-')
    }
    if f & SectionFlags::ALLOC != 0 {
        result.push('A');
        f ^= SectionFlags::ALLOC;
    } else {
        result.push('-')
    }
    if f & SectionFlags::EXECINSTR != 0 {
        result.push('X');
        f ^= SectionFlags::EXECINSTR;
    } else {
        result.push('-')
    }
    if f & SectionFlags::MERGE != 0 {
        result.push('M');
        f ^= SectionFlags::MERGE;
    } else {
        result.push('-')
    }
    if f & SectionFlags::STRINGS != 0 {
        result.push('S');
        f ^= SectionFlags::STRINGS;
    } else {
        result.push('-')
    }
    if f & SectionFlags::INFO_LINK != 0 {
        result.push('I');
        f ^= SectionFlags::INFO_LINK;
    } else {
        result.push('-')
    }
    if f & SectionFlags::LINK_ORDER != 0 {
        result.push('L');
        f ^= SectionFlags::LINK_ORDER;
    } else {
        result.push('-')
    }
    if f & SectionFlags::OS_NONCONFORMING != 0 {
        result.push('O');
        f ^= SectionFlags::OS_NONCONFORMING;
    } else {
        result.push('-')
    }
    if f & SectionFlags::GROUP != 0 {
        result.push('G');
        f ^= SectionFlags::GROUP;
    } else {
        result.push('-')
    }
    if f & SectionFlags::TLS != 0 {
        result.push('T');
        f ^= SectionFlags::TLS;
    } else {
        result.push('-')
    }
    if f & SectionFlags::MASKOS != 0 {
        result.push_str(&format!("{:0>2X}", (f & SectionFlags::MASKOS) >> 20));
        f &= !SectionFlags::MASKOS;
    } else {
        result.push_str("--");
    }
    if f & SectionFlags::MASKPROC != 0 {
        result.push_str(&format!("{:X}", (f & SectionFlags::MASKPROC) >> 28));
        f &= !SectionFlags::MASKOS;
    } else {
        result.push('-');
    }

    if f != 0 {
        result.push_str(format!("0x{:X}", f).as_str());
    }

    result
}
