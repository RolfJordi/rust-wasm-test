use std::fmt;
use std::ops::Add;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    real: f64,
    imaginary: f64,
) -> Result<(), JsValue> {
    // The real workhorse of this algorithm, generating pixel data
    let c = Complex { real, imaginary };
    let data = get_julia_set(width, height, c);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn get_julia_set(width: u32, height: u32, c: Complex) -> Vec<u8> {
    let mut data = Vec::new();

    let param_i = 1.5;
    let param_r = 1.5;
    let scale = 0.005;

    for x in 0..width {
        for y in 0..height {
            let z = Complex {
                real: f64::from(y) * scale - param_r,
                imaginary: f64::from(x) * scale - param_i,
            };
            let iter_index = get_iter_index(z, c);
            #[allow(clippy::cast_possible_truncation)]
            data.push((iter_index / 4) as u8);
            #[allow(clippy::cast_possible_truncation)]
            data.push((iter_index / 2) as u8);
            #[allow(clippy::cast_possible_truncation)]
            data.push(iter_index as u8);
            //alpha
            data.push(255);
        }
    }

    data
}

fn get_iter_index(z: Complex, c: Complex) -> u32 {
    let mut iter_index: u32 = 0;
    let mut z = z;
    while iter_index < 1000 {
        if z.norm() > 2.0 {
            break;
        }
        z = z.square() + c;
        iter_index += 1;
    }
    iter_index
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    real: f64,
    imaginary: f64,
}

// Implement `Display` for `MinMax`.
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.real, self.imaginary)
    }
}

impl Complex {
    fn square(self) -> Complex {
        let real = (self.real * self.real) - (self.imaginary * self.imaginary);
        let imaginary = 2.0 * self.real * self.imaginary;
        Complex { real, imaginary }
    }

    fn norm(&self) -> f64 {
        (self.real * self.real) + (self.imaginary * self.imaginary)
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_add() {
        let c = Complex {
            real: 1.0,
            imaginary: 2.0,
        };
        let e = Complex {
            real: 2.0,
            imaginary: 4.0,
        };

        let r = c + c;
        assert_eq!(r.real, e.real);
        assert_eq!(r.imaginary, e.imaginary);
    }

    #[test]
    fn test_display() {
        let c = Complex {
            real: 1.0,
            imaginary: 2.0,
        };
        assert_eq!(c.to_string(), "(1, 2)")
    }
}
