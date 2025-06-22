use crate::constants::MAX_ITERATIONS;
use crate::context::ComplexNb;

pub fn compute_mandelbrot(point: ComplexNb) -> u32 {
    let mut z = ComplexNb { real: 0., imag: 0. };
    let mut real_squared = 0.;
    let mut imag_squared = 0.;
    for i in 0..MAX_ITERATIONS {
        z.imag = (2. * z.real * z.imag) + point.imag;
        z.real = real_squared - imag_squared + point.real;
        real_squared = z.real * z.real;
        imag_squared = z.imag * z.imag;
        if real_squared + imag_squared > 4. {
            return i;
        }
    }
    MAX_ITERATIONS + 1
}
