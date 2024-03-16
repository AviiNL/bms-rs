// use crate::read_memory;

use crate::MemoryFile;

#[repr(C)]
#[derive(Debug, Default)]
pub struct IntellivibeData {
    pub aa_missile_fired: u8,
    pub ag_missile_fired: u8,
    pub bomb_dropped: u8,
    pub flare_dropped: u8,
    pub chaff_dropped: u8,
    pub bullets_fired: u8,
    pub collision_counter: i32,
    pub firing_gun: bool,
    pub end_flight: bool,
    pub ejecting: bool,
    pub in_3d: bool,
    pub paused: bool,
    pub frozen: bool,
    pub over_g: bool,
    pub on_ground: bool,
    pub exit_game: bool,
    pub g_force: f32,
    pub eyes_x: f32,
    pub eyes_y: f32,
    pub eyes_z: f32,
    pub last_damage: i32,
    pub damage_force: f32,
    pub when_damage: i32,
}

unsafe impl Send for IntellivibeData {}
unsafe impl Sync for IntellivibeData {}

impl IntellivibeData {
    pub fn new<'a>() -> Result<MemoryFile<'a, Self>, Box<dyn std::error::Error>> {
        let file = unsafe { MemoryFile::<'a, Self>::new("FalconIntellivibeSharedMemoryArea")? };
        Ok(file)
    }
}
