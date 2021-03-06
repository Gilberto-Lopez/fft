mod complex;
mod pfft;

pub use crate::complex::Complex;

use std::f64::consts::PI;

fn split(p: Vec<Complex>) -> (Vec<Complex>, Vec<Complex>) {
    let l = p.len();
    let m = l / 2;

    let mut even = Vec::with_capacity(m);
    let mut odd = Vec::with_capacity(m);

    let mut k = 0;
    while k < l {
        even.push(p[k]);
        odd.push(p[k + 1]);
        k += 2;
    }

    (even, odd)
}

#[allow(non_snake_case)]
pub fn FFT(p: Vec<Complex>) -> Vec<Complex> {
    let n = p.len();
    if n == 1 {
        return p;
    }

    let (pe, po) = split(p);
    let ye = FFT(pe);
    let yo = FFT(po);

    let half = n / 2;
    let w = Complex::new((-PI / half as f64).cos(), (-PI / half as f64).sin());
    let mut wj = Complex::from(1.);

    let mut y = vec![Complex::from(0.); n];
    for j in 0..half {
        let x = wj * yo[j];
        y[j] = ye[j] + x;
        y[j + half] = ye[j] - x;
        wj = wj * w;
    }

    return y;
}

pub fn PFFT(p: Vec<Complex>, max_threads: usize) -> Vec<Complex> {
    crate::pfft::thread_PFFT(p, max_threads)
}
