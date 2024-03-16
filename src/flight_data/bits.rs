bitflags::bitflags! {
    #[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
    pub struct LightBits: u32 {
        const MasterCaution = 0x1;
        // Brow Lights
        const TF = 0x2;          // Left eyebrow
        const OXY_BROW = 0x4;    // repurposed for eyebrow OXY LOW (was OBS, unused)
        const EQUIP_HOT = 0x8;   // Caution light; repurposed for cooling fault (was: not used)
        const ONGROUND = 0x10;   // True if on ground: this is not a lamp bit!
        const ENG_FIRE = 0x20;   // Right eyebrow; upper half of split face lamp
        const CONFIG = 0x40;     // Stores config, caution panel
        const HYD = 0x80;        // Right eyebrow; see also OIL (this lamp is not split face)
        const Flcs_ABCD = 0x100; // TEST panel FLCS channel lamps; repurposed, was OIL (see HYD; that lamp is not split face)
        const FLCS = 0x200;      // Right eyebrow; was called DUAL which matches block 25, 30/32 and older 40/42
        const CAN = 0x400;       // Right eyebrow
        const T_L_CFG = 0x800;   // Right eyebrow

        // AOA Indexers
        const AOAAbove = 0x1000;
        const AOAOn = 0x2000;
        const AOABelow = 0x4000;

        // Refuel/NWS
        const RefuelRDY = 0x8000;
        const RefuelAR = 0x10000;
        const RefuelDSC = 0x20000;

        // Caution Lights
        const FltControlSys = 0x40000;
        const LEFlaps = 0x80000;
        const EngineFault = 0x100000;
        const Overheat = 0x200000;
        const FuelLow = 0x400000;
        const Avionics = 0x800000;
        const RadarAlt = 0x1000000;
        const IFF = 0x2000000;
        const ECM = 0x4000000;
        const Hook = 0x8000000;
        const NWSFail = 0x10000000;
        const CabinPress = 0x20000000;

        const AutoPilotOn = 0x40000000; // TRUE if is AP on.  NB: This is not a lamp bit!
        const TFR_STBY = 0x80000000;    // MISC panel; lower half of split face TFR lamp

        // Used with the MAL/IND light code to light up "everything"
        // please update this if you add/change bits!
        const AllLampBitsOn = 0xBFFFFFEF;
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
    pub struct LightBits2: u32 {
        // Threat Warning Prime
        const HandOff = 0x1;
        const Launch  = 0x2;
        const PriMode = 0x4;
        const Naval   = 0x8;
        const Unk     = 0x10;
        const TgtSep  = 0x20;

        // EWS
        const Go		= 0x40;		// On and operating normally
        const NoGo    = 0x80;     // On but malfunction present
        const Degr    = 0x100;    // Status message: AUTO DEGR
        const Rdy     = 0x200;    // Status message: DISPENSE RDY
        const ChaffLo = 0x400;    // Bingo chaff quantity reached
        const FlareLo = 0x800;    // Bingo flare quantity reached

        // Aux Threat Warning
        const AuxSrch = 0x1000;
        const AuxAct  = 0x2000;
        const AuxLow  = 0x4000;
        const AuxPwr  = 0x8000;

        // ECM
        const EcmPwr  = 0x10000;
        const EcmFail = 0x20000;

        // Caution Lights
        const FwdFuelLow = 0x40000;
        const AftFuelLow = 0x80000;

        const EPUOn      = 0x100000;  // EPU panel; run light
        const JFSOn      = 0x200000;  // Eng Jet Start panel; run light

        // Caution panel
        const SEC          = 0x400000;
        const OXY_LOW      = 0x800000;
        const PROBEHEAT    = 0x1000000;
        const SEAT_ARM     = 0x2000000;
        const BUC          = 0x4000000;
        const FUEL_OIL_HOT = 0x8000000;
        const ANTI_SKID    = 0x10000000;

        const TFR_ENGAGED  = 0x20000000;  // MISC panel; upper half of split face TFR lamp
        const GEARHANDLE   = 0x40000000;  // Lamp in gear handle lights on fault or gear in motion
        const ENGINE       = 0x80000000;  // Lower half of right eyebrow ENG FIRE/ENGINE lamp

        // Used with the MAL/IND light code to light up "everything"
        // please update this if you add/change bits!
        const AllLampBits2On = 0xFFFFF03F;
        // const AllLampBits2OnExceptCarapace = Self::AllLampBits2On ^ Self::HandOff ^ Self::Launch ^ Self::PriMode ^ Self::Naval ^ Self::Unk ^ Self::TgtSep ^ Self::AuxSrch ^ Self::AuxAct ^ Self::AuxLow ^ Self::AuxPwr;
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
    pub struct LightBits3: u32 {
        // Elec panel
        const FlcsPmg = 0x1;
        const MainGen = 0x2;
        const StbyGen = 0x4;
        const EpuGen  = 0x8;
        const EpuPmg  = 0x10;
        const ToFlcs  = 0x20;
        const FlcsRly = 0x40;
        const BatFail = 0x80;

        // EPU panel
        const Hydrazine = 0x100;
        const Air       = 0x200;

        // Caution panel
        const Elec_Fault = 0x400;
        const Lef_Fault  = 0x800;

        const OnGround	    = 0x1000;   // weight-on-wheels
        const FlcsBitRun    = 0x2000;   // FLT CONTROL panel RUN light (used to be Multi-engine fire light)
        const FlcsBitFail   = 0x4000;   // FLT CONTROL panel FAIL light (used to be Lock light Cue; non-F-16)
        const DbuWarn       = 0x8000;   // Right eyebrow DBU ON cell; was Shoot light cue; non-F16
        const NoseGearDown  = 0x10000;  // Landing gear panel; on means down and locked
        const LeftGearDown  = 0x20000;  // Landing gear panel; on means down and locked
        const RightGearDown = 0x40000;  // Landing gear panel; on means down and locked
        const ParkBrakeOn   = 0x100000; // Parking brake engaged; NOTE: not a lamp bit
        const Power_Off     = 0x200000; // Set if there is no electrical power.  NB: not a lamp bit

        // Caution panel
        const cadc	= 0x400000;

        // Left Aux console
        const SpeedBrake = 0x800000;  // True if speed brake is in anything other than stowed position

        // Threat Warning Prime - additional bits
        const SysTest  = 0x1000000;

        // Master Caution WILL come up (actual lightBit has 3sec delay like in RL),
        // usable for cockpit builders with RL equipment which has a delay on its own.
        // Will be set to false again as soon as the MasterCaution bit is set.
        const MCAnnounced = 0x2000000;

        //MLGWOW is only for AFM , it means WOW switches on MLG are triggered => FLCS switches to WOWPitchRockGain
        const MLGWOW = 0x4000000;
        const NLGWOW = 0x8000000;

        const ATF_Not_Engaged = 0x10000000;

        // Caution panel
        const Inlet_Icing = 0x20000000;

        // Free bits in LightBits3
        //0x40000000,
        //0x80000000,

        // Used with the MAL/IND light code to light up "everything"
        // please update this if you add/change bits!
        const AllLampBits3On = 0x3147EFFF;
        // const AllLampBits3OnExceptCarapace = AllLampBits3On ^ SysTest;
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
    pub struct HsiBits: u32 {
        const ToTrue        = 0x01;    // HSI_FLAG_TO_TRUE == 1, TO
        const IlsWarning    = 0x02;    // HSI_FLAG_ILS_WARN
        const CourseWarning = 0x04;    // HSI_FLAG_CRS_WARN
        const Init          = 0x08;    // HSI_FLAG_INIT
        const TotalFlags    = 0x10;    // HSI_FLAG_TOTAL_FLAGS; never set
        const ADI_OFF       = 0x20;    // ADI OFF Flag
        const ADI_AUX       = 0x40;    // ADI AUX Flag
        const ADI_GS        = 0x80;    // ADI GS FLAG
        const ADI_LOC       = 0x100;   // ADI LOC FLAG
        const HSI_OFF       = 0x200;   // HSI OFF Flag
        const BUP_ADI_OFF   = 0x400;   // Backup ADI Off Flag
        const VVI           = 0x800;   // VVI OFF Flag
        const AOA           = 0x1000;  // AOA OFF Flag
        const AVTR          = 0x2000;  // AVTR Light
        const OuterMarker   = 0x4000;  // MARKER beacon light for outer marker
        const MiddleMarker  = 0x8000;  // MARKER beacon light for middle marker
        const FromTrue      = 0x10000; // HSI_FLAG_TO_TRUE == 2, FROM

        const Flying		  = 0x80000000; // true if player is attached to an aircraft (i.e. not in UI state).  NOTE: Not a lamp bit

        // Used with the MAL/IND light code to light up "everything"
        // please update this is you add/change bits!
        const AllLampHsiBitsOn = 0xE000;
    }
}
