use std::convert::AsRef;
use std::str::FromStr;
use std::string::ToString;
use strum_macros::AsRefStr;
use strum_macros::EnumString;

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
    Keep, // Special value used to not update
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
    Keep, // Special value used to not update
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
    Keep, // Special value used to not update
}

pub struct BCR2000Global {
    midimode: MidiMode,
    startup_preset: u8, // 255 means keep current, 0 means last
    footswitch: FootSwitch,
    receivechannel: u8, // 255 means keep current, 0 means off
    deviceid: u8,       // 255 means keep current
    txinterval: TXInternal,
    deadtime: u16, // 16384 means keep current
}

impl BCR2000Global {
    fn valid(&self) -> Vec<String> {
        let mut errors: Vec<String> = Vec::new();
        errors.clear();
        if !(0..=32).contains(&self.startup_preset) & (self.startup_preset != 255) {
            errors.push(format!(
                "Startup preset should be between 1 and 32, 255 (current) or 0 (last) not {}",
                self.startup_preset
            ));
        }
        if !(0..=16).contains(&self.receivechannel) & (self.receivechannel != 255) {
            errors.push(format!(
                "Receive channel should be between 1 and 16, 255 (current) or 0 (off) not {}",
                self.receivechannel
            ));
        }
        if !(1..=16).contains(&self.deviceid) {
            errors.push(format!(
                "Device id should be between 1 and 16 or 255 (current) not {}",
                self.deviceid
            ));
        }
        if !(0..=1000).contains(&self.deadtime) {
            errors.push(format!(
                "Deadtime should be between 1 and 16 or 16384 (current) not {}",
                self.deadtime
            ));
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
            _ => bcl.push_str(&format!(".midimode {}\n", self.midimode.as_ref())),
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
            _ => bcl.push_str(&format!(".footsw {}\n", self.footswitch.as_ref())),
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
            _ => bcl.push_str(&format!(".txinterval {}\n", self.txinterval.as_ref())),
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
            deadtime: 0,
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
    pub(crate) learn_output: Vec<Vec<u8>>,
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
            learn_output: Vec::new(),
        }
    }
}

impl BCR2000Preset {
    fn valid(&self) -> Vec<String> {
        let mut errors: Vec<String> = Vec::new();
        if self.name.len() > 24 {
            errors.push("Name is too long it should be less than 24 characters".to_string());
        }
        if self.name.contains('\'') {
            errors.push(
                "Behavior with apostrophes in name is too inconsistent, it is not allowed"
                    .to_string(),
            );
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
        bcl.push_str(&format!(
            ".snapshot {}\n",
            if self.snapshot { "on" } else { "off" }
        ));
        bcl.push_str(&format!(
            ".request {}\n",
            if self.request { "on" } else { "off" }
        ));
        bcl.push_str(&format!(".egroups {}\n", self.encoder_groups));
        bcl.push_str(&format!(
            ".fkeys {}\n",
            if self.function_keys { "on" } else { "off" }
        ));
        bcl.push_str(&format!(".lock {}\n", if self.lock { "on" } else { "off" }));
        for txblock in self.learn_output.iter() {
            if txblock.len() > 0 {
                bcl.push_str(&".tx ");
                let hexs = txblock
                    .iter()
                    .map(|v| format!("${:02x?}", v))
                    .collect::<Vec<_>>();
                bcl.push_str(&hexs.join(" "));
                bcl.push_str(&"\n");
            }
        }
        bcl
    }
}

#[derive(AsRefStr, Debug)]
pub enum BCR2000EasyParType {
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

#[derive(AsRefStr, Debug)]
pub enum BCR2000EasyParMode {
    #[strum(serialize = "toggleoff")]
    ToggleOff,
    #[strum(serialize = "toggleon")]
    ToggleOn,
    #[strum(serialize = "increment")]
    Increment,
}

pub struct BCR2000EasyPar {
    pub(crate) partype: BCR2000EasyParType,
    pub(crate) channel: u8,
    pub(crate) controller: u8,
    pub(crate) value1: u16,
    pub(crate) value2: Option<u16>,
    pub(crate) mode: BCR2000EasyParMode,
    pub(crate) increment_value: i8
}

impl BCR2000EasyPar {
    fn valid(&self) -> Vec<String> {
        let mut errors: Vec<String> = Vec::new();
        if !(1..=16).contains(&self.channel) { errors.push(format!("Channel must be between 1 and 16, it was {}", self.channel)) }
        match self.partype {
            BCR2000EasyParType::CC => {
                if !(0..=127).contains(&self.controller) { errors.push(format!("Controller must be between 0 and 127, it was {}", self.controller)) }
            },
            _ => panic!("Sorry only CC is handled")
        }
        errors
    }

