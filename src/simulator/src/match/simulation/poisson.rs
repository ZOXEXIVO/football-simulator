use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;

use std::f64::consts::E;

#[inline]
fn get_uniform() -> f64 {
    Uniform::from(0.0..1.0).sample(&mut rand::thread_rng())
}

//poisson distribution
pub fn get_poisson(lambda: f64) -> u32{
    let mut p: f64 = 1.0;
    let mut l = E.powf(lambda);
    
    let mut k = 0;
    
    loop {
        k += 1;
        
        p = get_uniform();
                
        if p < l {
            break;
        }
    }
    
    k - 1
}