use std::ops::RangeInclusive;

use rand::{thread_rng, Rng};

// TODO: Cache the ThreadRng for each thread.

/// Return a randol number in [0.0, 1.0).
pub fn random() -> f64 {
    thread_rng().gen_range(0.0..1.0)
}

/// Return a random number in the given range.
#[allow(dead_code)]
pub fn random_range(range: RangeInclusive<f64>) -> f64 {
    thread_rng().gen_range(range)
}
