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
        d.seconds as f64 / EARTH_YEAR_SECONDS * Self::ORBITAL_PERIOD
    }
}

macro_rules! impl_planet {
    ($planet_name:ident, $orbital_period:expr) => {
        pub struct $planet_name;

        impl Planet for $planet_name {
            const ORBITAL_PERIOD: f64 = $orbital_period;

            fn years_during(d: &Duration) -> f64 {
                d.seconds as f64 / (EARTH_YEAR_SECONDS * Self::ORBITAL_PERIOD)
            }
        }
    };
}

impl_planet!(Mercury, 0.2408467);
impl_planet!(Venus, 0.61519726);
impl_planet!(Earth, 1.0);
impl_planet!(Mars, 1.8808158);
impl_planet!(Jupiter, 11.862615);
impl_planet!(Saturn, 29.447498);
impl_planet!(Uranus, 84.016846);
impl_planet!(Neptune, 164.79132);
