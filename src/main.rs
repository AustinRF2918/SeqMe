use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

extern crate ears;
extern crate schedule_recv;

mod note;
mod instrument;

use note::Note;
use instrument::{InstrumentWrapper, TestPlugin};

fn main() {
    let sequence = Note::build_sequence("B:B:C:C:C:DS:DS:C");

    let mut sampler = InstrumentWrapper::InstrumentWrapper::new(TestPlugin::TestSampler::from_audio("../debug/sampe7.wav").unwrap());
    sampler.reveal_internal_instrument().change_pitch(0.3);

    let mut nb = Note::NoteBuilder::new();
    nb.set_note(Note::NotePitch::B);
    nb = nb.length((200.0) as u32);


    let note_sequence : Vec<Note::Note>= sequence.into_iter().map(|x|{
        let mut temp = nb.clone();
        temp.set_note(x);
        temp.safe_build()
    }).collect();

    thread::spawn(move ||
    {
        for i in note_sequence.clone().into_iter()
        {
            sampler.safe_play(&i);
        }
    })


}

