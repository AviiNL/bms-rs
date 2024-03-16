#[repr(u32)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StringId {
    // VERSION 1
    #[default]
    BmsExe = 0, // BMS exe name, full path
    KeyFile, // Key file name in use, full path

    BmsBasedir,      // BmsBasedir to BmsPictureDirectory:
    BmsBinDirectory, // - BMS directories in use
    BmsDataDirectory,
    BmsUIArtDirectory,
    BmsUserDirectory,
    BmsAcmiDirectory,
    BmsBriefingsDirectory,
    BmsConfigDirectory,
    BmsLogsDirectory,
    BmsPatchDirectory,
    BmsPictureDirectory,

    ThrName,        // Current theater name
    ThrCampaigndir, // ThrCampaigndir to ThrTacrefpicsdir:
    ThrTerraindir,  // - Current theater directories in use
    ThrArtdir,
    ThrMoviedir,
    ThrUisounddir,
    ThrObjectdir,
    Thr3ddatadir,
    ThrMisctexdir,
    ThrSounddir,
    ThrTacrefdir,
    ThrSplashdir,
    ThrCockpitdir,
    ThrSimdatadir,
    ThrSubtitlesdir,
    ThrTacrefpicsdir,

    AcName, // Current AC name
    AcNCTR, // Current AC NCTR

    // VERSION 2
    ButtonsFile, // Current 3dbuttons.dat file full path
    CockpitFile, // Current 3dckpit.dat file full path

    // VERSION 3
    NavPoint, // Multiple entries, one for each NavPoint. Format for each entry is (NP, O1, O2, PT can be concatenated):
    // (NavPoint, mandatory) NP:<index>,<type>,<x>,<y>,<z>,<grnd_elev>;
    //     <index>        int            NavPoint number, 1-99
    //     <type>         two chars      GM (GMPOINT), PO (POSPOINT), WP (WAYPOINT), MK (MARKPOINT), DL (DATALINK)
    //                                   CB (CAMPBULLSEYE), L1 (LINE1), L2 (LINE2), L3 (LINE3), L4 (LINE4), PT (PREPLANNEDTHREAD)
    //     <x>,<y>        float          position in sim coordinates
    //     <z>            float          altitude in 10s of feet
    //     <grnd_elev>    float          ground elevation in 10s of feet
    // (OA1/OA2, optional) O1:<bearing>,<range>,<alt>; (and/or) O2:<bearing>,<range>,<alt>;
    //     <bearing>      float
    //     <range>        unsigned int
    //     <alt>          unsigned int
    // (PPT, optional) PT:<str_id>,<range>,<declutter>;
    //     <str_id>       "string"
    //     <range>        float
    //     <declutter>    int            0 = false, 1 = true

    // VERSION 4
    ThrTerrdatadir,

    // FIXED LAST ENTRY
    StringIdentifierDIM, // (number of identifiers; add new IDs only *above* this one)
}

impl From<u8> for StringId {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::BmsExe,
            1 => Self::KeyFile,
            2 => Self::BmsBasedir,
            3 => Self::BmsBinDirectory,
            4 => Self::BmsDataDirectory,
            5 => Self::BmsUIArtDirectory,
            6 => Self::BmsUserDirectory,
            7 => Self::BmsAcmiDirectory,
            8 => Self::BmsBriefingsDirectory,
            9 => Self::BmsConfigDirectory,
            10 => Self::BmsLogsDirectory,
            11 => Self::BmsPatchDirectory,
            12 => Self::BmsPictureDirectory,
            13 => Self::ThrName,
            14 => Self::ThrCampaigndir,
            15 => Self::ThrTerraindir,
            16 => Self::ThrArtdir,
            17 => Self::ThrMoviedir,
            18 => Self::ThrUisounddir,
            19 => Self::ThrObjectdir,
            20 => Self::Thr3ddatadir,
            21 => Self::ThrMisctexdir,
            22 => Self::ThrSounddir,
            23 => Self::ThrTacrefdir,
            24 => Self::ThrSplashdir,
            25 => Self::ThrCockpitdir,
            26 => Self::ThrSimdatadir,
            27 => Self::ThrSubtitlesdir,
            28 => Self::ThrTacrefpicsdir,
            29 => Self::AcName,
            30 => Self::AcNCTR,
            31 => Self::ButtonsFile,
            32 => Self::CockpitFile,
            33 => Self::NavPoint,
            34 => Self::ThrTerrdatadir,
            _ => Self::StringIdentifierDIM, // Add appropriate handling for out-of-range values
        }
    }
}

impl From<StringId> for u8 {
    fn from(val: StringId) -> Self {
        val as u8
    }
}
