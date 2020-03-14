extern crate math;

use crate::data::currency;
use crate::misc;
use math::round;

pub struct CurrencyInfo {
    short: String,
    long: String,
}

pub fn compact() -> CurrencyInfo {
    let index = misc::random_data_index(currency::SHORT);
    CurrencyInfo {
        short: currency::SHORT[index].to_string(),
        long: currency::LONG[index].to_string(),
    }
}

pub fn short() -> String {
    misc::random_data(currency::SHORT).to_string()
}

pub fn long() -> String {
    misc::random_data(currency::LONG).to_string()
}

pub fn price(min: f64, max: f64) -> f64 {
    round::floor(misc::random::<f64>(min, max), 2)
}
