extern crate portmidi;

use portmidi::PortMidi;

use std::time;
use std::thread;

mod korg;
use korg::Message;
use korg::NK2;

const DEVICE_NUM: i32 = 3;
const MAX_MESSAGES: usize = 128;


fn main() {

  let context = PortMidi::new().unwrap();
  let info = context.device(DEVICE_NUM).unwrap();
  let in_port = context.input_port(info, MAX_MESSAGES).unwrap();

  let timeout = time::Duration::from_millis(1);

  let mut nk2 = NK2::default();

  loop {

    if in_port.poll().unwrap() {

      let events = in_port.read_n(MAX_MESSAGES).unwrap().unwrap();

      for event in events.iter() {

        let message = Message::new(&event);

        println!("{}", message);
        
        nk2.update(&message);

      }
    }

    //println!("no message");
    thread::sleep(timeout); //Sleep here so CPU is not 100%
  }
}

