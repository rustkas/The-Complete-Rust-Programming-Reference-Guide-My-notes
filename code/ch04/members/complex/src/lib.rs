#![allow(dead_code)]
use std::fmt::{Display, Formatter, Result};
use std::ops::Add;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
struct Complex<T> {
    /// Real part
    re: T,
    /// Complex part
    im: T,
}

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

impl<T: Add<T, Output = T>> Add for Complex<T> {
    type Output = Complex<T>;
    fn add(self, rhs: Complex<T>) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> From<(T, T)> for Complex<T> {
    fn from(value: (T, T)) -> Complex<T> {
        Complex {
            re: value.0,
            im: value.1,
        }
    }
}

impl<T: Display> Display for Complex<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} + {}i", self.re, self.im)
    }
}

#[cfg(test)]
mod tests {
    use crate::Complex;
    #[test]
    fn complex_basics() {
        let first = Complex::new(3, 5);
        let second: Complex<i32> = Complex::default();
        assert_eq!(first.re, 3);
        assert_eq!(first.im, 5);
        assert!(second.re == second.im);
    }
    #[test]
    fn complex_addition() {
        let a = Complex::new(1, -2);
        let b = Complex::default();
        let res = a + b;
        assert_eq!(a, res);
    }

    #[test]
    fn complex_from() {
        let re = 2345;
        let im = 456;
        let a = (re, im);
        let complex = Complex::from(a);
        assert_eq!(re, complex.re);
        assert_eq!(im, complex.im);
    }

    #[test]
    fn complex_debug() {
        let my_imaginary = Complex::new(2345, 456);
        println!("{my_imaginary:?}");
    }
    #[test]
    fn complex_display() {
        let my_imaginary = Complex::new(2345, 456);
        println!("{my_imaginary}");
    }
}
