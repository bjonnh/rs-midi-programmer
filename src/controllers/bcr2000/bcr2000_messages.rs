use std::{fmt, str};
use crate::controllers::bcr2000::bcr2000_bcl::BCLReplyContent;
use crate::controllers::bcr2000::bcr2000_commands::BCR2000Commands;

pub trait ConvertibleMesssageContent {
    fn to_vec(&self) -> Vec<u8>;
    fn from_vec(raw: Vec<u8>) -> Message<BCLReplyContent>;
}

pub struct Message<T: ConvertibleMesssageContent> {
    pub(crate) manufacturer: [u8; 3],
    pub(crate) device_id: u8,
    pub(crate) model: u8,
    pub(crate) command: u8,
    pub(crate) content: T,
}

pub struct RawContent {
    pub(crate) content: Vec<u8>,
}

impl RawContent {
    pub(crate) fn empty() -> RawContent {
        RawContent {
            content: Vec::new(),
        }
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
    pub(crate) value: String,
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

impl<T: ConvertibleMesssageContent> Message<T> {
    pub(crate) fn for_all_bcr(command: BCR2000Commands, data: T) -> Message<T> {
        Message {
            manufacturer: [0x00, 0x20, 0x32],
            device_id: 0x7f,
            model: 0x15,
            command: command.to_u8(),
            content: data,
        }
    }

    pub(crate) fn decode_as_raw(msg: Vec<u8>) -> Message<RawContent> {
        if msg[0] != 0xf0 {
            panic!("Invalid SysEX preamble expected 0xf0")
        }
        if msg[msg.len() - 1] != 0xf7 {
            panic!("Invalid SysEX closing expected 0xf7")
        }
        Message {
            manufacturer: msg[0..3].try_into().expect("Cannot get manufacturer"),
            device_id: msg[4].try_into().expect("Cannot get id"),
            model: msg[5].try_into().expect("Cannot get model"),
            command: msg[6].try_into().expect("Cannot get command"),
            content: RawContent {
                content: msg[7..msg.len() - 1].to_vec(),
            },
        }
    }

    pub(crate) fn new_with_content<U: ConvertibleMesssageContent>(&self, content: U) -> Message<U> {
        Message {
            manufacturer: self.manufacturer,
            device_id: self.device_id,
            model: self.model,
            command: self.command,
            content,
        }
    }
}
