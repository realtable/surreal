use super::Surreal;

pub fn add(x: &Surreal, y: &Surreal) -> Surreal {
    let recurse = |v: &Vec<Surreal>, n| -> Vec<Surreal> {
        v.clone().into_iter().map(|i| add(&i, n)).collect()
    };

    let mut left = recurse(&x.left, y);
    left.append(&mut recurse(&y.left, x));
    let mut right = recurse(&x.right, y);
    right.append(&mut recurse(&y.right, x));

    Surreal { left, right }
}

pub fn neg(sur: &Surreal) -> Surreal {
    let recurse =
        |v: &Vec<Surreal>| -> Vec<Surreal> { v.clone().into_iter().map(|i| neg(&i)).collect() };

    Surreal {
        left: recurse(&sur.right),
        right: recurse(&sur.left),
    }
}

pub fn mul(x: &Surreal, y: &Surreal) -> Surreal {
    let mut left = vec![];
    let mut right = vec![];

    let recurse = |v: &Vec<Surreal>, w: &mut Vec<Surreal>, n| {
        for i in v {
            w.push(&(&mul(n, y) + &mul(x, &i)) - &mul(&i, n));
        }
    };
        
    for i in &x.left {
        recurse(&y.left, &mut left, i);
        recurse(&y.right, &mut right, i);
    }
    
    for i in &x.right {
        recurse(&y.right, &mut left, i);
        recurse(&y.left, &mut right, i);
    }
    
    Surreal { left, right }
}
