pub mod Note{

    use std::fmt::Error;
    use std::fmt::Debug;
    use std::fmt::Formatter;
    use std::fmt::Display;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum NotePitch
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

    fn hz_from_pitch(p: NotePitch, r: Register) -> f32
    {
        let base_pitch = match p{
            NotePitch::C => {16.35},
            NotePitch::CSharp => {17.32},
            NotePitch::D => {18.35},
            NotePitch::DSharp => {19.45},
            NotePitch::E => {20.80},
            NotePitch::F => {21.83},
            NotePitch::FSharp => {23.12},
            NotePitch::G => {24.50},
            NotePitch::GSharp => {25.96},
            NotePitch::A => {27.50},
            NotePitch::ASharp => {29.14},
            NotePitch::B => {30.87},
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

    #[derive(Clone, Copy, Debug)]
    pub struct NoteBuilder
    {
        internal_note: Note,
    }

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

    impl NoteBuilder
    {
        pub fn new() -> NoteBuilder
        {
            NoteBuilder{
                internal_note: Note::new(NotePitch::C),
            }
        }

        pub fn set_note(&mut self, p: NotePitch)
        {
            self.internal_note.pitch = p;
            self.internal_note.pitch_hz = hz_from_pitch(p, Register::C4);
        }

        pub fn build(&self) -> Result<Note, IncompleteNote>
        {
            match self.internal_note.check_completion(){
                Err(err) => Err(err),
                Ok(()) => Ok(self.internal_note.clone()),
            }
        }

        pub fn safe_build(&self) -> Note
        {
            match self.internal_note.check_completion(){
                Err(err) => err.to_default(),
                Ok(()) => self.internal_note.clone(),
            }
        }

        setter!(register, Register);
        setter!(offset, f32);
        setter!(amplitude, f32);
        setter!(length, u32);
    }

    #[derive(Debug)]
    pub struct IncompleteNote
    {
        pub pitch: NotePitch,
        pub pitch_hz: f32,
        pub register: Option<Register>,
        pub offset: Option<f32>,
        pub amplitude: Option<f32>,
        pub length: Option<u32>,
    }

    impl IncompleteNote
    {
        pub fn new(r: Option<Register>, o: Option<f32>, a: Option<f32>, l: Option<u32>, p: NotePitch, ph: f32) -> IncompleteNote
        {
            IncompleteNote{
                register: r,
                offset: o,
                amplitude: a,
                length: l,
                pitch: p,
                pitch_hz: ph,
            }
        }

        pub fn to_note_builder(self) -> NoteBuilder
        {
            let mut nb = NoteBuilder::new();
            nb.set_note(self.pitch);

            let nb = match self.register
            {
                Some(reg) => {nb.register(reg)},
                None => {nb},
            };

            let nb = match self.offset
            {
                Some(reg) => {nb.offset(reg)},
                None => {nb},
            };

            let nb = match self.amplitude
            {
                Some(reg) => {nb.amplitude(reg)},
                None => {nb},
            };

            let nb = match self.length
            {
                Some(reg) => {nb.length(reg)},
                None => {nb},
            };

            nb
        }

        pub fn to_default(self) -> Note
        {
            let mut nb = NoteBuilder::new();
            nb.set_note(self.pitch);

            let nb = match self.register
            {
                Some(reg) => {nb.register(reg)},
                None => {nb.register(Register::C4)},
            };

            let nb = match self.offset
            {
                Some(reg) => {nb.offset(reg)},
                None => {nb.offset(0.0)},
            };

            let nb = match self.amplitude
            {
                Some(reg) => {nb.amplitude(reg)},
                None => {nb.amplitude(1.0)},
            };

            let nb = match self.length
            {
                Some(reg) => {nb.length(reg)},
                None => {nb.length(1000)},
            };

            nb.build().unwrap()
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Note
    {
        pub pitch: NotePitch,
        pub pitch_hz: f32,
        pub register: Option<Register>,
        pub offset: Option<f32>,
        pub amplitude: Option<f32>,
        pub length: Option<u32>,
    }

    impl Note
    {
        pub fn new(p: NotePitch) -> Note
        {
        Note{
            pitch: p,  
            pitch_hz: hz_from_pitch(p, Register::C4),
            register: None,
            offset: None,
            amplitude: None,
            length: None,
        }
        }



        pub fn check_completion(&self) -> Result<(), IncompleteNote>{
            let r = self.register;
            let o = self.offset;
            let a = self.amplitude;
            let l = self.length;

            match (r, o, a, l) 
            {
                (Some(_), Some(_), Some(_), Some(_)) => {Ok(())},
                _ => {Err(IncompleteNote::new(r, o, a, l, self.pitch, self.pitch_hz))},
            }

        }
    }

    impl Display for Note
    {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error>
        {
            writeln!(f, "A note.")
        }
    }

    pub fn build_sequence(arg: &str) -> Vec<NotePitch>
    {
        let mut ret_val = Vec::<NotePitch>::new();
        let x = arg.split(":");
        for i in x
        {
            match i{
                    "C" => {ret_val.push(NotePitch::C)},
                    "CS" => {ret_val.push(NotePitch::CSharp)},
                    "D" => {ret_val.push(NotePitch::D)},
                    "DS" => {ret_val.push(NotePitch::DSharp)},
                    "E" => {ret_val.push(NotePitch::E)},
                    "F" => {ret_val.push(NotePitch::F)},
                    "FS" => {ret_val.push(NotePitch::FSharp)},
                    "G" => {ret_val.push(NotePitch::G)},
                    "GS" => {ret_val.push(NotePitch::GSharp)},
                    "A" => {ret_val.push(NotePitch::A)},
                    "AS" => {ret_val.push(NotePitch::ASharp)},
                    "B" => {ret_val.push(NotePitch::B)},
                    _ => {},
            }
        }
        ret_val
    }
}
