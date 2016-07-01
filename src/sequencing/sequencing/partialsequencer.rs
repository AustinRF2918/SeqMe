pub mod PartialSequencer
{
    use std::collections::HashMap;

    use sequencing::tonation::note::NoteCollections::RawNote;
    use sequencing::tonation::note::NoteGeneration::NoteBuilder;

    use instrumentation::interface::instrument::InstrumentWrapper;
    use instrumentation::interface::instrument::TestPlugin;
    use instrumentation::interface::instrument::Instrument;

    use sequencing::timing::timing::Beat::BeatValue;
    use schedule_recv::periodic_ms;
    use schedule_recv::oneshot_ms;

    extern crate timer;
    use std::sync::mpsc::channel;
    use std::thread;

    pub struct PartialSequencer<T: Instrument::PlayableInstrument>
    {
        pub local_beat_builder: BeatValue,
        local_note_builder: NoteBuilder,
        instrument: InstrumentWrapper::InstrumentWrapper<T>,
        pub note_hash: HashMap<u64, RawNote>
    }

    impl<T: Instrument::PlayableInstrument>PartialSequencer<T>
    {
        pub fn new(bpm: f32, instrument: T) -> PartialSequencer<T>
        {
            PartialSequencer
            {
                local_beat_builder: BeatValue::from_bpm(bpm),
                local_note_builder: NoteBuilder::new(),
                instrument: InstrumentWrapper::InstrumentWrapper::new(instrument),
                note_hash: HashMap::new(),
            }
        }

        pub fn push_time_to_note(&mut self, bar: u64, bar_divisions: u64, note: RawNote)
        {
            self.note_hash.insert(self.local_beat_builder.u64_from_beats(bar_divisions, bar).unwrap(), note);
        }

        pub fn play_sequence_DEBUG(&mut self)
        {
            let mut time = 0;
            for j in 0..self.local_beat_builder.u64_from_beats(1, 1).unwrap()
            {
                match self.note_hash.get(&time)
                {
                    Some(x) =>
                    {
                        self.instrument.play(x);
                    },
                    None =>
                    {
                    },
                };
                time = time + 1;
            }
        }

    }
}
