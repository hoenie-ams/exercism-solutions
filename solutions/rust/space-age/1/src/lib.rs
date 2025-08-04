#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        // (60 seconds * 60 minutes * 24 hours * 365.25 days)
        Duration(seconds as f64 / (31557168.0))
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

#[macro_export]
macro_rules! planet {
    ($name:ident,$orbital_period:expr) => {
        pub struct $name;
        impl Planet for $name {
            fn years_during(d: &Duration) -> f64 {
                d.0 / $orbital_period
            }
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
