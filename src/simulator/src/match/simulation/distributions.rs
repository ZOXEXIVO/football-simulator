use rand::prelude::ThreadRng;
use rand_distr::{Distribution, Gamma, Normal, Uniform};

use libm::{log, pow, sin};
use std::f64::consts::E;

#[inline]
fn uniform_random() -> f64 {
    let uniform_distribution = Uniform::new(0.0, 1.0);
    uniform_distribution.sample(&mut rand::thread_rng())
}

pub(crate) fn normal_random(mu: f64, sigma: f64) -> f64 {
    Normal::new(mu, sigma)
        .unwrap()
        .sample(&mut rand::thread_rng())
}

pub fn random_gamma(alpha: f64, beta: f64) -> f64 {
    let mut new_alpha = 0.000001;
    if alpha != 0.0 {
        new_alpha = alpha;
    }

    Gamma::new(new_alpha, beta)
        .unwrap()
        .sample(&mut rand::thread_rng())
}
