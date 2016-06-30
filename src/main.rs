extern crate ears;
extern crate schedule_recv;

mod note;
mod instrument;
mod timing;
mod partialsequencer;

use note::Note;
use instrument::{InstrumentWrapper, TestPlugin};
use timing::Beat::BeatValue;
use partialsequencer::PartialSequencer;
use std::thread;

fn main() {
    let internal_beat = BeatValue::from_bpm(126.0);

    let mut children = vec![];


    children.push(thread::spawn(move || {
    let mut nb_seq1 = Note::NoteBuilder::new();
    nb_seq1 = nb_seq1.length(internal_beat.u64_from_beats(16, 128).unwrap() as u32);
    let sequence = Note::build_sequence("A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A");
    let mut sampler = InstrumentWrapper::InstrumentWrapper::new(TestPlugin::TestSampler::from_audio("../debug/sampe4.wav").unwrap());
    sampler.reveal_internal_instrument().change_pitch(0.19);
    let note_sequence : Vec<Note::Note>= sequence.into_iter().map(|x|{
        nb_seq1.set_note(x);
        nb_seq1.safe_build()
    }).collect();
        for i in note_sequence
        {
            sampler.play(&i);
        }
    }));

    children.push(thread::spawn(move || {
    let mut nb_seq1 = Note::NoteBuilder::new();
    nb_seq1 = nb_seq1.length(internal_beat.u64_from_beats(4, 8).unwrap() as u32);
    let sequence = Note::build_sequence("E:E:E:E:E:E:E:E:E:E:E:E:E:E:E:E");
    let mut sampler = InstrumentWrapper::InstrumentWrapper::new(TestPlugin::TestSampler::from_audio("../debug/clap.wav").unwrap());
    sampler.reveal_internal_instrument().change_pitch(0.3);
    let note_sequence : Vec<Note::Note>= sequence.into_iter().map(|x|{
        nb_seq1.set_note(x);
        nb_seq1.safe_build()
    }).collect();
        for i in note_sequence
        {
            sampler.play(&i);
        }
    }));

    children.push(thread::spawn(move || {
    let mut nb_seq1 = Note::NoteBuilder::new();
    nb_seq1 = nb_seq1.length(internal_beat.u64_from_beats(4, 4).unwrap() as u32);
    let sequence = Note::build_sequence("A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A");
    let mut sampler = InstrumentWrapper::InstrumentWrapper::new(TestPlugin::TestSampler::from_audio("../debug/bass.wav").unwrap());
    sampler.reveal_internal_instrument().change_pitch(0.3);
    let note_sequence : Vec<Note::Note>= sequence.into_iter().map(|x|{
        nb_seq1.set_note(x);
        nb_seq1.safe_build()
    }).collect();
        for i in note_sequence
        {
            sampler.play(&i);
        }
    }));

    children.push(thread::spawn(move || {
    let mut nb_seq1 = Note::NoteBuilder::new();
    nb_seq1 = nb_seq1.length(internal_beat.u64_from_beats(8, 2).unwrap() as u32);
    let sequence = Note::build_sequence("A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A");
    let mut sampler = InstrumentWrapper::InstrumentWrapper::new(TestPlugin::TestSampler::from_audio("../debug/hats.wav").unwrap());
    sampler.reveal_internal_instrument().change_pitch(0.35);
    let note_sequence : Vec<Note::Note>= sequence.into_iter().map(|x|{
        nb_seq1.set_note(x);
        nb_seq1.safe_build()
    }).collect();
        for i in note_sequence
        {
            sampler.play(&i);
        }
    }));
    for i in children{
        let x = i.join();
    }
}

