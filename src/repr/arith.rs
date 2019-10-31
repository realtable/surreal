use super::Surreal;

pub fn add(x: &Surreal, y: &Surreal) -> Surreal {
    let recurse = |v: &Vec<Surreal>, n| {
        let mut res: Vec<Surreal> = vec![];
        for i in v.clone() {
            res.push(add(&i, n));
        }
        res
    };

    let mut left: Vec<Surreal> = recurse(&x.left, y);
    let mut right: Vec<Surreal> = recurse(&x.right, y);

    left.append(&mut recurse(&y.left, x));
    right.append(&mut recurse(&y.right, x));

    Surreal { left, right }
}

pub fn neg(sur: &Surreal) -> Surreal {
    let recurse = |v: &Vec<Surreal>| {
        let mut res: Vec<Surreal> = vec![];
        for i in v.clone() {
            res.push(neg(&i));
        }
        res
    };

    let left: Vec<Surreal> = recurse(&sur.right);
    let right: Vec<Surreal> = recurse(&sur.left);

    Surreal { left, right }
}

pub fn mul(x: &Surreal, y: &Surreal) -> Surreal {
    super::super::ftos(super::super::stof(x) * super::super::stof(y))
}

pub fn inv(x: &Surreal) -> Surreal {
    super::super::ftos(1.0 / super::super::stof(x))
}
