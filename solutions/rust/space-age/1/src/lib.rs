// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration{
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds : s }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        let earth_year_seconds = 31_557_600.0;
        d.seconds as f64 / (earth_year_seconds * Self::ORBITAL_PERIOD)
    }
}

pub struct Mercury;
impl Planet for Mercury {
    const ORBITAL_PERIOD: f64 = 0.2408467;
}

pub struct Venus;
impl Planet for Venus {
    const ORBITAL_PERIOD: f64 = 0.61519726;
}

pub struct Earth;
impl Planet for Earth {
    const ORBITAL_PERIOD: f64 = 1.0;
}

pub struct Mars;
impl Planet for Mars {
    const ORBITAL_PERIOD: f64 = 1.8808158;
}

pub struct Jupiter;
impl Planet for Jupiter {
    const ORBITAL_PERIOD: f64 = 11.862615;
}

pub struct Saturn;
impl Planet for Saturn {
    const ORBITAL_PERIOD: f64 = 29.447498;
}

pub struct Uranus;
impl Planet for Uranus {
    const ORBITAL_PERIOD: f64 = 84.016846;
}

pub struct Neptune;
impl Planet for Neptune {
    const ORBITAL_PERIOD: f64 = 164.79132;
}