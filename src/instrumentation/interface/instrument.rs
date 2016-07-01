pub mod Instrument
{
    use sequencing::tonation::note::NoteCollections::RawNote;

    pub trait PlayableInstrument
    {
        fn play(&mut self, &RawNote);
    }
}


pub mod InstrumentWrapper
{
    use sequencing::tonation::note::NoteCollections::RawNote;
    use instrumentation::interface::instrument::Instrument;
    use std::thread;
    
    ///T represents an implemented instrument that has the play
    ///function: The play function returns an Option<???> which
    ///at a later point will be able to unwrap to modify portions
    ///of metadata in the Sequence or PartialSequence.
    #[derive(Copy, Clone)]
    pub struct InstrumentWrapper<T: Instrument::PlayableInstrument>
    {
        internal_instrument: T,
        //To implement: voice count.
    }

    impl <T: Instrument::PlayableInstrument>InstrumentWrapper<T>{
        pub fn new(instrument: T) -> InstrumentWrapper<T>
        {
            InstrumentWrapper
            {
                internal_instrument: instrument,
            }
        }

        ///Safe play returns no value and panics if a note has
        ///not been implemented for this particular instrument.
        ///Reimplement as similar result to playable instrument at
        ///later point.
        pub fn play(&mut self, note: &RawNote) 
        {
            self.internal_instrument.play(note);
            //if blah blah returned a none value, panic.
        }

        ///Safe play returns no value and handles an improper play
        ///argument by doing nothing.
        pub fn safe_play(&mut self, note: &RawNote) -> bool
        {
            self.internal_instrument.play(note);
            //if blah blah returns none value, false, else...
            true
        }

        //For debug uses only
        pub fn reveal_internal_instrument_DEBUG(&mut self) -> &mut T
        {
            &mut self.internal_instrument
        }
    }
}

pub mod TestPlugin
{
    use sequencing::tonation::note::NoteCollections::RawNote;
    use instrumentation::interface::instrument::Instrument;
    use ears::{Sound, AudioController};
    use schedule_recv::oneshot_ms;
    use std::thread;

    pub struct TestSampler
    {
        internal_audio: Sound,
        audio_scaler: f32,
        cache: f32,
    }

    impl Instrument::PlayableInstrument for TestSampler
    {
        fn play(&mut self, note: &RawNote)
        {
            if self.internal_audio.is_playing()
            {
                self.internal_audio.set_position([0f32, 0f32, 0f32]);
            }

            if self.cache != note.pitch_hz * 0.0015 * 8.0 * self.audio_scaler
            {
                self.internal_audio.set_pitch(note.pitch_hz * 0.0015 * 8.0 * self.audio_scaler);
            }
            self.play_core(&note);
        }

        
    }

    impl TestSampler
    {
        pub fn from_audio(sample_path: &str) -> Option<TestSampler>
        {
            let sample = Sound::new(sample_path);

            match sample
            {
                Some(data) => {
                    Some(TestSampler{
                        internal_audio: data,
                        audio_scaler: 1.0,
                        cache: 0f32,
                    })
                },
                None => {
                    None
                }
            }
        }

        fn play_core(&mut self, note: &RawNote)
        {
            let local_timer = oneshot_ms(note.length);
            self.internal_audio.play();
            local_timer.recv().unwrap();
            self.internal_audio.stop();
        }

        pub fn change_pitch(&mut self, value: f32)
        {
            self.audio_scaler = value;
            self.internal_audio.set_pitch(value);
        }
    }


}
