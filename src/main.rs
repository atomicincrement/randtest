use rand::{thread_rng};
use rand::Rng;
use rand_distr::{Normal, Distribution};

use rayon::prelude::*;

type fty = f64;
type ity = i64;
type uty = u64;

use std::f64::consts::LOG2_10;
use std::f64::consts::LOG2_E;
use std::f64::consts::PI;

fn select(a: bool, b: fty, c: fty) -> fty {
    if a {
        b
    } else {
        c
    }
}

fn iabs(i: ity) -> ity {
    i.abs()
}

fn f(f: fty) -> fty {
    f
}

fn from_bits(u: uty) -> fty {
    fty::from_bits(u)
}

fn to_bits(f: fty) -> uty {
    fty::to_bits(f)
}

fn exp2(arg: fty) -> fty {
    let r: fty = arg.round();
    let mul: fty = fty::from_bits(
        (r.mul_add(0x0010000000000000_u64 as fty, 0x3ff0000000000000_u64 as fty)) as uty,
    );
    let x: fty = arg - r;
    (from_bits(4549839347750377909u64))
        .mul_add(x, from_bits(4563827094295188139u64))
        .mul_add(x, from_bits(4576698039041613846u64))
        .mul_add(x, from_bits(4588159642448921967u64))
        .mul_add(x, from_bits(4597823092488205992u64))
        .mul_add(x, from_bits(4604418534717280147u64))
        .mul_add(x, from_bits(4607182418800017408u64))
        * mul
}

pub fn exp(arg: fty) -> fty {
    exp2(arg * LOG2_E)
}

fn negate_on_odd(x: fty, y: fty) -> fty {
    let sign_bit: uty = (((x as ity) & 1) << 63i32) as uty;
    fty::from_bits(sign_bit ^ y.to_bits())
}
fn recip_approx(x: fty) -> fty {
    let y: fty = fty::from_bits(
        ((x.abs().to_bits() as fty).mul_add(-1.0, 0x3ff0000000000000_u64 as fty * 2.0)) as uty,
    );
    (y - 0.08).copysign(x)
}
fn sqrt_approx(x: fty) -> fty {
    let y: fty = fty::from_bits(
        ((x.abs().to_bits() as fty).mul_add(0.5, 0x3ff0000000000000_u64 as fty * 0.5)) as uty,
    );
    y - 0.08
}
fn cbrt_approx(x: fty) -> fty {
    let y: fty = fty::from_bits(
        ((x.abs().to_bits() as fty).mul_add(1.0 / 3.0, 0x3ff0000000000000_u64 as fty * 2.0 / 3.0))
            as uty,
    );
    (y - 0.08).copysign(x)
}

pub fn qnorm(arg: fty) -> fty {
    let x: fty = arg - 0.5;
    let recip: fty = 1.0 / (x * x - 0.25);
    let y: fty = (from_bits(4730221388428958202u64))
        .mul_add(x * x, -from_bits(4731626383781768040u64))
        .mul_add(x * x, from_bits(4727627778628654481u64))
        .mul_add(x * x, -from_bits(4720012863723153492u64))
        .mul_add(x * x, from_bits(4708869911609092829u64))
        .mul_add(x * x, -from_bits(4695087533321972728u64))
        .mul_add(x * x, from_bits(4678670384600451257u64))
        .mul_add(x * x, -from_bits(4658680898319303328u64))
        .mul_add(x * x, from_bits(4635605149421499302u64))
        .mul_add(x * x, from_bits(4578476110820645018u64))
        .mul_add(x * x, from_bits(4611041379213747643u64))
        .mul_add(x * x, -from_bits(4603819697584151827u64))
        * x;
    y * recip
}

/// See https://xorshift.di.unimi.it/splitmix64.c
pub fn runif(index: usize) -> f64 {
    let mut z = (index + 1) as u64 * 0x9e3779b97f4a7c15;
    z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9;
    z = (z ^ (z >> 27)) * 0x94d049bb133111eb;
    z = z ^ (z >> 31);
    from_bits((z >> 2) | 0x3ff0000000000000_u64) - 1.0
}

pub fn rnorm(index: usize) -> f64 {
    qnorm(runif(index) * 0.999 + 0.0005)
}

fn ref_to_usize<T>(t: &T) -> usize {
    unsafe { std::mem::transmute(t) }
}

