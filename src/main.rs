extern crate portmidi;

use portmidi::PortMidi;
use portmidi::types::MidiEvent;

use std::fmt::Display;
use std::time;
use std::thread;


const DEVICE_NUM: i32 = 3;
const MAX_MESSAGES: usize = 128;
const NUM_SLIDERS: usize = 8;


#[derive(Copy, Clone, Default)]
struct Control {
  knob: u8,
  slider: u8,
  solo: bool,
  mute: bool,
  rec: bool,
}

#[derive(Default)]
struct NK2 {
  controls: [Control; NUM_SLIDERS],
  track_left: bool,
  track_right: bool,
  cycle: bool,
  set: bool,
  marker_left: bool,
  marker_right: bool,
  rewind: bool,
  fast_forward: bool,
  stop: bool,
  play: bool,
  record: bool,
}


#[derive(Debug, PartialEq)] 
enum Korg {
  S, M, R,
  Knob, Slider,
  TrackLeft, TrackRight,
  Cycle,
  Set, MarkerLeft, MarkerRight,
  Rewind, FastForward, Stop, Play, Record,
}

struct Message {
    name: Korg,
    group: u8,
    value: u8,
    timestamp: u32,
}

impl Message {
  fn to_bool(&self) -> bool {
    match self.value {
        0 => false,
        _ => true,
    }
  }
}

// Value is templated so that the buttons can print boolean values and the sliders ints
fn print_helper<T: Display>(group: u8, value: T, name: &Korg, timestamp: u32) {
  println!("Group {}, Value {}, {:?}, Time {}", group, value, name, timestamp);
}

fn print_message(message: &Message) {
  match message.name {
    Korg::Knob | Korg::Slider => print_helper(message.group, 
        message.value, &message.name, message.timestamp),
    _ => print_helper(message.group, 
          message.to_bool(), &message.name, message.timestamp),
  };
}

// Converts the midi event into a more readable struct
fn decode_midi(midi_event: &MidiEvent) -> Message {

  let timestamp = midi_event.timestamp;

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

  Message {name:control, group:group, value:data2, timestamp:timestamp}
}


//Updates NK2 with the midi message
fn write_message(message: &Message, nk2: &mut NK2) {

  let value_idx = (message.group - 1) as usize; // The groups are off by 1
  let pressed = message.to_bool();

  match message.name {
    Korg::Knob        => nk2.controls[value_idx].knob = message.value,
    Korg::Slider      => nk2.controls[value_idx].slider = message.value,
    Korg::S           => nk2.controls[value_idx].solo = pressed,
    Korg::M           => nk2.controls[value_idx].mute = pressed,
    Korg::R           => nk2.controls[value_idx].rec = pressed,
    Korg::TrackLeft   => nk2.track_left = pressed, 
    Korg::TrackRight  => nk2.track_right = pressed,
    Korg::Cycle       => nk2.cycle = pressed,
    Korg::Set         => nk2.set = pressed, 
    Korg::MarkerLeft  => nk2.marker_left = pressed, 
    Korg::MarkerRight => nk2.marker_right = pressed,
    Korg::Rewind      => nk2.rewind = pressed, 
    Korg::FastForward => nk2.fast_forward = pressed, 
    Korg::Stop        => nk2.stop = pressed, 
    Korg::Play        => nk2.play = pressed,
    Korg::Record      => nk2.record = pressed,
  }
}


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

        let message = decode_midi(&event);

        print_message(&message);
        write_message(&message, &mut nk2);

      }
    }

    //println!("no message");
    thread::sleep(timeout); //Sleep here so CPU is not 100%
  }
}

