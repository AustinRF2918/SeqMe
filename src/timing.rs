pub mod Beat
{
    use timing::Timing;

    #[derive(Copy, Clone, Hash, PartialEq, Eq)]
    pub struct BeatValue
    {
        time_per_bar: Timing::Time,
    }

    impl BeatValue
    {
        pub fn new(time: Timing::Time) -> BeatValue
        {
            BeatValue
            {
                time_per_bar: time,
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
                    Some(x / prec * beats)
                }
                None =>
                {
                    None
                }
            }
        }
    }
}

pub mod Timing
{
    macro_rules! setter{
        ($( $name: ident), *) => (
            $(
                pub fn $name(mut self, $name: u16) -> TimeBuilder
                {
                    self.internal_timer.$name = Some($name);
                    println!("{:?}", self.internal_timer.$name);
                    self
                }
            )*
        )
    }

    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
    pub struct Time
    {
        pub h: Option<u16>,
        pub m: Option<u16>,
        pub s: Option<u16>,
        pub ms: Option<u16>,
    }

    enum TimeValue
    {
        Complete(Time),
        Incomplete(Time),
    }

    impl Time
    {
        pub fn as_ms(&self) -> Option<u64>
        {
            match (self.h, self.m, self.s, self.ms)
            {
                (Some(h_value), Some(m_value), Some(s_value), Some(ms_value)) => {
                    Some((h_value as u64 + m_value as u64 + s_value as u64 + ms_value as u64))
                },
                _ => {None},
            }
        }

        fn new() -> Time
        {
            Time{
              h: None,
              m: None,
              s: None,
              ms: None,
            }
        }

        pub fn from_ms(value: u64) -> Time
        {
            let ms = (value % 1000) as u16;
            println!("MS2: {}", ms);
            let mut qr = value / 1000;
            let s = (qr % 60) as u16;
            println!("S: {}", s);
            qr = (qr % 60) / 60;
            println!("qr: {}", qr);
            let m = (qr % 60) as u16; 
            println!("M: {}", m);
            qr = (qr % 60) / 60;
            println!("qr: {}", qr);
            let h = (qr % 60) as u16;
            println!("H: {}", h);

            Time{
              h: Some(h),
              m: Some(m),
              s: Some(s),
              ms: Some(ms),
            }
        }

        fn check_completion(&self) -> bool 
        {
            let h = self.h;
            let m = self.h;
            let s = self.h;
            let ms = self.h;

            match (h, m, s, ms)
            {
                (Some(_), Some(_), Some(_), Some(_)) => {true},
                _ => {false},
            }
        }
    }

    #[derive(Debug)]
    pub struct TimeBuilder
    {
        internal_timer: Time,
    }

    impl TimeBuilder
    {
        pub fn new() -> TimeBuilder
        {
            TimeBuilder
            {
                internal_timer: Time{
                h: None,
                m: None,
                s: None,
                ms: None,
                }
            }
        }

        pub fn build(&self) -> TimeValue
        {
            let timer = self.internal_timer;
            match (timer.h, timer.m, timer.s, timer.ms)
            {
                (Some(_), Some(_), Some(_), Some(_)) => {
                    TimeValue::Complete(self.internal_timer.clone())
                }
                _ => {
                    TimeValue::Incomplete(self.internal_timer.clone())
                }
            }
        }

        pub fn safe_build(&self) -> Time
        {
            let mut h = self.internal_timer.h;
            let mut m = self.internal_timer.m;
            let mut s = self.internal_timer.s;
            let mut ms = self.internal_timer.ms;

            match h
            {
                Some(_) => {},
                None => {h = Some(0)},
            }

            match m
            {
                Some(_) => {},
                None => {m = Some(0)},
            }

            match s
            {
                Some(_) => {},
                None => {s = Some(0)},
            }

            match ms 
            {
                Some(_) => {},
                None => {ms = Some(0)},
            }

            Time
            {
                h: h,
                m: m,
                s: s,
                ms: ms,
            }
        }

        setter!(ms);
        setter!(h);
        setter!(m);
        setter!(s);
    }

}
