use readelf::ReadElf;
use serde::de::Error;
use serde::Deserialize;
use std::path::PathBuf;

// The following structs allow us to implement serialization / deserialization
// with `serde` when reading a configuration file.

#[derive(Debug)]
pub(crate) struct Class(pub(crate) readelf::Class);

impl<'de> Deserialize<'de> for Class {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let variant = String::deserialize(deserializer)?;
        match variant.as_str() {
            "Elf32" => Ok(Class(readelf::Class::Elf32)),
            "Elf64" => Ok(Class(readelf::Class::Elf64)),
            //_ => Err("Unknown Class (must be 'Elf32' or 'Elf64')").map_err(D::Error::custom),
            _ => Err(D::Error::custom(
                "Unknown Class (must be 'Elf32' or 'Elf64')",
            )),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Endian(pub(crate) readelf::Endian);

impl<'de> Deserialize<'de> for Endian {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let variant = String::deserialize(deserializer)?;
        match variant.as_str() {
            "Big" => Ok(Endian(readelf::Endian::Big)),
            "Little" => Ok(Endian(readelf::Endian::Little)),
            _ => Err(D::Error::custom(
                "Unknown Endian (must be 'Big' or 'Little')",
            )),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ExecutableType(pub(crate) readelf::ExecutableType);

impl<'de> Deserialize<'de> for ExecutableType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let variant = String::deserialize(deserializer)?;
        match variant.as_str() {
            "None" => Ok(ExecutableType(readelf::ExecutableType::None)),
            "Relocatable" => Ok(ExecutableType(readelf::ExecutableType::Relocatable)),
            "Executable" => Ok(ExecutableType(readelf::ExecutableType::Executable)),
            "Shared" => Ok(ExecutableType(readelf::ExecutableType::Dynamic)),
            "Core" => Ok(ExecutableType(readelf::ExecutableType::Core)),
            _ => match variant.parse::<u16>() {
                Ok(v) => Ok(ExecutableType(readelf::ExecutableType::Unknown(v))),
                _ => Err(D::Error::custom("Unknown Executable Type)")),
            },
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ElfHeaders {
    pub(crate) elf_headers: Vec<ElfHeader>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ElfHeader {
    pub(crate) path: String,
    pub(crate) class: Class,
    pub(crate) data: Endian,
    pub(crate) version: u32,
    pub(crate) osabi: String,
    pub(crate) abi_version: u8,
    pub(crate) exec_type: ExecutableType,
    pub(crate) machine: String,
    pub(crate) flags: u32,
}

pub fn load_elf_file_vec<'elf>(elf_file: &str) -> ReadElf<'elf> {
    let test_path = test_resource_path(elf_file);
    let elf_file = std::fs::read(test_path).unwrap();

    ReadElf::from_vec(elf_file).unwrap()
}

pub fn load_elf_file<'elf>(elf_file: &str) -> ReadElf<'elf> {
    let test_path = test_resource_path(elf_file);
    ReadElf::open(test_path).unwrap()
}

pub fn test_resource_path(path: &str) -> PathBuf {
    let paths = path.split('/');
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources");
    d.push("tests");

    for path in paths {
        d.push(path);
    }
    d
}
