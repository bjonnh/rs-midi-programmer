use crate::controllers::midi_programmer::MidiProgrammer;

pub struct LaunchkeyProgrammer {
    midi_programmer: MidiProgrammer,
}

impl LaunchkeyProgrammer {
    pub(crate) fn new() -> LaunchkeyProgrammer {
        let mut prog = LaunchkeyProgrammer {
            midi_programmer: MidiProgrammer::new(String::from("BCR2000 MIDI 1")),
        };
        match prog.midi_programmer.connect() {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err),
        }
        prog
    }
}
