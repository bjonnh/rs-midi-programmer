use std::str::FromStr;
use strum_macros::EnumString;
use std::convert::AsRef;
use std::string::ToString;
use strum_macros::AsRefStr;

#[derive(AsRefStr, Debug)]
enum MidiMode {
    #[strum(serialize = "U-1")]
    U1,
    #[strum(serialize = "U-2")]
    U2,
    #[strum(serialize = "U-3")]
    U3,
    #[strum(serialize = "U-4")]
    U4,
    #[strum(serialize = "S-1")]
    S1,
    #[strum(serialize = "S-2")]
    S2,
    #[strum(serialize = "S-3")]
    S3,
    #[strum(serialize = "S-4")]
    S4,
    #[strum(serialize = "Keep")]
    Keep // Special value used to not update
}

#[derive(Debug, PartialEq, AsRefStr)]
enum FootSwitch {
    #[strum(serialize = "norm")]
    Normal,
    #[strum(serialize = "inv")]
    Inverse,
    #[strum(serialize = "auto")]
    Auto,
    #[strum(serialize = "Keep")]
    Keep // Special value used to not update
}

#[derive(Debug, PartialEq, AsRefStr)]
enum TXInternal {
    #[strum(serialize = "2")]
    Two,
    #[strum(serialize = "5")]
    Five,
    #[strum(serialize = "10")]
    Ten,
    #[strum(serialize = "20")]
    Twenty,
    #[strum(serialize = "50")]
    Fifty,
    #[strum(serialize = "100")]
    Hundred,
    #[strum(serialize = "Keep")]
    Keep // Special value used to not update
}


pub struct BCR2000Global {
    midimode: MidiMode,
    startup_preset: u8, // 255 means keep current, 0 means last
    footswitch: FootSwitch,
    receivechannel: u8, // 255 means keep current, 0 means off
    deviceid: u8, // 255 means keep current
    txinterval: TXInternal,
    deadtime: u16 // 16384 means keep current
}

impl BCR2000Global {
    fn valid(&self) -> Vec<String> {
        let mut errors: Vec<String> = Vec::new();
        errors.clear();
        if !(0..=32).contains(&self.startup_preset) & (self.startup_preset!=255) {
            errors.push(format!("Startup preset should be between 1 and 32, 255 (current) or 0 (last) not {}", self.startup_preset));
        }
        if !(0..=16).contains(&self.receivechannel) & (self.receivechannel!=255) {
            errors.push(format!("Receive channel should be between 1 and 16, 255 (current) or 0 (off) not {}", self.receivechannel));
        }
        if !(1..=16).contains(&self.deviceid) {
            errors.push(format!("Device id should be between 1 and 16 or 255 (current) not {}", self.deviceid));
        }
        if !(0..=1000).contains(&self.deadtime) {
            errors.push(format!("Deadtime should be between 1 and 16 or 16384 (current) not {}", self.deadtime));
        }
        errors
    }

    pub(crate) fn to_bcl(&self) -> String {
        if !self.valid().is_empty() {
            panic!("{:?}", self.valid());
        }
        let mut bcl = String::with_capacity(64);
        bcl.push_str("$global\n");
        match self.midimode {
            MidiMode::Keep => (),
            _ => { bcl.push_str(&format!(".midimode {}\n", self.midimode.as_ref())) }
        }
        if self.startup_preset != 255 {
            let startup_preset: String;
            if self.startup_preset == 0 {
                startup_preset = String::from("last");
            } else {
                startup_preset = self.startup_preset.to_string();
            }
            bcl.push_str(&format!(".startup {}\n", startup_preset));
        }
        match self.footswitch {
            FootSwitch::Keep => (),
            _ => { bcl.push_str(&format!(".footsw {}\n", self.footswitch.as_ref())) }
        }
        if self.receivechannel != 255 {
            let receive_channel: String;
            if self.receivechannel == 0 {
                receive_channel = String::from("off");
            } else {
                receive_channel = self.receivechannel.to_string();
            }
            bcl.push_str(&format!(".rxch {}\n", receive_channel));
        }
        if self.deviceid != 255 {
            let deviceid = self.deviceid.to_string();
            bcl.push_str(&format!(".deviceid {}\n", deviceid));
        }
        match self.txinterval {
            TXInternal::Keep => (),
            _ => { bcl.push_str(&format!(".txinterval {}\n", self.txinterval.as_ref())) }
        }
        if self.deadtime != 16384 {
            let deadtime = self.deadtime.to_string();
            bcl.push_str(&format!(".deadtime {}\n", deadtime));
        }
        bcl
    }

    pub(crate) fn default() -> BCR2000Global {
        BCR2000Global {
            midimode: MidiMode::U3,
            startup_preset: 0,
            footswitch: FootSwitch::Auto,
            receivechannel: 0,
            deviceid: 1,
            txinterval: TXInternal::Two,
            deadtime: 0
        }
    }
}

