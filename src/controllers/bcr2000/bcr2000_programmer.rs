// Almost all of this is coming from the BC-Midi-Implementation v1.2.9
// From https://mountainutilities.eu/bc2000

use crate::controllers::midi_programmer::MidiProgrammer;
use std::collections::HashMap;
use std::error::Error;
use std::str;
use crate::controllers::bcr2000::bcr2000_bcl::{BCLContent, BCLErrors, BCLReplyContent};
use crate::controllers::bcr2000::bcr2000_commands::BCR2000Commands;
pub(crate) use crate::controllers::bcr2000::bcr2000_messages::{ConvertibleMesssageContent, Message, RawContent, StringContent};

pub struct BCR2000Programmer {
    midi_programmer: MidiProgrammer,
}

impl BCR2000Programmer {
    pub(crate) fn new() -> BCR2000Programmer {
        let mut prog = BCR2000Programmer {
            midi_programmer: MidiProgrammer::new(String::from("BCR2000 MIDI 1")),
        };
        match prog.midi_programmer.connect() {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err),
        }
        prog
    }

    fn decode_prefix(msg: Vec<u8>) -> Message<RawContent> {
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

    fn send_message<T: ConvertibleMesssageContent>(&mut self, message: Message<T>) -> Vec<u8> {
        let mut msg: Vec<u8> = Vec::with_capacity(16);
        msg.push(0xf0);
        msg.append(&mut message.manufacturer.to_vec());
        msg.push(message.device_id);
        msg.push(message.model);
        msg.push(message.command);
        msg.append(&mut message.content.to_vec());
        msg.push(0xf7);
        let reply = self.midi_programmer.send_sysex(&*msg);
        if reply.is_err() {
            panic!("Invalid return type for ID")
        }
        reply.unwrap()
    }

    pub(crate) fn send_id(&mut self) -> Result<String, Box<dyn Error>> {
        let sent_msg: Message<RawContent> = Message::for_all_bcr(BCR2000Commands::Id, RawContent::empty());
        let msg = self.send_message(sent_msg);
        if msg.len() == 0 {
            panic!("No response received")
        }
        let msg = BCR2000Programmer::decode_prefix(msg);

        if msg.command != BCR2000Commands::IdReply.to_u8() {
            panic!("Invalid response type")
        }

        let output = msg.new_with_content(StringContent {
            value: str::from_utf8(&msg.content.content)
                .expect("Cannot get content")
                .to_owned(),
        });
        Ok(output.content.value)
    }

    pub(crate) fn send_bcl_text(&mut self, text: &str) -> Result<String, Box<dyn Error>> {
        let mut index = 0;
        let mut lines: HashMap<u16, &str> = HashMap::new();
        for line in text.split('\n') {
            lines.insert(index, line);
            let sent_msg: Message<BCLContent> =
                Message::for_all_bcr(BCR2000Commands::BCL, BCLContent::new(index, line));
            index += 1;
            let msg = self.send_message(sent_msg);
            if msg.len() == 0 {
                panic!("No response received")
            }
            let msg = BCLReplyContent::from_vec(msg);

            if msg.command != BCR2000Commands::BCLReply.to_u8() {
                panic!("Invalid response type")
            }
            match msg.content.error {
                BCLErrors::OK => (),
                _ => {
                    println!(
                        "BCL at index: {} - error: {:?} for line: {}",
                        msg.content.index, msg.content.error, lines[&msg.content.index]
                    );
                }
            }
        }
        Ok("ok".parse().unwrap())
    }
}
