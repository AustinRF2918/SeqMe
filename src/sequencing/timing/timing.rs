///Primitive values for beats: this includes information regarding
///relative timing of a beat: that being, for example, 16ths, 8ths,
///etc. This will also be used at a later point in conjunction with
///beat for using weird time signatures (3/4, etc)
pub mod BeatPrimitives {
    ///Division: Simple divisior based on common note divisions.
    ///Note that this also includes the ability to multiply notes
    ///into spanning times (4 quarternotes = full note)
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Division {
        Whole(u32),
        Half(u32),
        Quarter(u32),
        Eighth(u32),
        Sixteenth(u32),
        ThirtySecond(u32),
        SixtyFourth(u32),
        OneHundredTwentyEighth(u32),
        TwoHundredFiftySixth(u32),
    }

    ///division_from_str(s: &str, r: u32) -> Option<BeatPrimitives::Division>: Easy
    ///way to generate a duration of time from a string and a u32. s should be a
    ///number 1 / 2^n up to 256.
    pub fn division_from_str(s: &str, r: u32) -> Option<Division> {
        match s {
            "1" => Some(Division::Whole(r)),
            "1/2" => Some(Division::Half(r)),
            "1/4" => Some(Division::Quarter(r)),
            "1/8" => Some(Division::Eighth(r)),
            "1/16" => Some(Division::Sixteenth(r)),
            "1/32" => Some(Division::ThirtySecond(r)),
            "1/64" => Some(Division::SixtyFourth(r)),
            "1/128" => Some(Division::OneHundredTwentyEighth(r)),
            "1/256" => Some(Division::TwoHundredFiftySixth(r)),
            _ => None,
        }
    }
}

///BeatGeneration: This module which derives from BeatPrimitive directly acts
///as a generation facility for beats that allows us to easily use a builder
///pattern for building actual beats.
pub mod BeatGeneration {
    use sequencing::timing::timing::BeatPrimitives::Division;
    use sequencing::timing::timing::BeatPrimitives::division_from_str;

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

    ///Clearer macro allows us to simply clear our BeatBuilder of
    ///Cruft accumulated after building of an arbitrary note.
    macro_rules! clearer {
        ( $name: ident) => (
            self.internal_note.$name = None;
        );
    }

    ///BeatBuilder: This is an object that allows us to use builder style
    ///construction of beat objects. Has multiple functions for maintaining
    ///and creating beats.
    #[derive(Clone, Copy, Debug)]
    pub struct BeatBuilder {
        internal_beat: IncompleteBeat;
    }

    impl BeatBuilder {
        ///BeatBuilder::new(): Generates a new BeatBuilder object.
        pub fn new() -> BeatBuilder {
            BeatBuilder {
                internal_beat: IncompleteBeat::new(),
            }
        }

        ///BeatBuilder::clear(): Automatically clears all our
        ///variables inside of our BeatBuilder to None so that we
        ///can create a new, unique beat that we can use later.
        pub fn clear(&mut self) {
            self.internal_beat.whole = None;
            self.internal_beat.half = None;
            self.internal_beat.quarter = None;
            self.internal_beat.eighth = None;
            self.internal_beat.sixteenth = None;
            self.internal_beat.thirty_second = None;
            self.internal_beat.sixty_fourth = None;
            self.internal_beat.one_hundred_twenty_eighth = None;
            self.internal_beat.two_hundred_fifty_sixth = None;
        }

        pub fn build(&mut self) -> BeatResult {
            
        }
    }
}

pub mod Beat
{
    use std::time::Duration;

    #[derive(Copy, Clone, Hash, PartialEq, Eq)]
    pub struct BeatPrimitive
    {
        duration_per_bar: Duration,
    }

    impl BeatValue
    {
        pub fn from_ms(time: Duration) -> BeatValue {
            BeatValue {
            duration_per_bar: time;
            }
        }

        pub fn from_bpm(time: f32) -> BeatValue
        {
            let to_ms = (60000.0 / time) as u64;
            println!("MS: {:?}", to_ms);
            BeatValue
            {
                time_per_bar: Timing::Time::from_ms(to_ms),
            }
        }

        pub fn u64_from_beats(&self, prec: u64, beats: u64) -> Option<u64>
        {
            match self.time_per_bar.as_ms()
            {
                Some(x) =>
                {
                    Some((x / prec ) * beats)
                }
                None =>
                {
                    None
                }
            }
        }
    }
}
