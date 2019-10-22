use super::Surreal;

pub fn add(x: &Surreal, y: &Surreal) -> Surreal {
    let recurse = |v: &Vec<Surreal>, n| v.clone().iter().map(|i| i + n).collect();

    let mut left: Vec<Surreal> = recurse(&x.left, y);
    let mut right: Vec<Surreal> = recurse(&x.right, y);

    left.append(&mut recurse(&y.left, x));
    right.append(&mut recurse(&y.right, x));

    Surreal { left, right }
}

pub fn neg(sur: &Surreal) -> Surreal {
    let recurse = |v: &Vec<Surreal>| v.clone().iter().map(|i| -i).collect();

    let left: Vec<Surreal> = recurse(&sur.right);
    let right: Vec<Surreal> = recurse(&sur.left);

    Surreal { left, right }
}

pub fn mul(x: &Surreal, y: &Surreal) -> Surreal {
    let zero = Surreal::new(vec![], vec![]);
    let one = Surreal::new(vec![&zero], vec![]);
    let neg_one = Surreal::new(vec![], vec![&zero]);

    if x == &one {
        return y.clone();
    } else if x == &neg_one {
        return -y;
    } else if x == &zero || y == &zero {
        return zero;
    }

    let recurse = |v: &Vec<Surreal>, w: &Vec<Surreal>| {
        v.iter()
            .flat_map(|i: &Surreal| -> Vec<Surreal> {
                w.iter()
                    .map(|j: &Surreal| -> Surreal { &(&(i * y) + &(x * j)) - &(i * j) })
                    .collect()
            })
            .collect()
    };

    let mut left: Vec<Surreal> = recurse(&x.left, &y.left);
    let mut right: Vec<Surreal> = recurse(&x.left, &y.right);

    left.append(&mut recurse(&x.right, &y.right));
    right.append(&mut recurse(&x.right, &y.left));

    Surreal { left, right }
}

pub fn inv(x: &Surreal) -> Surreal {
    let zero = Surreal::new(vec![], vec![]);
    let one = Surreal::new(vec![&zero], vec![]);
    let neg_one = Surreal::new(vec![], vec![&zero]);

    // break from recursion
    if x == &one {
        return one;
    } else if x == &neg_one {
        return neg_one;
    }

    let recurse = |v: &Vec<Surreal>, w: &Vec<Surreal>| {
        v.iter()
            .flat_map(|i: &Surreal| -> Vec<Surreal> {
                w.iter()
                    .map(|j: &Surreal| -> Surreal { &(&one + &(&(i - x) * j)) / i })
                    .collect()
            })
            .collect()
    };

    let mut left: Vec<Surreal> = recurse(&x.right, &inv(x).left);
    let mut right: Vec<Surreal> = recurse(&x.left, &inv(x).left);

    left.push(zero);
    left.append(&mut recurse(&x.left, &inv(x).right));
    right.append(&mut recurse(&x.right, &inv(x).right));

    Surreal { left, right }
}
