// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_YEAR_SECONDS: f64 = 31_557_600.0;

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64 = 1.0;

    fn years_during(d: &Duration) -> f64 {
        //todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
        d.seconds as f64 / EARTH_YEAR_SECONDS * Self::ORBITAL_PERIOD
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const ORBITAL_PERIOD: f64 = 0.2408467;

    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (EARTH_YEAR_SECONDS * Mercury::ORBITAL_PERIOD)
    }
}
impl Planet for Venus {
    const ORBITAL_PERIOD: f64 = 0.61519726;

    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (EARTH_YEAR_SECONDS * Venus::ORBITAL_PERIOD)
    }
}
impl Planet for Earth {
    const ORBITAL_PERIOD: f64 = 1.0;

    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (EARTH_YEAR_SECONDS * Earth::ORBITAL_PERIOD)
    }
}
impl Planet for Mars {
    const ORBITAL_PERIOD: f64 = 1.8808158;

    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (EARTH_YEAR_SECONDS * Mars::ORBITAL_PERIOD)
    }
}
impl Planet for Jupiter {
    const ORBITAL_PERIOD: f64 = 11.862615;

    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (EARTH_YEAR_SECONDS * Jupiter::ORBITAL_PERIOD)
    }
}
impl Planet for Saturn {
    const ORBITAL_PERIOD: f64 = 29.447498;

    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (EARTH_YEAR_SECONDS * Saturn::ORBITAL_PERIOD)
    }
}
impl Planet for Uranus {
    const ORBITAL_PERIOD: f64 = 84.016846;

    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (EARTH_YEAR_SECONDS * Uranus::ORBITAL_PERIOD)
    }
}
impl Planet for Neptune {
    const ORBITAL_PERIOD: f64 = 164.79132;

    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (EARTH_YEAR_SECONDS * Neptune::ORBITAL_PERIOD)
    }
}