#[target_feature(enable = "avx2,fma")]
unsafe fn do_par<F: Fn(&mut f64) + Send + Sync + Clone>(d: &mut [f64], f: F) {
    d.par_chunks_mut(0x1000)
        .for_each(|c| c.iter_mut().for_each(f.clone()));
}

#[target_feature(enable = "avx2,fma")]
unsafe fn test_runif(d: &mut [f64]) {
    for (i, d) in d.iter_mut().enumerate() {
        *d = runif(i);
    }
}

#[target_feature(enable = "avx2,fma")]
unsafe fn test_par_runif(d: &mut [f64]) {
    do_par(d, |d| *d = runif(ref_to_usize(d)));
}

#[target_feature(enable = "avx2")]
unsafe fn test_rust_runif(d: &mut [f64]) {
    let mut rng = thread_rng();
    // mean 2, standard deviation 3
    d.iter_mut().for_each(|d| *d = rng.gen::<f64>() );
}

#[target_feature(enable = "avx2")]
unsafe fn test_par_rust_runif(d: &mut [f64]) {
    let normal = Normal::new(1.0, 1.0).unwrap();
    d.par_chunks_mut(0x1000)
        .for_each_init(|| thread_rng(), |rng, chunk| {
            chunk.iter_mut().for_each(|d| *d = rng.gen::<f64>() );
        })
}

#[target_feature(enable = "avx2")]
unsafe fn test_rust_rnorm(d: &mut [f64]) {
    let mut rng = thread_rng();
    // mean 2, standard deviation 3
    let normal = Normal::new(1.0, 1.0).unwrap();
    d.iter_mut().for_each(|d| *d = normal.sample(&mut rng) );
}

#[target_feature(enable = "avx2")]
unsafe fn test_par_rust_rnorm(d: &mut [f64]) {
    let normal = Normal::new(1.0, 1.0).unwrap();
    d.par_chunks_mut(0x1000)
        .for_each_init(|| thread_rng(), |rng, chunk| {
            chunk.iter_mut().for_each(|d| *d = normal.sample(rng) );
        })
}

#[target_feature(enable = "avx2,fma")]
unsafe fn test_rnorm(d: &mut [f64]) {
    for (i, d) in d.iter_mut().enumerate() {
        *d = rnorm(i);
    }
}

#[target_feature(enable = "avx2,fma")]
unsafe fn test_par_rnorm(d: &mut [f64]) {
    do_par(d, |d| *d = rnorm(ref_to_usize(d)));
}

fn main() {
    let mut d = vec![0.0; 1000000];
    for i in 0..10 {
        unsafe {
            let t0 = std::time::Instant::now();
            test_runif(&mut d);
            if i > 5 { println!("runif           {} {:?}", d.len(), t0.elapsed()); }
        }
        unsafe {
            let t0 = std::time::Instant::now();
            test_par_runif(&mut d);
            if i > 5 { println!("par_runif       {} {:?}", d.len(), t0.elapsed()); }
        }
        unsafe {
            let t0 = std::time::Instant::now();
            test_rust_runif(&mut d);
            if i > 5 { println!("rust_runif      {} {:?}", d.len(), t0.elapsed()); }
        }
        unsafe {
            let t0 = std::time::Instant::now();
            test_par_rust_runif(&mut d);
            if i > 5 { println!("rust_par_runif  {} {:?}", d.len(), t0.elapsed()); }
        }
        unsafe {
            let t0 = std::time::Instant::now();
            test_rnorm(&mut d);
            if i > 5 { println!("rnorm           {} {:?}", d.len(), t0.elapsed()); }
        }
        unsafe {
            let t0 = std::time::Instant::now();
            test_par_rnorm(&mut d);
            if i > 5 { println!("par_rnorm       {} {:?}", d.len(), t0.elapsed()); }
        }
        unsafe {
            let t0 = std::time::Instant::now();
            test_rust_rnorm(&mut d);
            if i > 5 { println!("rust_rnorm      {} {:?}", d.len(), t0.elapsed()); }
        }
        unsafe {
            let t0 = std::time::Instant::now();
            test_par_rust_rnorm(&mut d);
            if i > 5 { println!("par_rust_rnorm  {} {:?}", d.len(), t0.elapsed()); }
        }
    }
}
