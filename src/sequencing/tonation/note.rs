///Primitives values for notes: this includes information regarding general
///tonation of sounds. That being pitch. Semitone holds the twelve tone
///variable, but in the future a multiple tone interface should be built
///around note in order to allow for multiple scales, maybe even reduced
///scales. This could be done by creating a STUCTURE instead that provides
///a function that returns a tuple of herz and maybe a string from an implemented
///scale object.
pub mod NotePrimitives
{
    ///Semitone: Temporary note pitch object that is used to define the western
    ///scale note.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Semitone
    {
        C,
        CSharp,
        D,
        DSharp,
        E,
        F,
        FSharp,
        G,
        GSharp,
        A,
        ASharp,
        B,
    }

    ///semitone_from_str(s: &str) -> Option<NotePrimitives::Semitone> Simple generator to take a string argument and return a Semitone (see NotePrimitives::{Semitone}.
    pub fn semitone_from_str(s: &str) -> Option<Semitone>
    {
        match s
        {
            "C" => {Some(Semitone::C)},
            "CS" => {Some(Semitone::CSharp)},
            "D" => {Some(Semitone::D)},
            "DS" => {Some(Semitone::DSharp)},
            "E" => {Some(Semitone::E)},
            "F" => {Some(Semitone::F)},
            "FS" => {Some(Semitone::FSharp)},
            "G" => {Some(Semitone::G)},
            "GS" => {Some(Semitone::GSharp)},
            "A" => {Some(Semitone::A)},
            "AS" => {Some(Semitone::ASharp)},
            "B" => {Some(Semitone::B)},
            _ => {None},
        }
    }


    ///Register: Decides the octave of our note..
    #[derive(Clone, Copy, Debug)]
    pub enum Register
    {
        C0,
        C1,
        C2,
        C3,
        C4,
        C5,
    }

    ///register_from_str(s: &str) -> Option<Register>: Takes a string
    ///and attempts to parse it for register (octaves), returns a
    ///Option<NotePrimitives::Register
    pub fn register_from_str(s: &str) -> Option<Register>
    {
        match s
        {
            "C0" => {Some(Register::C0)},
            "C1" => {Some(Register::C1)},
            "C2" => {Some(Register::C2)},
            "C3" => {Some(Register::C3)},
            "C4" => {Some(Register::C4)},
            "C5" => {Some(Register::C5)},
            _ => {None},
        }
    }

    ///Conversion function for primitives to herz. Takes our
    ///Semitone as p and our Register as r and then uses
    ///simple math to determine the herz as an f32 value.
    pub fn primitives_to_herz(p: Semitone, r: Register) -> f32
    {
        let base_pitch = match p{
            Semitone::C => {16.35},
            Semitone::CSharp => {17.32},
            Semitone::D => {18.35},
            Semitone::DSharp => {19.45},
            Semitone::E => {20.80},
            Semitone::F => {21.83},
            Semitone::FSharp => {23.12},
            Semitone::G => {24.50},
            Semitone::GSharp => {25.96},
            Semitone::A => {27.50},
            Semitone::ASharp => {29.14},
            Semitone::B => {30.87},
        };

        let base_register = match r{
            Register::C0 => {1.0},
            Register::C1 => {2.0},
            Register::C2 => {4.0},
            Register::C3 => {8.0},
            Register::C4 => {16.0},
            Register::C5 => {32.0},
        };

        base_pitch * base_register
    }
}

///NoteGeneration: This module which derives from Note directly acts
///as a generate facility for notes that allows us to easily create
///return note objects.
pub mod NoteGeneration
{
    use sequencing::tonation::note::NotePrimitives::Semitone;
    use sequencing::tonation::note::NotePrimitives::Register;
    use sequencing::tonation::note::NotePrimitives;
    use sequencing::tonation::note::NoteCollections::IncompleteNote;
    use sequencing::tonation::note::NoteCollections::RawNote;
    use sequencing::tonation::note::NoteCollections::NoteResult;

