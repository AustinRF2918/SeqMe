pub mod PartialSequencer
{
    use std::collections::HashMap;
    use note::Note;
    use instrument::{InstrumentWrapper, TestPlugin, Instrument};
    use timing::Beat::BeatValue;

    pub struct PartialSequencer<T: Instrument::PlayableInstrument>
    {
        local_beat_builder: BeatValue,
        local_note_builder: Note::NoteBuilder,
        instrument: InstrumentWrapper::InstrumentWrapper<T>,
        note_hash: HashMap<u64, Note::Note>
    }

    impl<T: Instrument::PlayableInstrument>PartialSequencer<T>
    {
        pub fn new(bpm: f32, instrument: T) -> PartialSequencer<T>
        {
            PartialSequencer
            {
                internal_beat: BeatValue::from_bpm(bpm),
                local_note_builder: Note::NoteBuilder::new(),
                instrument: InstrumentWrapper::InstrumentWrapper::new(instrument),
                note_hash: HashMap::new(),
            }
        }

        pub fn internal_instrument(&mut self) -> &mut InstrumentWrapper::InstrumentWrapper<T>
        {
            &mut self.instrument
        }

        pub fn push_time_to_note(&mut self, bar: u64, bar_divisions: u64, note: Note::Note)
        {
            self.note_hash.insert(self.internal_beat.u64_from_beats(bar, bar_divisions).unwrap(), note);
        }

        pub fn push_time_to_note_from_str(&mut self, s : &str)
        {
            let x: Vec<&str> = s.split(":").collect();

            if x.len() != 5
            {
                panic!("Corrupted note meta data.")
            }
            else
            {
                let bar_precision = x.iter().nth(0).unwrap();
                let bar = x.iter().nth(1).unwrap();
                let bar_subdivision = x.iter().nth(2).unwrap();
                let note_pitch = x.iter().nth(3).unwrap();
                let note_octave = x.iter().nth(4).unwrap();

                let note = self.local_note_builder.from_str(*note_pitch + "/" + *note_octave);
            }
        }
    }
}
