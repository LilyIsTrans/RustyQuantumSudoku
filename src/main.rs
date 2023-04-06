use ::num::complex::Complex;

const MAX_ITERATIONS: u64 = 200;

fn mandelbrot_iteration(z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    z * z + c
}

fn mandelbrot_iteration_count(c: Complex<f64>) -> u64 {
    let mut iterations: u64 = 0;
    let mut z: Complex<f64> = Complex { re: (0f64), im: (0f64) };
    while z.norm_sqr() < 4.0 && iterations < MAX_ITERATIONS {
        z = mandelbrot_iteration(z, c);
        iterations += 1;
    }
    iterations
}

fn main() {
    println!("Hello, world!");
}
