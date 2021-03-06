use std::cmp::Ordering;
use std::fmt;
use std::ops;

mod arith;
mod order;

/// A struct to represent surreal numbers with non-infinite sets.
#[derive(Debug, Clone)]
pub struct Surreal {
    left: Vec<Surreal>,
    right: Vec<Surreal>,
}

impl Surreal {
    /// Creates a new surreal number given two vectors of references to surreal numbers. Each
    /// vector corresponds to a left set and a right set, where each number in the left set must
    /// be less than all numbers in the right set.
    ///
    /// # Panics
    ///
    /// Panics if any `Surreal` in `left` is greater than or equal to any `Surreal` in `right`.
    ///
    /// # Examples
    ///
    /// ```
    /// let zero = surreal::Surreal::new(vec![], vec![]);
    /// let one = surreal::Surreal::new(vec![&zero], vec![]);
    /// let neg_one = surreal::Surreal::new(vec![], vec![&zero]);
    /// ```
    pub fn new(left: Vec<&Surreal>, right: Vec<&Surreal>) -> Surreal {
        for xl in &left {
            for xr in &right {
                if order::leq(xr, xl) {
                    panic!("Items in the left set must be less than items in the right set");
                }
            }
        }

        Surreal {
            left: order::cnv(left),
            right: order::cnv(right),
        }
    }

    /// Creates a new pseudo-surreal number given two vectors of references to surreal numbers.
    /// While each vector still corresponds to a left set and a right set, each number in the left
    /// set doesn't have to be less than all numbers in the right set.
    ///
    /// # Examples
    ///
    /// ```
    /// let zero = surreal::Surreal::new(vec![], vec![]);
    /// let pseudo = surreal::Surreal::pseudo(vec![&zero], vec![&zero]);
    /// ```
    pub fn pseudo(left: Vec<&Surreal>, right: Vec<&Surreal>) -> Surreal {
        Surreal {
            left: order::cnv(left),
            right: order::cnv(right),
        }
    }

    /// Returns the left set of a surreal number (as a `Vec<Surreal>` instead of a
    /// `Vec<&Surreal>`).
    ///
    /// # Examples
    ///
    /// ```
    /// let zero = surreal::Surreal::new(vec![], vec![]);
    /// let one = surreal::Surreal::new(vec![&zero], vec![]);
    ///
    /// assert!(zero.left().is_empty());
    /// assert!(one.left()[0] == zero);
    /// ```
    pub fn left(&self) -> Vec<Surreal> {
        self.left.clone()
    }

    /// Returns the right set of a surreal number (as a `Vec<Surreal>` instead of a
    /// `Vec<&Surreal>`).
    ///
    /// # Examples
    ///
    /// ```
    /// let zero = surreal::Surreal::new(vec![], vec![]);
    /// let neg_one = surreal::Surreal::new(vec![], vec![&zero]);
    ///
    /// assert!(zero.right().is_empty());
    /// assert!(neg_one.right()[0] == zero);
    /// ```
    pub fn right(&self) -> Vec<Surreal> {
        self.right.clone()
    }
}

impl PartialEq for Surreal {
    fn eq(&self, other: &Surreal) -> bool {
        order::leq(self, other) && order::leq(other, self)
    }
}

impl Eq for Surreal {}

impl PartialOrd for Surreal {
    fn partial_cmp(&self, other: &Surreal) -> Option<Ordering> {
        if !order::leq(self, other) {
            Some(Ordering::Greater)
        } else if !order::leq(other, self) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl<'a, 'b> ops::Add<&'b Surreal> for &'a Surreal {
    type Output = Surreal;

    fn add(self, other: &'b Surreal) -> Surreal {
        arith::add(self, other)
    }
}

/// **Note**: Currently, division does not work for numbers created on or after day 4, i.e. numbers
/// with 4 or more layers of nested sets.
///
/// # Panics
///
/// Panics if the divisor is zero. Panics if the quotient will have infinite sets.
impl<'a, 'b> ops::Div<&'b Surreal> for &'a Surreal {
    type Output = Surreal;

    fn div(self, other: &'b Surreal) -> Surreal {
        let zero = Surreal::new(vec![], vec![]);

        if other == &zero {
            panic!("Cannot divide by zero");
        } else if (super::stof(self) / super::stof(other)).fract() * 256.0 % 1.0 != 0.0 {
            panic!("Quotient will have infinite sets"); // isn't representable as finite f32
        }

        arith::mul(self, &arith::inv(other))
    }
}

/// **Note**: Currently, multiplication does not work for numbers created on or after day 4, i.e.
/// numbers with 4 or more layers of nested sets.
impl<'a, 'b> ops::Mul<&'b Surreal> for &'a Surreal {
    type Output = Surreal;

    fn mul(self, other: &'b Surreal) -> Surreal {
        arith::mul(self, other)
    }
}

impl<'a> ops::Neg for &'a Surreal {
    type Output = Surreal;

    fn neg(self) -> Surreal {
        arith::neg(self)
    }
}

impl<'a, 'b> ops::Sub<&'b Surreal> for &'a Surreal {
    type Output = Surreal;

    fn sub(self, other: &'b Surreal) -> Surreal {
        arith::add(self, &arith::neg(other))
    }
}

impl fmt::Display for Surreal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self) // alias to debug print
    }
}

/// Tests if a division will produce a result with finite sets. Useful in making sure a division of
/// surreal numbers won't panic.
///
/// # Examples
///
/// ```
/// let zero = surreal::Surreal::new(vec![], vec![]);
/// let one = surreal::Surreal::new(vec![], vec![&zero]);
///
/// if surreal::is_divisible(&zero, &one) {
///     println!("{}", &zero / &one);
/// }
pub fn is_divisible(x: &Surreal, y: &Surreal) -> bool {
    (super::stof(&x) / super::stof(&y)).fract() * 256.0 % 1.0 == 0.0
}

/// Tests if a given input is a pseudo-surreal number (created with `surreal::Surreal::pseudo`).
///
/// # Examples
///
/// ```
/// let zero = surreal::Surreal::new(vec![], vec![]);
/// assert!(!surreal::is_pseudo(&zero));
///
/// let some_pseudo = surreal::Surreal::pseudo(vec![&zero], vec![&zero]);
/// assert!(surreal::is_pseudo(&some_pseudo));
/// ```
pub fn is_pseudo(sur: &Surreal) -> bool {
    let mut res = false;

    for xl in sur.left() {
        for xr in sur.right() {
            if xl >= xr {
                res = true;
            }
        }
    }

    res
}
