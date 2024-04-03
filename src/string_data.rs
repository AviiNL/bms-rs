// FalconSharedMemoryAreaString
mod string_id;
use std::{collections::HashMap, mem::size_of};

pub use string_id::*;

use crate::{MemoryFile, RawMemoryFile};

#[repr(C)]
#[derive(Debug, Default)]
pub struct StringAreaHeader {
    version_num: u32,
    no_of_strings: u32,
    data_size: u32,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct StringHeader {
    str_id: StringId,
    length: u32,
}

pub struct StringData;

impl StringData {
    pub fn read() -> Result<HashMap<StringId, String>, Box<dyn std::error::Error + Send + Sync>> {
        let header =
            unsafe { MemoryFile::<StringAreaHeader>::new("FalconSharedMemoryAreaString")? };

        let header = header.read();

        let mut offset = size_of::<StringAreaHeader>();
        let mut strings: HashMap<StringId, String> = HashMap::new();

        for _ in 0..header.no_of_strings {
            if offset >= header.data_size as usize {
                break;
            }

            let header_size = size_of::<StringHeader>();
            let string_header = unsafe {
                MemoryFile::<StringHeader>::new_with_offset_and_size(
                    "FalconSharedMemoryAreaString",
                    offset,
                    header_size,
                )?
            };

            offset += header_size;
            if offset >= header.data_size as usize {
                break;
            }

            let string_data = string_header.read();

            let string = unsafe {
                RawMemoryFile::new_with_offset_and_size(
                    "FalconSharedMemoryAreaString",
                    offset,
                    string_data.length as usize,
                )?
            };
            let string = String::from_utf8_lossy(string.read());

            strings.insert(string_data.str_id, string.to_string());

            offset += string_data.length as usize + 1;
        }

        Ok(strings)
    }
}
