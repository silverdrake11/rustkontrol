use portmidi::types::MidiEvent;

use std::fmt;

const NUM_SLIDERS: usize = 8;


#[derive(Debug, PartialEq)] 
pub enum Korg {
  S, M, R,
  Knob, Slider,
  TrackLeft, TrackRight,
  Cycle,
  Set, MarkerLeft, MarkerRight,
  Rewind, FastForward, Stop, Play, Record,
}


// A more readable representation of a midi event
pub struct Message {
    pub name: Korg,
    pub group: u8,
    pub value: u8,
    pub timestamp: u32,
}

impl Message {

  // Creates Message from MidiEvent
  pub fn new(midi_event: &MidiEvent) -> Message {

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

  pub fn to_bool(&self) -> bool {
    match self.value {
        0 => false,
        _ => true,
    }
  }
}

impl fmt::Display for Message {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    let group = self.group;
    let value = self.value;
    let timestamp = self.timestamp;
    let pressed = self.to_bool();

    let name = &self.name;
    match name {
      Korg::Knob | Korg::Slider => 
          write!(f, "Group {}, Value {}, {:?}, Time {}", group, value, name, timestamp),
      _ => write!(f, "Group {}, Value {}, {:?}, Time {}", group, pressed, name, timestamp),
    }
  }
}


#[derive(Copy, Clone, Default)]
pub struct Control {
  pub knob: u8,
  pub slider: u8,
  pub solo: bool,
  pub mute: bool,
  pub rec: bool,
}

#[derive(Default)]
pub struct NK2 {
  pub controls: [Control; NUM_SLIDERS],
  pub track_left: bool,
  pub track_right: bool,
  pub cycle: bool,
  pub set: bool,
  pub marker_left: bool,
  pub marker_right: bool,
  pub rewind: bool,
  pub fast_forward: bool,
  pub stop: bool,
  pub play: bool,
  pub record: bool,
}

impl NK2 {

  //Updates NK2 with the midi message
  pub fn update(&mut self, message: &Message) {

    let value_idx = (message.group - 1) as usize; // The groups are off by 1
    let pressed = message.to_bool();

    match message.name {
      Korg::Knob        => self.controls[value_idx].knob = message.value,
      Korg::Slider      => self.controls[value_idx].slider = message.value,
      Korg::S           => self.controls[value_idx].solo = pressed,
      Korg::M           => self.controls[value_idx].mute = pressed,
      Korg::R           => self.controls[value_idx].rec = pressed,
      Korg::TrackLeft   => self.track_left = pressed, 
      Korg::TrackRight  => self.track_right = pressed,
      Korg::Cycle       => self.cycle = pressed,
      Korg::Set         => self.set = pressed, 
      Korg::MarkerLeft  => self.marker_left = pressed, 
      Korg::MarkerRight => self.marker_right = pressed,
      Korg::Rewind      => self.rewind = pressed, 
      Korg::FastForward => self.fast_forward = pressed, 
      Korg::Stop        => self.stop = pressed, 
      Korg::Play        => self.play = pressed,
      Korg::Record      => self.record = pressed,
    }
  }

}