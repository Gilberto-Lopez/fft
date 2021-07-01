use crate::Complex;

use std::{f64::consts::PI, thread};

#[allow(non_snake_case)]
pub fn thread_PFFT(p: Vec<Complex>, n_threads: usize) -> Vec<Complex> {
    let n = p.len();
    if n == 1 {
        return p;
    }

    if n_threads == 1 {
        crate::FFT(p)
    } else {
        let (pe, po) = crate::split(p);

        let new_thread = thread::spawn(move || thread_PFFT(po, n_threads / 2));

        let ye = thread_PFFT(pe, n_threads / 2);
        let yo = new_thread.join().unwrap();

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

        y
    }
}
