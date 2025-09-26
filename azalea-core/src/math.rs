use std::{
    f64::consts::PI,
    ops::{Add, Div, Sub},
    sync::LazyLock,
};

pub const EPSILON: f64 = 1.0e-7;

pub static SIN: LazyLock<[f32; 65536]> =
    LazyLock::new(|| std::array::from_fn(|i| f64::sin((i as f64) * PI * 2. / 65536.) as f32));

/// A sine function that uses a lookup table.
pub fn sin(x: f32) -> f32 {
    let x = x * 10430.378;
    let x = x as i32 as usize & 0xFFFF;
    SIN[x]
}

/// A cosine function that uses a lookup table.
pub fn cos(x: f32) -> f32 {
    let x = x * 10430.378 + 16384.;
    let x = x as i32 as usize & 0xFFFF;
    SIN[x]
}

pub fn binary_search<
    T: Ord + PartialOrd + Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<u8> + Copy,
>(
    mut min: T,
    max: T,
    predicate: impl Fn(T) -> bool,
) -> T {
    let mut diff = max - min;
    while diff > T::from(0) {
        let diff_mid = diff / T::from(2);
        let mid = min + diff_mid;
        if predicate(mid) {
            diff = diff_mid;
        } else {
            min = mid + T::from(1);
            diff = diff - (diff_mid + T::from(1));
        }
    }

    min
}

pub fn lcm(a: u32, b: u32) -> u64 {
    let gcd = gcd(a, b);
    (a as u64) * (b / gcd) as u64
}
pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn lerp<T: num_traits::Float>(amount: T, a: T, b: T) -> T {
    a + amount * (b - a)
}

pub fn ceil_log2(x: u32) -> u32 {
    u32::BITS - x.saturating_sub(1).leading_zeros()
}

pub fn fract(x: f64) -> f64 {
    let x_int = x as i64 as f64;
    let floor = if x < x_int { x_int - 1. } else { x_int };
    x - floor
}

// these are copied from the java standard library, we don't calculate the
// consts ourself to make sure it's the same as java
pub fn to_radians(degrees: f64) -> f64 {
    degrees * 0.017453292519943295
}
pub fn to_degrees(radians: f64) -> f64 {
    radians * 57.29577951308232
}

/// Returns either -1, 0, or 1, depending on whether the number is negative,
/// zero, or positive.
///
/// This function exists because f64::signum doesn't check for 0.
pub fn sign(num: f64) -> f64 {
    if num == 0. { 0. } else { num.signum() }
}
pub fn sign_as_int(num: f64) -> i32 {
    if num == 0. { 0 } else { num.signum() as i32 }
}

pub fn ceil_long(x: f64) -> i64 {
    let x_i64 = x as i64;
    if x > x_i64 as f64 { x_i64 + 1 } else { x_i64 }
}

pub fn equal(a: f64, b: f64) -> bool {
    (b - a).abs() < 1.0e-5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(1, 1), 1);

        assert_eq!(gcd(0, 1), 1);
        assert_eq!(gcd(1, 0), 1);

        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(8, 12), 4);

        assert_eq!(gcd(12, 9), 3);
        assert_eq!(gcd(9, 12), 3);

        assert_eq!(gcd(12, 7), 1);
        assert_eq!(gcd(7, 12), 1);
    }

    #[test]
    fn test_sin() {
        const PI: f32 = std::f32::consts::PI;
        // check that they're close enough
        fn assert_sin_eq_enough(number: f32) {
            let a = sin(number);
            let b = f32::sin(number);
            assert!((a - b).abs() < 0.01, "sin({number}) failed, {a} != {b}");
        }
        assert_sin_eq_enough(0.0);
        assert_sin_eq_enough(PI / 2.0);
        assert_sin_eq_enough(PI);
        assert_sin_eq_enough(PI * 2.0);
        assert_sin_eq_enough(PI * 3.0 / 2.0);
        assert_sin_eq_enough(-PI / 2.0);
        assert_sin_eq_enough(-PI);
        assert_sin_eq_enough(-PI * 2.0);
        assert_sin_eq_enough(-PI * 3.0 / 2.0);
    }
}
