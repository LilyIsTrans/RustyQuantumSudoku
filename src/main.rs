use ::num::complex::Complex;

enum EscapeTime {
    Iterations(usize),
    Never
}

fn mandelbrot_iteration(z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    z * z + c
}

fn mandelbrot_iteration_count(c: Complex<f64>) -> EscapeTime {
    const MAX_ITERATIONS: usize = 200;
    let mut iterations: usize = 0;
    let mut z: Complex<f64> = Complex { re: (0f64), im: (0f64) };
    while z.norm_sqr() < 4.0 && iterations < MAX_ITERATIONS {
        z = mandelbrot_iteration(z, c);
        iterations += 1;
    }
    match iterations {
        
    }
}



fn main() {
    println!("Hello, world!");
}