pub struct BCR2000Preset {
    pub(crate) init: bool,
    pub(crate) name: String,
    pub(crate) snapshot: bool,
    pub(crate) request: bool,
    pub(crate) encoder_groups: u8,
    pub(crate) function_keys: bool,
    pub(crate) lock: bool,
    pub(crate) learn_output: Vec<Vec<u8>>
}

impl BCR2000Preset {
    pub(crate) fn default() -> BCR2000Preset {
        BCR2000Preset {
            name: "                        ".to_string(),
            init: true,
            snapshot: false,
            request: false,
            encoder_groups: 4,
            function_keys: true,
            lock: false,
            learn_output: Vec::new()
        }
    }
}

impl BCR2000Preset {
    fn valid(&self) -> Vec<String> {
        let mut errors: Vec<String> = Vec::new();
        errors.clear();
        if self.name.len()>24 {
            errors.push("Name is too long it should be less than 24 characters".to_string());
        }
        if self.name.contains('\'') {
            errors.push("Behavior with apostrophes in name is too inconsistent, it is not allowed".to_string());
        }

        if !(1..=4).contains(&self.encoder_groups) {
            errors.push("Encoder groups can only be between 1 and 4".to_string());
        }
        errors
    }

    fn to_bcl(&self) -> String {
        if !self.valid().is_empty() {
            panic!("{:?}", self.valid());
        }
        let mut bcl = String::with_capacity(64);
        bcl.push_str("$preset\n");
        if self.init {
            bcl.push_str(&".init\n");
        }
        if self.name != "" {
            bcl.push_str(&format!(".name '{}'\n", self.name));
        }
        bcl.push_str(&format!(".snapshot {}\n", if self.snapshot {"on"} else {"off"}));
        bcl.push_str(&format!(".request {}\n", if self.request {"on"} else {"off"}));
        bcl.push_str(&format!(".egroups {}\n", self.encoder_groups));
        bcl.push_str(&format!(".fkeys {}\n", if self.function_keys {"on"} else {"off"}));
        bcl.push_str(&format!(".lock {}\n", if self.lock {"on"} else {"off"}));
        for txblock in self.learn_output.iter() {
            if (txblock.len() > 0) {
                bcl.push_str(&".tx ");
                let hexs = txblock.iter().map(|v| format!("${:02x?}", v)).collect::<Vec<_>>();
                bcl.push_str(&hexs.join(" "));
                bcl.push_str(&"\n");
            }
        }
        bcl
    }
}

#[derive(AsRefStr, Debug)]
enum EasyParType {
    #[strum(serialize = "NOTE")]
    Note,
    #[strum(serialize = "AT")]
    NoteAfterTouch,
    #[strum(serialize = "CC")]
    CC,
    #[strum(serialize = "NRPN")]
    NRPN,
    #[strum(serialize = "GS/XG")]
    GSXG,
    #[strum(serialize = "PC")]
    ProgramChange,
    #[strum(serialize = "AT")]
    ChannelAfterTouch,
    #[strum(serialize = "PB")]
    PitchBend,
    #[strum(serialize = "MMC")]
    Exclusive,
}

pub struct BCR2000ControlElement {
    // off on, easypar is setting to off so you need to put after it
    showvalue: Option<bool>, // We should probably use that on all those bools
    // We will use higher values for off, off is implicit and easypar sets the default to 0 or whatever is appropriate
    default: Option<u32>,
}

pub struct BCR2000State {
    pub(crate) global: BCR2000Global,
    pub(crate) preset: BCR2000Preset,
}


static BCR_PREAMBLE: &'static str = "$rev R\n";
static BCR_END: &'static str = "$end\n";

impl BCR2000State {

    pub(crate) fn to_bcl(&self) -> String {
        if !self.global.valid().is_empty() {
            panic!("{:?}", self.global.valid());
        }
        let mut bcl = String::with_capacity(128);
        bcl.push_str(BCR_PREAMBLE);
        bcl.push_str(&self.global.to_bcl());
        bcl.push_str(&self.preset.to_bcl());
        bcl.push_str(BCR_END);
        bcl
    }

    pub(crate) fn store_to_bcl(&self, preset: u8) -> String {
        if !(1..=32).contains(&preset) {
            panic!("Can only store in presets 1 to 32");
        }
        let mut bcl = String::with_capacity(32);
        bcl.push_str(BCR_PREAMBLE);
        bcl.push_str(&format!("$store {}", preset));
        bcl.push_str(BCR_END);
        bcl
    }

    pub(crate) fn recall_from_bcl(&self, preset: u8) -> String {
        if !(1..=32).contains(&preset) {
            panic!("Can only recall from presets 1 to 32");
        }
        let mut bcl = String::with_capacity(32);
        bcl.push_str(BCR_PREAMBLE);
        bcl.push_str(&format!("$recall {}", preset));
        bcl.push_str(BCR_END);
        bcl
    }
}