    fn to_bcl(&self) -> String {
        if !self.valid().is_empty() {
            panic!("{:?}", self.valid());
        }
        let mut bcl = String::with_capacity(64);
        match self.partype {
            BCR2000EasyParType::CC => {
                let value2 = match self.value2 {
                    Some(val) => format!("{}", val),
                    None => "off".to_string()
                };
                bcl.push_str(&format!(".easypar CC {} {} {} {} ", self.channel, self.controller, self.value1, value2));
                match self.mode {
                    BCR2000EasyParMode::ToggleOff => {
                        bcl.push_str("toggleoff\n");
                    }
                    BCR2000EasyParMode::ToggleOn => {
                        bcl.push_str("toggleon\n");
                    }
                    BCR2000EasyParMode::Increment => {
                        bcl.push_str(&format!("increment {}\n", self.increment_value));
                    }
                }
            },
            _ => panic!("Sorry only CC is handled")
        }
        bcl
    }
}

#[derive(AsRefStr, Debug)]
pub enum BCR2000ButtonMode {
    #[strum(serialize = "down")]
    Down,
    #[strum(serialize = "updown")]
    Updown,
    #[strum(serialize = "toggle")]
    Toggle,
}

pub struct BCR2000Button {
    pub(crate) id: u8,
    pub(crate) showvalue: Option<bool>,
    pub(crate) default: Option<bool>,
    pub(crate) easypar: Option<BCR2000EasyPar>,
    pub(crate) mode: Option<BCR2000ButtonMode>,
}

impl BCR2000Button {
    fn valid(&self) -> Vec<String> {
        let mut errors: Vec<String> = Vec::new();
        if !(1..=64).contains(&self.id) {
            errors.push(format!("Button id is invalid, should be between 1 and 64, was {}",self.id));
        }
        errors
    }

    fn to_bcl(&self) -> String {
        if !self.valid().is_empty() {
            panic!("{:?}", self.valid());
        }
        let mut bcl = String::with_capacity(64);
        bcl.push_str(&format!("$button {}\n", self.id));
        if self.easypar.is_some() {
            println!("Yes it is some");
            bcl.push_str(&self.easypar.as_ref().unwrap().to_bcl());
        }
        bcl
    }
}

pub struct BCR2000State {
    pub(crate) global: Option<BCR2000Global>,
    pub(crate) preset: Option<BCR2000Preset>,
    pub(crate) buttons: Vec<BCR2000Button>
}

static BCR_PREAMBLE: &'static str = "$rev R\n";
static BCR_END: &'static str = "$end\n";

impl BCR2000State {
    pub(crate) fn to_bcl(&self) -> String {
        let mut bcl = String::with_capacity(128);
        bcl.push_str(BCR_PREAMBLE);
        if self.global.is_some() {
            let global = self.global.as_ref().unwrap();
            bcl.push_str(&global.to_bcl());
        }
        if self.preset.is_some() {
            let preset = self.preset.as_ref().unwrap();
            bcl.push_str(&preset.to_bcl());
        }
        for button in self.buttons.iter() {
            bcl.push_str(&button.to_bcl());
        }
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
