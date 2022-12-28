use strum_macros::FromRepr;
use std::str;
use crate::controllers::bcr2000::bcr2000_messages::{ConvertibleMesssageContent, Message};
pub(crate) use crate::controllers::bcr2000::bcr2000_programmer::RawContent;

#[derive(FromRepr, Debug)]
#[allow(dead_code)]
pub enum BCLErrors {
    OK,
    // 0
    UnknownToken,
    // 1
    DataWithoutToken,
    // 2
    ArgumentMissing,
    // 3
    WrongModel,
    // 4
    WrongRevision,
    // 5
    MissingRevision,
    // 6
    InternalError,
    // 7 (you will never see that)
    ModeMissing,
    // 8
    BadItemIndex,
    // 9
    NotANumber,
    // 10
    ValueOutOfRange,
    // 11
    InvalidArgument,
    // 12
    InvalidCommand,
    // 13
    WrongNumberOfArgs,
    // 14
    TooMuchMidiOutputData,
    // 15
    AlreadyDefined,
    // 16 (you will never see that)
    PresetMissing,
    // 17 (you will never see that)
    PresetTooComplex,
    // 18 (preset size is bigger than 4344 bytes)
    WrongPreset,
    // 19 (you will never see that)
    UnknownPresetError,
    // 20
    UnknownPresetCheck,
    // 21
    InvalidMessageIndex,
    // 22
    WrongContext,
    // 23
    UnknownError, // 24
}

pub struct BCLContent {
    index: u16,
    text: String,
}

impl BCLContent {
    pub(crate) fn new(index: u16, text: &str) -> BCLContent {
        BCLContent {
            index,
            text: text.parse().unwrap(),
        }
    }
}

impl ConvertibleMesssageContent for BCLContent {
    fn to_vec(&self) -> Vec<u8> {
        let msb: u8 = (self.index >> 8) as u8;
        let lsb: u8 = (self.index & 0xff) as u8;
        let mut msg: Vec<u8> = Vec::with_capacity(self.text.len() + 2);
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
    pub(crate) index: u16,
    pub(crate) error: BCLErrors,
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
            error: BCLErrors::from_repr(rawdata[2] as usize).unwrap(),
        })
    }
}
