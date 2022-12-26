// Almost all of this is coming from the BC-Midi-Implementation v1.2.9
// From https://mountainutilities.eu/bc2000

use std::{fmt, str};
use std::collections::HashMap;
use std::error::Error;
use crate::controllers::controller_programmer::ControllerProgrammer;
use crate::controllers::midi_programmer::MidiProgrammer;
use strum_macros;
use strum_macros::EnumIter;
use strum_macros::FromRepr;


pub struct BCR2000Programmer {
    controller_programmer: ControllerProgrammer
}

enum Commands {
    Id,
    IdReply,
    BCL,
    BCLReply,
    PresetSelection,
    FirmwareSend,
    FirmwareReply,
    RequestData,
    RequestSetup,
    RequestPresetName,
    RequestSnapshot,
    SendText
}

impl Commands {
    fn to_u8(&self) -> u8 {
        match self {
            Commands::Id => 0x01,
            Commands::IdReply => 0x02,
            Commands::BCL => 0x20,
            Commands::BCLReply => 0x21,
            Commands::PresetSelection => 0x22,
            Commands::FirmwareSend => 0x34,
            Commands::FirmwareReply => 0x35,
            Commands::RequestData => 0x40,
            Commands::RequestSetup => 0x41,
            Commands::RequestPresetName => 0x42,
            Commands::RequestSnapshot => 0x43,
            Commands::SendText => 0x78
        }
    }
}

#[derive(FromRepr, Debug)]
#[allow(dead_code)]
enum BCLErrors {
    OK, // 0
    UnknownToken, // 1
    DataWithoutToken, // 2
    ArgumentMissing, // 3
    WrongModel, // 4
    WrongRevision, // 5
    MissingRevision, // 6
    InternalError, // 7 (you will never see that)
    ModeMissing, // 8
    BadItemIndex, // 9
    NotANumber, // 10
    ValueOutOfRange, // 11
    InvalidArgument, // 12
    InvalidCommand, // 13
    WrongNumberOfArgs, // 14
    TooMuchMidiOutputData, // 15
    AlreadyDefined, // 16 (you will never see that)
    PresetMissing, // 17 (you will never see that)
    PresetTooComplex, // 18 (preset size is bigger than 4344 bytes)
    WrongPreset, // 19 (you will never see that)
    UnknownPresetError, // 20
    UnknownPresetCheck, // 21
    InvalidMessageIndex, // 22
    WrongContext, // 23
    UnknownError, // 24
}

pub trait ConvertibleMesssageContent {
    fn to_vec(&self) -> Vec<u8>;
    fn from_vec(raw: Vec<u8>) -> Message<BCLReplyContent>;
}

pub struct Message<T: ConvertibleMesssageContent> {
    manufacturer: [u8; 3],
    device_id: u8,
    model: u8,
    command: u8,
    pub(crate) content: T
}

struct RawContent {
    content: Vec<u8>
}

impl RawContent {
    fn empty() -> RawContent {
        RawContent { content: Vec::new() }
    }
}

impl ConvertibleMesssageContent for RawContent {
    fn to_vec(&self) -> Vec<u8> {
        self.content.to_vec()
    }

    fn from_vec(_: Vec<u8>) -> Message<BCLReplyContent> {
        panic!("This is not an input format!")
    }
}

pub struct StringContent {
    value: String
}

impl StringContent {
    fn new(text: &str) -> StringContent {
        StringContent { value: text.parse().unwrap() }
    }
}

impl ConvertibleMesssageContent for StringContent {
    fn to_vec(&self) -> Vec<u8> {
        self.value.clone().into_bytes()
    }

    fn from_vec(_: Vec<u8>) -> Message<BCLReplyContent> {
        panic!("This is not an input format!")
    }
}

impl fmt::Display for StringContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl <T: ConvertibleMesssageContent> Message<T> {
    fn for_all_bcr(command: Commands, data: T) -> Message<T> {
        Message {
            manufacturer: [0x00, 0x20, 0x32],
            device_id: 0x7f,
            model: 0x15,
            command: command.to_u8(),
            content: data
        }
    }

    fn decode_as_raw(msg: Vec<u8>) -> Message<RawContent> {
        if msg[0] != 0xf0 {
            panic!("Invalid SysEX preamble expected 0xf0")
        }
        if msg[msg.len()-1] != 0xf7 {
            panic!("Invalid SysEX closing expected 0xf7")
        }
        Message {
            manufacturer: msg[0..3].try_into().expect("Cannot get manufacturer"),
            device_id: msg[4].try_into().expect("Cannot get id"),
            model: msg[5].try_into().expect("Cannot get model"),
            command: msg[6].try_into().expect("Cannot get command"),
            content: RawContent { content: msg[7..msg.len()-1].to_vec() }
        }
    }

    fn new_with_content<U: ConvertibleMesssageContent>(&self, content: U) -> Message<U> {
        Message {
            manufacturer: self.manufacturer,
            device_id: self.device_id,
            model: self.model,
            command: self.command,
            content
        }
    }
}

struct BCLContent {
    index: u16,
    text: String
}

