use std::cmp::Ordering;
use std::fmt;

mod first; // first half

pub struct Surreal {
    left: Vec<Surreal>,
    right: Vec<Surreal>,
}

impl Surreal {
    pub fn new(left: Vec<&Surreal>, right: Vec<&Surreal>) -> Surreal {
        for xl in &left {
            for xr in &right {
                if first::leq(xr, xl) {
                    panic!("Items in the left set must be less than items in the right set");
                }
            }
        }
        
        Surreal { left: cnv(left), right: cnv(right) }    
    } // rule 1
    
    pub fn left(&self) -> Vec<Surreal> {
        first::cpy(&self.left)
    }
    
    pub fn right(&self) -> Vec<Surreal> {
        first::cpy(&self.right)
    }
}

impl PartialEq for Surreal {
    fn eq(&self, other: &Surreal) -> bool {
        first::bbl(&self.left).last() == first::bbl(&other.left).last() &&
        first::bbl(&self.right).first() == first::bbl(&other.right).first()
    }
} // theorem 8

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
} // theorem 4

impl fmt::Display for Surreal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fmt_vec = |v: &Vec<Surreal>| {
            let mut s = String::new();
            for i in v {
                s.push_str(format!("{}", i).as_str());
            }
            s
        };
        
        write!(f, "({{{}}}, {{{}}})", fmt_vec(&self.left), fmt_vec(&self.right))
    }
}

fn cnv(sur: Vec<&Surreal>) -> Vec<Surreal> {
    sur.into_iter().map(|n| {
        Surreal { left: first::cpy(&n.left), right: first::cpy(&n.right) }
    }).collect()
}
