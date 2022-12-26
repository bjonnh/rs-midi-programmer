use std::error::Error;
use midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use std::time;
use std::thread::sleep;
use std::sync::{Arc, Mutex};
use crate::controllers::midi_programmer::MidiProgrammer;

impl MidiProgrammer for ControllerProgrammer {
    fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let midi_in = MidiInput::new("midi-programmer")?;
        let midi_out = MidiOutput::new("midi-programmer")?;

        for p in midi_out.ports() {
            let port_name = midi_out.port_name(&p)?;
            if port_name.contains(&"BCR2000 MIDI 1") {
                println!("Found BCR output port named: {}", port_name);
                self.conn_out = Some(midi_out.connect(&p, "sysex")?);
                break;
            }
        }

        for p in midi_in.ports() {
            let port_name = midi_in.port_name(&p)?;
            if port_name.contains(&"BCR2000 MIDI 1") {
                println!("Found BCR in port named: {}", port_name);
                self.conn_in = Some(midi_in.connect(&p, "input_read", ControllerProgrammer::receive_callback, self.response.clone())?);
                break;
            }
        }
        Ok(())
    }

    fn send_sysex(&mut self, message: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        if self.conn_out.is_none() || self.conn_in.is_none() {
            panic!("Sorry, the device wasn't found or you didn't call connect!")
        }
        *self.response.lock().unwrap() = Vec::new();
        if self.conn_out.is_some() {
            self.conn_out.as_mut().unwrap().send(message).unwrap();
        }
        // We wait up to one second for a reply
        let mut wait = 50;
        let pause = time::Duration::from_millis(1);
        while wait > 0 {
            if !self.response.lock().unwrap().is_empty() {
                break;
            }
            sleep(pause);
            wait = wait-1;
        }
        Ok(self.response.lock().unwrap().to_vec())
    }

    fn receive_callback(_: u64, message: &[u8], received: &mut Arc<Mutex<Vec<u8>>>) {
        *received.lock().unwrap() = message.to_vec();
    }
}

pub struct ControllerProgrammer {
    conn_in: Option<MidiInputConnection<Arc<Mutex<Vec<u8>>>>>,
    conn_out: Option<MidiOutputConnection>,
    response: Arc<Mutex<Vec<u8>>>
}

impl ControllerProgrammer {
    pub(crate) fn new() -> ControllerProgrammer {
        return ControllerProgrammer { ..Default::default() }
    }
}

impl Default for ControllerProgrammer {
    fn default() -> ControllerProgrammer {
        ControllerProgrammer {
            conn_in: None,
            conn_out: None,
            response: Arc::new(Mutex::new(Vec::new()))
        }
    }
}
