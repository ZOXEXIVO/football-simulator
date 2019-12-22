use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;

use std::f64::consts::E;
use libm::{log, sin, pow};

#[inline]
fn uniform_random() -> f64 {
    Uniform::from(0.0..1.0).sample(&mut rand::thread_rng())
}

//poisson distribution
pub(crate) fn poisson(lambda: f64) -> u32{
    let mut p: f64 = 1.0;
    let mut l = E.powf(lambda);
    
    let mut k = 0;
    
    loop {
        k += 1;
        
        p = uniform_random();
                
        if p < l {
            break;
        }
    }
    
    k - 1
}

pub(crate) fn normal_random(mu: f64, sigma: f64) -> f64 {
    let u1 = uniform_random();
    let u2 = uniform_random();
    
    let z0 = (-2.0 * log(u1)).sqrt() * sin(2.0 * std::f64::consts::PI * u2);
    
    mu + z0 * sigma
}

pub fn random_gamma(alpha: f64, beta: f64) -> f64 {
    let d = alpha - 1.0 / 3.0;
    let c = 1.0 / (9.0 * d).sqrt();
    
    let mut z = 0.0;
    
    loop {
        let mut x =  0.0;
        let mut v = 0.0;
        
        loop {
            x = normal_random(0.0, 1.0);
            v = 1.0 + c * x;
            println!("{} v =", v);
            if v > 0.0 {
                break;
            }
        }
        println!("{} 1 =", v);
        v = pow(v, 3.0);
        
        let u = uniform_random();
        
        if u < 1.0 - 0.0331 * pow(x, 4.0){
            z = d * v;
            break;
        }
        
        if log(u) < 0.5 * pow(x, 2.0) + d * (1.0 - v + log(v)){
            z = d * v;
            break;
        }
    }
    
    z / beta
}