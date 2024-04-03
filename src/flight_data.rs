mod bits;
use std::fmt::Debug;

pub use bits::*;

use crate::MemoryFile;

#[repr(C)]
#[derive(Default)]
pub struct Line([[u8; 26]; 5]);

impl Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();

        for line in &self.0 {
            ret += &String::from_utf8_lossy(line);
            ret += "\n";
        }

        write!(f, "{}", ret.trim_end_matches('\n'))
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct FlightData {
    pub x: f32,           // Ownship North (Ft)
    pub y: f32,           // Ownship East (Ft)
    pub z: f32, // Ownship Down (Ft) --- NOTE: use FlightData2 AAUZ for barometric altitude!
    pub x_dot: f32, // Ownship North Rate (ft/sec)
    pub y_dot: f32, // Ownship East Rate (ft/sec)
    pub z_dot: f32, // Ownship Down Rate (ft/sec)
    pub alpha: f32, // Ownship AOA (Degrees)
    pub beta: f32, // Ownship Beta (Degrees)
    pub gamma: f32, // Ownship Gamma (Radians)
    pub pitch: f32, // Ownship Pitch (Radians)
    pub roll: f32, // Ownship Pitch (Radians)
    pub yaw: f32, // Ownship Pitch (Radians)
    pub mach: f32, // Ownship Mach number
    pub kias: f32, // Ownship Indicated Airspeed (Knots)
    pub vt: f32, // Ownship True Airspeed (Ft/Sec)
    pub gs: f32, // Ownship Normal Gs
    pub wind_offset: f32, // Wind delta to FPM (Radians)
    pub nozzle_pos: f32, // Ownship engine nozzle percent open (0-100)
    //pub nozzlePos2: f32,   // MOVED TO FlightData2! Ownship engine nozzle2 percent open (0-100)
    pub internal_fuel: f32, // Ownship internal fuel (Lbs)
    pub external_fuel: f32, // Ownship external fuel (Lbs)
    pub fuel_flow: f32,     // Ownship fuel flow (Lbs/Hour)
    pub rpm: f32,           // Ownship engine rpm (Percent 0-103)
    //pub rpm2: f32,         // MOVED TO FlightData2! Ownship engine rpm2 (Percent 0-103)
    pub ftit: f32, // Ownship Forward Turbine Inlet Temp (Degrees C)
    //pub ftit2: f32,        // MOVED TO FlightData2! Ownship Forward Turbine Inlet Temp2 (Degrees C)
    pub gear_pos: f32,     // Ownship Gear position 0 = up, 1 = down: f32,
    pub speed_brake: f32,  // Ownship speed brake position 0 = closed, 1 = 60 Degrees open
    pub epu_fuel: f32,     // Ownship EPU fuel (Percent 0-100)
    pub oil_pressure: f32, // Ownship Oil Pressure (Percent 0-100)
    //pub oilPressure2: f32, // MOVED TO FlightData2! Ownship Oil Pressure2 (Percent 0-100)
    pub light_bits: LightBits, // Cockpit Indicator Lights, one bit per bulb. See enum

    // These are inputs. Use them carefully
    // NB: these do not work when TrackIR device is enabled
    // NB2: launch falcon with the '-head' command line parameter to activate !
    pub head_pitch: f32, // Head pitch offset from design eye (radians)
    pub head_roll: f32,  // Head roll offset from design eye (radians)
    pub head_yaw: f32,   // Head yaw offset from design eye (radians)

    // new lights
    pub light_bits2: LightBits2, // Cockpit Indicator Lights, one bit per bulb. See enum
    pub light_bits3: LightBits3, // Cockpit Indicator Lights, one bit per bulb. See enum

    // chaff/flare
    pub chaff_count: f32, // Number of Chaff left
    pub flare_count: f32, // Number of Flare left

    // landing gear
    pub nose_gear_pos: f32, // Position of the nose landinggear: f32, caution: full down values defined in dat files
    pub left_gear_pos: f32, // Position of the left landinggear: f32, caution: full down values defined in dat files
    pub right_gear_pos: f32, // Position of the right landinggear: f32, caution: full down values defined in dat files

    // ADI values
    pub adi_ils_hor_pos: f32, // Position of horizontal ILS bar
    pub adi_ils_ver_pos: f32, // Position of vertical ILS bar

    // HSI states
    pub course_state: i32,  // HSI_STA_CRS_STATE
    pub heading_state: i32, // HSI_STA_HDG_STATE
    pub total_states: i32,  // HSI_STA_TOTAL_STATES: i32, never set

    // HSI values
    pub course_deviation: f32,     // HSI_VAL_CRS_DEVIATION
    pub desired_course: f32,       // HSI_VAL_DESIRED_CRS
    pub distance_to_beacon: f32,   // HSI_VAL_DISTANCE_TO_BEACON
    pub bearing_to_beacon: f32,    // HSI_VAL_BEARING_TO_BEACON
    pub current_heading: f32,      // HSI_VAL_CURRENT_HEADING
    pub desired_heading: f32,      // HSI_VAL_DESIRED_HEADING
    pub deviation_limit: f32,      // HSI_VAL_DEV_LIMIT
    pub half_deviation_limit: f32, // HSI_VAL_HALF_DEV_LIMIT
    pub localizer_course: f32,     // HSI_VAL_LOCALIZER_CRS
    pub airbase_x: f32,            // HSI_VAL_AIRBASE_X
    pub airbase_y: f32,            // HSI_VAL_AIRBASE_Y
    pub total_values: f32,         // HSI_VAL_TOTAL_VALUES: f32, never set

    pub trim_pitch: f32, // Value of trim in pitch axis, -0.5 to +0.5
    pub trim_roll: f32,  // Value of trim in roll axis, -0.5 to +0.5
    pub trim_yaw: f32,   // Value of trim in yaw axis, -0.5 to +0.5

    // HSI flags
    pub hsi_bits: HsiBits, // HSI flags

    //DED Lines
    pub dedlines: Line, // [5][26]: char,  //25 usable chars
    pub invert: Line,   // [5][26]: char,    //25 usable chars

    //PFL Lines
    pub pfllines: Line,  //25 usable chars
    pub pflinvert: Line, //25 usable chars

    //TacanChannel
    pub ufc_tchan: i32,
    pub aux_tchan: i32,

    // RWR
    pub rwr_object_count: i32,
    pub rwr_symbol: [i32; 40],
    pub bearing: [f32; 40],
    pub missile_activity: [u32; 40],
    pub missile_launch: [u32; 40],
    pub selected: [u32; 40],
    pub lethality: [f32; 40],
    pub new_detection: [u32; 40],

    //fuel values
    pub fwd: f32,
    pub aft: f32,
    pub total: f32,

    pub version_num: i32, // Version of FlightData mem area

    // New values added here for header file compatibility but not implemented
    // in this version of the code at present.
    pub head_x: f32, // Head X offset from design eye (feet)
    pub head_y: f32, // Head Y offset from design eye (feet)
    pub head_z: f32, // Head Z offset from design eye (feet)

    pub main_power: i32, // Main Power switch state, 0=down, 1=middle, 2=up
}

