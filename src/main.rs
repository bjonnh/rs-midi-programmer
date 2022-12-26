#![feature(option_result_contains)]

use crate::controllers::bcr2000::bcr2000_programmer::BCR2000Programmer;
use crate::controllers::bcr2000::bcr2000_state::{BCR2000Global, BCR2000Preset, BCR2000State};

mod controllers;

fn main() {
    let mut controller = BCR2000Programmer::new();

    println!("Getting ID");
    match controller.send_id() {
        Ok(received) => println!("Received: {}", received),
        Err(err) => println!("Error: {}", err)
    }
    println!("Sending our first BCL test");

    let mut pr = BCR2000Preset::default();
    /*pr.learnOutput = Vec::from([Vec::from([0x00,0x01]),
        Vec::from([0x02,0x03])]);*/
    let st = BCR2000State {
        global: BCR2000Global::default(),
        preset: pr
    };
    println!("{}", st.to_bcl());

    match controller.send_bcl_text(&st.to_bcl()) {
        Ok(received) => println!("Received: {}", received),
        Err(err) => println!("Error: {}", err)
    }
    println!("Done");
}
