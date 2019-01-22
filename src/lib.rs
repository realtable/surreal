//! Implementation of J. H. Conway's surreal numbers, as explained in the book
//! *[Surreal Numbers](https://www.amazon.com/dp/0201038129)* by Donald Knuth. This crate provides
//! an interface to the rules and theorems in the book, as well as a comprehensive surreal type.

use std::cmp::Ordering;
use std::fmt;

mod first;

/// A struct to represent surreal numbers with non-infinite sets.
#[derive(Debug)]
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
        
        Surreal { left: first::cnv(left), right: first::cnv(right) }    
    }
    
    /// Returns the left set of a surreal number (as a `Vec<Surreal>` instead of a
    /// `Vec<&Surreal>`). 
    ///
    /// # Examples
    ///
    /// ```
    /// # let zero = surreal::Surreal::new(vec![], vec![]);
    /// let one = surreal::Surreal::new(vec![&zero], vec![]);
    ///
    /// assert!(one.left()[0] == zero);
    /// ```
    pub fn left(&self) -> Vec<Surreal> {
        first::cpy(&self.left)
    }
    
    /// Returns the right set of a surreal number (as a `Vec<Surreal>` instead of a
    /// `Vec<&Surreal>`). 
    ///
    /// # Examples
    ///
    /// ```
    /// # let zero = surreal::Surreal::new(vec![], vec![]);
    /// let neg_one = surreal::Surreal::new(vec![], vec![&zero]);
    ///
    /// assert!(neg_one.right()[0] == zero);
    /// ```
    pub fn right(&self) -> Vec<Surreal> {
        first::cpy(&self.right)
    }
}

impl PartialEq for Surreal {
    fn eq(&self, other: &Surreal) -> bool {
        first::bbl(&self.left).last() == first::bbl(&other.left).last() &&
        first::bbl(&self.right).first() == first::bbl(&other.right).first()
    }
}

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

impl fmt::Display for Surreal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fmt_vec = |v: &Vec<Surreal>| {
            let mut s = String::new();
            for i in v {
                s.push_str(format!("{}", i).as_str());
            }
            s
        };
        
        write!(f, "Surreal ({{{}}}, {{{}}})", fmt_vec(&self.left), fmt_vec(&self.right))
    }
}

#[cfg(test)]
mod tests;