impl Default for FlightData {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
            x_dot: Default::default(),
            y_dot: Default::default(),
            z_dot: Default::default(),
            alpha: Default::default(),
            beta: Default::default(),
            gamma: Default::default(),
            pitch: Default::default(),
            roll: Default::default(),
            yaw: Default::default(),
            mach: Default::default(),
            kias: Default::default(),
            vt: Default::default(),
            gs: Default::default(),
            wind_offset: Default::default(),
            nozzle_pos: Default::default(),
            internal_fuel: Default::default(),
            external_fuel: Default::default(),
            fuel_flow: Default::default(),
            rpm: Default::default(),
            ftit: Default::default(),
            gear_pos: Default::default(),
            speed_brake: Default::default(),
            epu_fuel: Default::default(),
            oil_pressure: Default::default(),
            light_bits: Default::default(),
            head_pitch: Default::default(),
            head_roll: Default::default(),
            head_yaw: Default::default(),
            light_bits2: Default::default(),
            light_bits3: Default::default(),
            chaff_count: Default::default(),
            flare_count: Default::default(),
            nose_gear_pos: Default::default(),
            left_gear_pos: Default::default(),
            right_gear_pos: Default::default(),
            adi_ils_hor_pos: Default::default(),
            adi_ils_ver_pos: Default::default(),
            course_state: Default::default(),
            heading_state: Default::default(),
            total_states: Default::default(),
            course_deviation: Default::default(),
            desired_course: Default::default(),
            distance_to_beacon: Default::default(),
            bearing_to_beacon: Default::default(),
            current_heading: Default::default(),
            desired_heading: Default::default(),
            deviation_limit: Default::default(),
            half_deviation_limit: Default::default(),
            localizer_course: Default::default(),
            airbase_x: Default::default(),
            airbase_y: Default::default(),
            total_values: Default::default(),
            trim_pitch: Default::default(),
            trim_roll: Default::default(),
            trim_yaw: Default::default(),
            hsi_bits: Default::default(),
            dedlines: Default::default(),
            invert: Default::default(),
            pfllines: Default::default(),
            pflinvert: Default::default(),
            ufc_tchan: Default::default(),
            aux_tchan: Default::default(),
            rwr_object_count: Default::default(),
            rwr_symbol: [0; 40],
            bearing: [0.0; 40],
            missile_activity: [0; 40],
            missile_launch: [0; 40],
            selected: [0; 40],
            lethality: [0.0; 40],
            new_detection: [0; 40],
            fwd: Default::default(),
            aft: Default::default(),
            total: Default::default(),
            version_num: Default::default(),
            head_x: Default::default(),
            head_y: Default::default(),
            head_z: Default::default(),
            main_power: Default::default(),
        }
    }
}

unsafe impl Send for FlightData {}
unsafe impl Sync for FlightData {}

impl FlightData {
    pub fn new<'a>() -> Result<MemoryFile<'a, Self>, Box<dyn std::error::Error + Send + Sync>> {
        let file = unsafe { MemoryFile::<'a, Self>::new("FalconSharedMemoryArea")? };
        Ok(file)
    }
}
