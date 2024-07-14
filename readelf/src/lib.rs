//! ReadElf is to read ELF files for inspecting the contents.
//!
//! Use the [ReadElf] struct to open a file on disk and get the contents of the
//! ELF file.

#![warn(absolute_paths_not_starting_with_crate)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(unit_bindings)]
#![warn(unreachable_pub)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_macro_rules)]
#![deny(keyword_idents)]
#![deny(non_ascii_idents)]
#![deny(trivial_numeric_casts)]

mod endian;
pub use endian::Endian;

mod osabi;
pub use osabi::OsAbi;

mod class;
pub use class::Class;

mod executable_type;
pub use executable_type::ExecutableType;

mod machine;
pub use machine::Machine;

mod segment_type;
pub use segment_type::SegmentType;

mod segment_flags;
pub use segment_flags::SegmentFlags;

mod section_type;
pub use section_type::SectionType;

mod section_flags;
pub use section_flags::SectionFlags;

mod binparser;

mod readelf;
pub use readelf::{ProgramHeader, ProgramHeaders, ReadElf, SectionHeader, SectionHeaders};
