use std::env;
use std::process::ExitCode;

use readelf::*;

fn main() -> ExitCode {
    let mut argnum = 0;
    for file in env::args() {
        argnum += 1;

        // Ignore the first parameter, as it's the tool itself.
        if argnum > 1 {
            let path = std::path::PathBuf::from(&file);
            let file_data = std::fs::read(path).unwrap();
            let slice = file_data.as_slice();

            let r = ReadElf::from_slice(slice).unwrap();
            println!("ELF File: {}", &file);
            println!(" Class: {} ({})", r.class, u8::from(r.class));
            println!(" Data: {} ({})", r.data, u8::from(r.data));
            println!(" Version: {}", r.version);
            println!(" OS ABI: {} ({})", r.osabi, u8::from(r.osabi));
            println!(" ABI Version: {}", r.abi_version);
            println!(" Type: {} ({})", r.exec_type, u16::from(r.exec_type));
            println!(" Machine: {} ({})", r.machine, u16::from(r.machine));
            println!(" Entry: 0x{:0>8X}", r.entry);
            println!(" Flags: 0x{:0>8X}", r.flags);
        }
    }
    ExitCode::SUCCESS
}
