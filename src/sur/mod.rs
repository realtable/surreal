use std::cmp::Ordering;
use std::fmt;
use std::ops;

mod first;
mod second;

/// A struct to represent surreal numbers with non-infinite sets.
///
/// **N.B.** Currently, multiplication does not work for numbers on or after day 4.
#[derive(Debug, Clone)]
pub struct Surreal {
    pub left: Vec<Surreal>,
    pub right: Vec<Surreal>,
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
    /// // the lines below will compile
    /// let zero = surreal::Surreal::new(vec![], vec![]);
    /// let one = surreal::Surreal::new(vec![&zero], vec![]);
    /// let neg_one = surreal::Surreal::new(vec![], vec![&zero]);
    ///
    /// // the lines below will panic
    /// // let err = surreal::Surreal::new(vec![&one], vec![&neg_one]);
    /// // let err = surreal::Surreal::new(vec![&zero], vec![&zero]);
    /// ```
    pub fn new(left: Vec<&Surreal>, right: Vec<&Surreal>) -> Surreal {
        for xl in &left {
            for xr in &right {
                if first::leq(xr, xl) {
                    panic!("Items in the left set must be less than items in the right set");
                }
            }
        }

        Surreal {
            left: first::cnv(left),
            right: first::cnv(right),
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
        first::leq(self, other) && first::leq(other, self)
    }
}

impl Eq for Surreal {}

impl PartialOrd for Surreal {
    fn partial_cmp(&self, other: &Surreal) -> Option<Ordering> {
        if !first::leq(self, other) {
            Some(Ordering::Greater)
        } else if !first::leq(other, self) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl<'a, 'b> ops::Add<&'b Surreal> for &'a Surreal {
    type Output = Surreal;

    fn add(self, other: &'b Surreal) -> Surreal {
        second::add(self, other)
    }
}

impl<'a, 'b> ops::Mul<&'b Surreal> for &'a Surreal {
    type Output = Surreal;

    fn mul(self, other: &'b Surreal) -> Surreal {
        second::mul(self, other)
    }
}

impl<'a> ops::Neg for &'a Surreal {
    type Output = Surreal;

    fn neg(self) -> Surreal {
        second::neg(self)
    }
}

impl<'a, 'b> ops::Sub<&'b Surreal> for &'a Surreal {
    type Output = Surreal;

    fn sub(self, other: &'b Surreal) -> Surreal {
        second::add(self, &second::neg(other))
    }
}

impl fmt::Display for Surreal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fmt_vec = |v: &Vec<Surreal>| {
            let mut s = String::new();
            for i in v {
                s.push_str(format!("{}", i).as_str());
            }
            s
        };

        write!(
            f,
            "Surreal ({{{}}}, {{{}}})",
            fmt_vec(&self.left),
            fmt_vec(&self.right)
        )
    }
}
