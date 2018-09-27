extern crate portmidi;

use portmidi::PortMidi;
use portmidi::types::MidiEvent;

use std::time::Duration;
use std::thread;


const DEVICE_NUM: i32 = 3;
const MAX_MESSAGES: usize = 128;


#[derive(Debug)] // This line allows you to print enum values
enum Korg {
  S, M, R,
  Knob, Slider,
  TrackLeft, TrackRight,
  Cycle,
  Set, MarkerLeft, MarkerRight,
  Rewind, FastForward, Stop, Play, Record,
}


struct Control {
    name: Korg,
    group: u8,
    value: u8,
}

impl Control {
  fn to_bool(&self) -> bool {
    match self.value {
        0 => false,
        _ => true,
    }
  }
}

fn decode_message(midi_event: &MidiEvent) -> Control {

  let data1 = midi_event.message.data1;
  let data2 = midi_event.message.data2;

  let control = match data1 {
      0...7   => Korg::Slider,
      16...23 => Korg::Knob,
      32...39 => Korg::S,
      48...55 => Korg::M,
      64...71 => Korg::R,
      41      => Korg::Play,
      42      => Korg::Stop,
      43      => Korg::Rewind,
      44      => Korg::FastForward,
      45      => Korg::Record,
      46      => Korg::Cycle,
      58      => Korg::TrackLeft,
      59      => Korg::TrackRight,
      60      => Korg::Set,
      61      => Korg::MarkerLeft,
      62      => Korg::MarkerRight,
      _       => Korg::Cycle,
  };

  let group: u8 = match data1 {
      41...46 => 0,
      58...62 => 0,
      _       => data1 % 8 + 1,
  };

  Control {name:control, group:group, value:data2}
}


fn print_control(control: &Control) {
  match control.name {
    Korg::Knob | Korg::Slider => println!("Group {}, Value {}, {:?}", 
        control.group, control.value, control.name),
    _ => println!("Group {}, Value {}, {:?}", 
        control.group, control.to_bool(), control.name),
  };
}


fn main() {

    let context = PortMidi::new().unwrap();

    let timeout = Duration::from_millis(10);

    let info = context.device(DEVICE_NUM).unwrap();
    println!("Listening on: {}) {}", info.id(), info.name());

    let in_port = context.input_port(info, MAX_MESSAGES).unwrap();

    loop {

      if in_port.poll().unwrap() {

        let events = in_port.read_n(MAX_MESSAGES).unwrap().unwrap();

        for event in events.iter() {

          let control = decode_message(&event);

          print_control(&control)
          
        }
      }

      thread::sleep(timeout);
    }
}

