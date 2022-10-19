// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const BASE_PRODUCTION: u8 = 221;

fn success_rate(speed: u8) -> f64 {
    match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=u8::MAX => 0.77,
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    (BASE_PRODUCTION * speed) as f64 * success_rate(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
