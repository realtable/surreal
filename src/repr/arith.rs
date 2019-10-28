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
    let recurse = |v: &Vec<Surreal>, w: &Vec<Surreal>| {
        let mut res: Vec<Surreal> = vec![];
        for i in v.clone() {
            for j in w.clone() {
                let mul_a = mul(&i, y);
                let mul_b = mul(x, &j);
                let mul_c = mul(&i, &j);
                res.push(&(&mul_a + &mul_b) - &mul_c);
            }
        }
        res
    };

    let mut left: Vec<Surreal> = recurse(&x.left, &y.left);
    let mut right: Vec<Surreal> = recurse(&x.left, &y.right);

    left.append(&mut recurse(&x.right, &y.right));
    right.append(&mut recurse(&x.right, &y.left));

    Surreal { left, right }
}

pub fn inv(x: &Surreal) -> Surreal {
    // REALLY CHEATY
    super::super::ftos(1.0 / super::super::stof(x))
}