    ///Setter macro that allows us to use a builder pattern to return a
    ///object that has been modified from it's previous call with a
    ///single attribute: think about JavaScript returning objects.
    ///Takes $name and $typ, which is the name of the internal variable
    ///and the type of the internal variable and generates said function.
    macro_rules! setter {
        ($( $name: ident, $typ: ty), *) => (
            $(
                pub fn $name(mut self, $name: $typ) -> NoteBuilder
                {
                    self.internal_note.$name = Some($name);
                    self
                }
            )*
        )
    }

    //Macro clearer: Simple clearer macro for single variables inside
    //of our note builder so that we can automatically clear out our
    //builder and make new things
    macro_rules! clearer {
        ( $name: ident) => (
            self.internal_note.$name = None;
        );
    }

    ///NoteBuilder: This is an object that allows us to use builder style
    ///construction of note objects. Has multiple functions for maintaining
    ///and creating notes.
    #[derive(Clone, Copy, Debug)]
    pub struct NoteBuilder
    {
        internal_note: IncompleteNote,
    }

    impl NoteBuilder
    {
        ///NoteBuilder::new(): Generates a new NoteBuilder object.
        pub fn new() -> NoteBuilder
        {
            NoteBuilder{
                internal_note: IncompleteNote::new(),
            }
        }

        ///NoteBuilder.clear(): Automatically clears all our variables
        ///inside of our NoteBuilder to None values so that we can
        ///create a new, uniq note that we will build nater
        pub fn clear(&mut self)
        {
            self.internal_note.semitone = None;
            self.internal_note.pitch_hz = None;
            self.internal_note.register = None;
            self.internal_note.offset = None;
            self.internal_note.amplitude = None;
            self.internal_note.length = None;
        }

        ///NoteBuilder.build_sequence_DEBUG(arg: &str) -> Vec<RawNote>: Builds a simple
        ///sequence for testing.
        pub fn build_sequence_DEBUG(&mut self, arg: &str) -> Vec<RawNote>
        {
            let mut ret_val = Vec::<RawNote>::new();
            let x : Vec<&str> = arg.split(":").collect();
            for i in x
            {
                ret_val.push(RawNote::new(NotePrimitives::semitone_from_str(i).unwrap(), NotePrimitives::Register::C3, 0.0, 15.0, self.internal_note.length.unwrap()));
            }
            ret_val
        }

        ///NoteBuider.build() -> NoteResult: Attempts to build
        ///the note that is currently in our internal_note variable
        ///if it fails, it will return a IncompleteNote (actually a
        ///deep copy of the internal note at the current time), or
        ///A raw note value, a note with all the proper values inside
        ///unwrapped options.
        pub fn build(&mut self) -> NoteResult
        {
            match (self.internal_note.semitone, self.internal_note.register)
            {
                (Some(x), Some(y)) =>
                {
                    self.internal_note.pitch_hz = Some(NotePrimitives::primitives_to_herz(x, y))
                }
                _ =>
                {
                }
            }
            match self.internal_note.to_raw()
            {
                Some(note) =>
                {
                    NoteResult::Complete(note)
                },
                None =>
                {
                    NoteResult::Incomplete(self.internal_note.clone())
                }
            }
        }

        ///NoteBuilder.semitone(semitone: Semitone): Generalized setter for semitone.
        setter!(semitone, Semitone);

        ///NoteBuilder.register(register: Register): Generalized setter for offset.
        setter!(register, Register);

        ///NoteBuilder.offset(offset: f32): Generalized setter for register.
        setter!(offset, f32);

        ///NoteBuilder.amplitude(amplitude: f32): Generalized setter for amplitude.
        setter!(amplitude, f32);

        ///NoteBuilder.length(length: u32): Generalized setter for length.
        setter!(length, u32);
    }

}

pub mod NoteCollections{
    use sequencing::tonation::note::NotePrimitives::Semitone;
    use sequencing::tonation::note::NotePrimitives::Register;
    use sequencing::tonation::note::NotePrimitives;

    use std::fmt::Error;
    use std::fmt::Debug;
    use std::fmt::Formatter;
    use std::fmt::Display;

    ///NoteResult: Enum which handles whether a note is complete or not.
    pub enum NoteResult
    {
        Complete(RawNote),
        Incomplete(IncompleteNote),
    }

