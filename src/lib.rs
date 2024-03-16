mod flight_data;
mod intellivibe_data;

pub use flight_data::FlightData;
pub use intellivibe_data::IntellivibeData;

use std::mem::size_of;

use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{CloseHandle, HANDLE},
        System::Memory::{
            MapViewOfFile, OpenFileMappingW, UnmapViewOfFile, FILE_MAP_READ,
            MEMORY_MAPPED_VIEW_ADDRESS,
        },
    },
};

pub struct MemoryFile<'a, T> {
    data: &'a T,
    map: HANDLE,
    p_buf: MEMORY_MAPPED_VIEW_ADDRESS,
}

// I need to know how much of a bad idea this is...
unsafe impl<'a, T> Send for MemoryFile<'a, T> {}
unsafe impl<'a, T> Sync for MemoryFile<'a, T> {}

impl<'a, T> MemoryFile<'a, T> {
    /// # Safety
    /// If the generic struct given does not match the memory layout,
    /// this is probably going to make shit hit the fan
    /// Do not use directly unless you know what you're doing.
    pub unsafe fn new(name: &'a str) -> Result<Self, Box<dyn std::error::Error>> {
        let hname: HSTRING = HSTRING::from(name);
        let map = OpenFileMappingW(FILE_MAP_READ.0, false, &hname)?;

        let p_buf = MapViewOfFile(map, FILE_MAP_READ, 0, 0, 0);

        let p = p_buf.Value;
        let ptr: *const u8 = p as *const u8;

        let slice = std::slice::from_raw_parts(ptr, size_of::<T>());
        let data = &slice.align_to::<T>().1[0];

        Ok(Self { data, map, p_buf })
    }

    pub fn read(&self) -> &T {
        self.data
    }
}

impl<'a, T> Drop for MemoryFile<'a, T> {
    fn drop(&mut self) {
        let _ = unsafe { CloseHandle(self.map) };
        let _ = unsafe { UnmapViewOfFile(self.p_buf) };
    }
}
