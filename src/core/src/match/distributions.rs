use rand_distr::{Distribution, Normal, Uniform};

#[inline]
#[allow(dead_code)]
pub(crate) fn random(low: f64, high: f64) -> f64 {
    let uniform_distribution = Uniform::new(low, high);
    uniform_distribution.sample(&mut rand::thread_rng())
}

#[inline]
#[allow(dead_code)]
pub(crate) fn uniform_random() -> f64 {
    let uniform_distribution = Uniform::new(0.0, 1.0);
    uniform_distribution.sample(&mut rand::thread_rng())
}

#[inline]
#[allow(dead_code)]
pub(crate) fn normal_random(mu: f64, sigma: f64) -> f64 {
    Normal::new(mu, sigma)
        .unwrap()
        .sample(&mut rand::thread_rng())
}