    ///IncompleteNote: Structure that represents a note which may or may not have the proper
    ///fields initialized yet. This is helped by our note builder which manages the handling
    ///of incomplete data.
    #[derive(Debug, Copy, Clone)]
    pub struct IncompleteNote
    {
        pub semitone: Option<Semitone>,
        pub pitch_hz: Option<f32>,
        pub register: Option<Register>,
        pub offset: Option<f32>,
        pub amplitude: Option<f32>,
        pub length: Option<u32>,
    }

    ///has_value<T>: Simple utility function for checking if a type has a value. Just saves space.
    fn has_value<T>(o: Option<T>) -> bool
    {
        match o
        {
            Some(_) =>
            {
                true
            }
            None =>
            {
                false
            }
        }
    }

    impl IncompleteNote
    {
        ///IncompleteNote::new() -> IncompleteNote: Generates a new incomplete note with zeroed out memory.
        pub fn new() -> IncompleteNote
        {
            IncompleteNote{
                register: None,
                offset: None,
                amplitude: None,
                length: None,
                semitone: None,
                pitch_hz: None,
            }
        }

        ///IncompleteNote::from_options() -> IncompleteNote: Generates a new incomplete note that may, or may not
        ///be convertable to a raw note: Our note generator handles the ability to generate
        ///to a note.
        pub fn from_options(r: Option<Register>, a: Option<f32>, l: Option<u32>, p: Option<Semitone>) -> IncompleteNote
        {
            let mut ret = IncompleteNote{
                register: r,
                offset: None,
                amplitude: a,
                length: l,
                semitone: p,
                pitch_hz: None,
            };

            ret.pitch_hz = match(ret.register, ret.semitone)
            {
                (Some(x), Some(y)) =>
                {
                    Some(NotePrimitives::primitives_to_herz(y, x))
                }
                _ =>
                {
                    None
                }
            };

            ret
        }

        ///IncompleteNote.to_raw() -> Option<RawNote>: Generates the possibility of a
        ///raw note which can have data directly drawn from it. See: struct RawNote.
        pub fn to_raw(&self) -> Option<RawNote>
        {
            if has_value(self.semitone) &&
                has_value(self.pitch_hz) &&
                has_value(self.register) &&
                has_value(self.offset) &&
                has_value(self.amplitude) &&
                has_value(self.length)
            {
                Some
                (
                    RawNote{
                    semitone: self.semitone.unwrap(),
                    pitch_hz: self.pitch_hz.unwrap(),
                    register: self.register.unwrap(),
                    offset: self.offset.unwrap(),
                    amplitude: self.amplitude.unwrap(),
                    length: self.length.unwrap(),
                    }
                )
            }
            else
            {
                None
            }
        }
    }

    ///Raw note is the type that is returned when our notebuilder has been passed
    ///good information: it contains raw data that resembles midi data that can
    ///at a later point actually be placing into a sequencer and converted into
    ///a midi sequence.
    #[derive(Copy, Clone, Debug)]
    pub struct RawNote
    {
        pub semitone: Semitone,
        pub pitch_hz: f32,
        pub register: Register,
        pub offset: f32,
        pub amplitude: f32,
        pub length: u32,
    }

    impl RawNote
    {
        ///RawNote::new(): Simple constructer that takes p: Semitone, which is in the
        ///primitives module, ph, f32, which should be a calculated herz value from p,
        ///r: Register value which is also in the primitives module, o: 32, which
        ///designates the offset of our note, a: f32, which designates the amplitude of
        ///our note, and l: u32, which designates the length of our note
        pub fn new(p: Semitone, r: Register, o: f32, a: f32, l: u32) -> RawNote
        {
            RawNote{
                semitone: p,  
                pitch_hz: NotePrimitives::primitives_to_herz(p, r),
                register: r,
                offset: o,
                amplitude: a,
                length: l,
            }
        }
    }

    ///RawNote::fmt()...: Trait that allows us to print a formatted raw note.
    impl Display for RawNote
    {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
        {
            writeln!(f, "Pitch: {:?}", self.semitone);
            writeln!(f, "Pitch as herz: {}", self.pitch_hz);
            writeln!(f, "Register: {:?}", self.register);
            writeln!(f, "Offset: {}", self.offset);
            writeln!(f, "Amplitude: {}", self.amplitude);
            writeln!(f, "Length: {}", self.length)
        }
    }

    
}
