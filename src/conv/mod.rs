use super::Surreal;
use std::f32::EPSILON;

mod extra;

/// Converts a surreal number (with non-infinite sets) into a floating-point number.
///
/// # Panics
///
/// Panics if given a psuedo-surreal number.
///
/// # Examples
///
/// ```
/// let zero = surreal::Surreal::new(vec![], vec![]);
/// let one = surreal::Surreal::new(vec![&zero], vec![]);
///
/// assert!(surreal::stof(&zero) == 0.0);
/// assert!(surreal::stof(&one) == 1.0);
/// ```
pub fn stof(sur: &Surreal) -> f32 {
    if extra::is_pseudo(sur) {
        panic!("Cannot convert pseudo-surreal numbers to real numbers");
    }

    let mut left: Vec<f32> = vec![];
    let mut right: Vec<f32> = vec![];

    for xl in sur.left() {
        left.push(stof(&xl));
    }

    for xr in sur.right() {
        right.push(stof(&xr));
    }

    extra::logic(&mut left, &mut right) // <f|f> or <|f> or <f|> or <|>
}

/// Converts a floating-point number into a surreal number.
///
/// # Panics
///
/// Given that all floating point numbers, i.e. all finite binary representable numbers, map to a
/// surreal number with non-infinite sets, theoretically there is no case where this function will
/// panic. However, because this function uses the currently broken multiplication feature,
/// `ftos()` will not work for numbers created on or after day 4, i.e. those with 4 or more layers
/// of nested surreal numbers.
///
/// # Examples
///
/// ```
/// let zero = surreal::Surreal::new(vec![], vec![]);
/// let one = surreal::Surreal::new(vec![&zero], vec![]);
///
/// assert!(surreal::ftos(0.0) == zero);
/// assert!(surreal::ftos(1.0) == one);
/// ```
pub fn ftos(fl: f32) -> Surreal {
    let zero = Surreal::new(vec![], vec![]);
    let one = Surreal::new(vec![&zero], vec![]);
    let neg_one = Surreal::new(vec![], vec![&zero]);
    let half = Surreal::new(vec![&zero], vec![&one]);

    let mut diff: Surreal;
    let mut curr = zero.clone();
    let mut min = zero.clone();
    if fl > 0.0 {
        diff = one.clone();
    } else {
        diff = neg_one.clone();
    }

    while (fl - stof(&curr)).abs() > EPSILON {
        // check within some error
        curr = min.clone();
        while fl.abs() > stof(&curr).abs() {
            min = curr.clone();
            curr = &curr + &diff;
        }
        diff = &diff * &half;
    }

    curr
}
