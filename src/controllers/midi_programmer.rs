use std::error::Error;
use std::sync::{Arc, Mutex};

pub(crate) trait MidiProgrammer {
    fn connect(&mut self) -> Result<(), Box<dyn Error>>;
    fn send_sysex(&mut self, message: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>;
    fn receive_callback(stamp: u64, message: &[u8], received: &mut Arc<Mutex<Vec<u8>>>);
}
