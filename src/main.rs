extern crate ears;
extern crate schedule_recv;

mod sequencing;
mod instrumentation;

use sequencing::tonation::note::NoteGeneration::NoteBuilder;

use sequencing::tonation::note::NoteCollections::IncompleteNote;
use sequencing::tonation::note::NoteCollections::NoteResult;
use sequencing::tonation::note::NoteCollections::RawNote;
use sequencing::tonation::note::NotePrimitives::Register;
use sequencing::tonation::note::NotePrimitives::Semitone;
use sequencing::timing::timing::Beat::BeatValue;

use instrumentation::interface::instrument::InstrumentWrapper::InstrumentWrapper;
use instrumentation::interface::instrument::TestPlugin;

use sequencing::sequencing::partialsequencer::PartialSequencer::PartialSequencer;

use std::thread;


fn main() {
    let mut children = vec![];
    children.push(thread::spawn(move || {
        let sampler = TestPlugin::TestSampler::from_audio("../debug/bassdrum.aiff").unwrap();
        //
        let mut nb = NoteBuilder::new();
        let mut x = PartialSequencer::<TestPlugin::TestSampler>::new(130.0, sampler);
        nb = nb.semitone(Semitone::A).register(Register::C2).offset(0.0).amplitude(15.0).length(x.local_beat_builder.u64_from_beats(1, 1).unwrap() as u32);
        for i in 0..4
        {
            match nb.build()
            {
                NoteResult::Incomplete(x) =>
                {
                },
                NoteResult::Complete(note) =>
                {
                    nb = nb.amplitude(i as f32);
                    x.push_time_to_note(i, 4, note);
                },
            }
        }
        loop
        {
            x.play_sequence_DEBUG();
        }
    }));

    children.push(thread::spawn(move || {
        let sampler = TestPlugin::TestSampler::from_audio("../release/pianoc.aiff").unwrap();
        let mut nb = NoteBuilder::new();
        let mut x = PartialSequencer::<TestPlugin::TestSampler>::new(130.0, sampler);
        nb = nb.semitone(Semitone::C).register(Register::C2).offset(0.0).amplitude(15.0).length(x.local_beat_builder.u64_from_beats(1, 1).unwrap() as u32);
        for i in 0..4
        {
            match nb.build()
            {
                NoteResult::Incomplete(_) =>
                {
                },
                NoteResult::Complete(note) =>
                {
                    if i == 0
                    {
                        nb = nb.semitone(Semitone::B).register(Register::C2);
                    }
                    if i == 1
                    {
                        nb = nb.semitone(Semitone::CSharp).register(Register::C2);
                    }
                    if i == 2
                    {
                        nb = nb.semitone(Semitone::A).register(Register::C2);
                    }
                    if i == 3
                    {
                        nb = nb.semitone(Semitone::A).register(Register::C2);
                    }
                    if i == 4
                    {
                        nb = nb.semitone(Semitone::A).register(Register::C2);
                    }
                    nb = nb.amplitude(i as f32);
                    match nb.build()
                    {
                        NoteResult::Complete(note) =>
                        {
                            x.push_time_to_note(i, 4, note);
                        }
                        _ => {}
                    }
                },
            }
        }
        loop
        {
            x.play_sequence_DEBUG();
        }
    }));

    for i in children
    {
        i.join();
    }


    /*
    let internal_beat = BeatValue::from_bpm(126.0);
    let mut children = vec![];
    children.push(thread::spawn(move || {
    let mut nb_seq1 = NoteBuilder::new();
    nb_seq1 = nb_seq1.length(internal_beat.u64_from_beats(16, 16).unwrap() as u32);
    let sequence = nb_seq1.build_sequence_DEBUG("A:B:A:B:A:A:A:A:A:A:A:A:A:A:A:A");
    let mut sampler = InstrumentWrapper::InstrumentWrapper::new(TestPlugin::TestSampler::from_audio("../debug/sampe4.wav").unwrap());
    sampler.reveal_internal_instrument_DEBUG().change_pitch(0.19);
        for i in sequence
        {
            sampler.play(&i);
        }
    }));

    children.push(thread::spawn(move || {
    let mut nb_seq1 = NoteBuilder::new();
    nb_seq1 = nb_seq1.length(internal_beat.u64_from_beats(4, 8).unwrap() as u32);
    let sequence = nb_seq1.build_sequence_DEBUG("E:E:E:E:E:E:E:E:E:E:E:E:E:E:E:E");
    let mut sampler = InstrumentWrapper::InstrumentWrapper::new(TestPlugin::TestSampler::from_audio("../debug/clap.wav").unwrap());
    sampler.reveal_internal_instrument_DEBUG().change_pitch(0.3);
        for i in sequence
        {
            sampler.play(&i);
        }
    }));

    children.push(thread::spawn(move || {
    let mut nb_seq1 = NoteBuilder::new();
    nb_seq1 = nb_seq1.length(internal_beat.u64_from_beats(4, 4).unwrap() as u32);
    let sequence = nb_seq1.build_sequence_DEBUG("A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A");
    let mut sampler = InstrumentWrapper::InstrumentWrapper::new(TestPlugin::TestSampler::from_audio("../debug/bass.wav").unwrap());
    sampler.reveal_internal_instrument_DEBUG().change_pitch(0.3);
        for i in sequence
        {
            sampler.play(&i);
        }
    }));

    children.push(thread::spawn(move || {
    let mut nb_seq1 = NoteBuilder::new();
    nb_seq1 = nb_seq1.length(internal_beat.u64_from_beats(8, 2).unwrap() as u32);
    let sequence = nb_seq1.build_sequence_DEBUG("A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A:A");
    let mut sampler = InstrumentWrapper::InstrumentWrapper::new(TestPlugin::TestSampler::from_audio("../debug/hats.wav").unwrap());
    sampler.reveal_internal_instrument_DEBUG().change_pitch(0.35);
        for i in sequence
        {
            sampler.play(&i);
        }
    }));
    for i in children{
        let x = i.join();
    }
    */
}