impl BCLContent {
    fn new(index: u16, text: &str) -> BCLContent {
        BCLContent {
            index,
            text: text.parse().unwrap()
        }
    }
}

impl ConvertibleMesssageContent for BCLContent {
    fn to_vec(&self) -> Vec<u8> {
        let msb: u8 = (self.index >> 8) as u8;
        let lsb: u8 = (self.index & 0xff) as u8;
        let mut msg: Vec<u8> = Vec::with_capacity(self.text.len()+2);
        msg.push(msb);
        msg.push(lsb);
        msg.append(&mut self.text.clone().into_bytes());
        msg
    }

    fn from_vec(_: Vec<u8>) -> Message<BCLReplyContent> {
        panic!("This is not an input format!")
    }
}

pub struct BCLReplyContent {
    index: u16,
    error: BCLErrors
}

impl BCLReplyContent {
    fn new(index: u16, error: BCLErrors) -> BCLReplyContent {
        BCLReplyContent {
            index,
            error
        }
    }
}

impl ConvertibleMesssageContent for BCLReplyContent {
    fn to_vec(&self) -> Vec<u8> {
        panic!("This message cannot be sent");
     /*   let msb: u8 = (self.index >> 8) as u8;
        let lsb: u8 = (self.index & 0xff) as u8;
        let mut msg: Vec<u8> = Vec::with_capacity(self.text.len()+2);
        msg.push(msb);
        msg.push(lsb);
        msg.append(&mut self.text.clone().into_bytes());
        msg*/
    }

    fn from_vec(raw: Vec<u8>) -> Message<BCLReplyContent> {
        let rawmsg = Message::<RawContent>::decode_as_raw(raw);
        let rawdata = rawmsg.content.content.to_vec();
        rawmsg.new_with_content(BCLReplyContent {
            index: ((rawdata[0] as u16) << 8) + rawdata[1] as u16,
            error: BCLErrors::from_repr(rawdata[2] as usize).unwrap()
        })
    }
}


pub type MessageID = Message<StringContent>;

impl BCR2000Programmer {
    pub(crate) fn new() -> BCR2000Programmer {
        let mut prog = BCR2000Programmer {
            controller_programmer: ControllerProgrammer::new()
        };
        match prog.controller_programmer.connect() {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err)
        }
        prog
    }

    fn decode_prefix(msg: Vec<u8>) -> Message<RawContent> {
        if msg[0] != 0xf0 {
            panic!("Invalid SysEX preamble expected 0xf0")
        }
        if msg[msg.len()-1] != 0xf7 {
            panic!("Invalid SysEX closing expected 0xf7")
        }
        Message {
            manufacturer: msg[0..3].try_into().expect("Cannot get manufacturer"),
            device_id: msg[4].try_into().expect("Cannot get id"),
            model: msg[5].try_into().expect("Cannot get model"),
            command: msg[6].try_into().expect("Cannot get command"),
            content: RawContent { content: msg[7..msg.len()-1].to_vec() }
        }
    }

    fn send_message<T: ConvertibleMesssageContent>(&mut self, message: Message<T>) -> Vec<u8> {
        let mut msg : Vec<u8> = Vec::with_capacity(16);
        msg.push(0xf0);
        msg.append(&mut message.manufacturer.to_vec());
        msg.push(message.device_id);
        msg.push(message.model);
        msg.push(message.command);
        msg.append(&mut message.content.to_vec());
        msg.push(0xf7);
        let reply = self.controller_programmer.send_sysex(&*msg);
        if reply.is_err() {
            panic!("Invalid return type for ID")
        }
        reply.unwrap()
    }

    pub(crate) fn send_id(&mut self) -> Result<String, Box<dyn Error>> {
        let sent_msg: Message<RawContent> = Message::for_all_bcr(Commands::Id, RawContent::empty());
        let msg = self.send_message(sent_msg);
        if msg.len() == 0 {
            panic!("No response received")
        }
        let msg = BCR2000Programmer::decode_prefix(msg);

        if msg.command != Commands::IdReply.to_u8() {
            panic!("Invalid response type")
        }

        let output = msg.new_with_content(
            StringContent {
                value: str::from_utf8(&msg.content.content).expect("Cannot get content")
                    .to_owned()
            }
            );
        Ok(output.content.value)
    }

    pub(crate) fn send_bcl_text(&mut self, text: &str) -> Result<String, Box<dyn Error>> {
        let mut index = 0;
        let mut lines: HashMap<u16, &str> = HashMap::new();
        for line in text.split('\n') {
            lines.insert(index, line);
            let sent_msg: Message<BCLContent> = Message::for_all_bcr(
                Commands::BCL, BCLContent::new(index, line));
            index += 1;
            let msg = self.send_message(sent_msg);
            if msg.len() == 0 {
                panic!("No response received")
            }
            let msg = BCLReplyContent::from_vec(msg);

            if msg.command != Commands::BCLReply.to_u8() {
                panic!("Invalid response type")
            }
            match msg.content.error  {
                BCLErrors::OK => (),
                _ => { println!("BCL at index: {} - error: {:?} for line: {}", msg.content.index, msg.content.error, lines[&msg.content.index]); }
            }
        }
        Ok("ok".parse().unwrap())
    }
}
