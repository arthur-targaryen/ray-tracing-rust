use std::ops::Range;

use rand::{thread_rng, Rng};

// TODO: Cache the ThreadRng for each thread.

/// Return a random number in [0.0, 1.0).
pub fn random() -> f64 {
    thread_rng().gen_range(0.0..1.0)
}

/// Return a random number in the given range.
pub fn random_range(range: Range<f64>) -> f64 {
    thread_rng().gen_range(range)
}
