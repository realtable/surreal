use super::Surreal;

pub fn add(x: &Surreal, y: &Surreal) -> Surreal {
    let recurse = |v: &Vec<Surreal>, n| -> Vec<Surreal> {
        v.clone().into_iter().map(|i| add(&i, n)).collect()
    };

    Surreal {
        left: app(recurse(&x.left, y), recurse(&y.left, x)),
        right: app(recurse(&x.right, y), recurse(&y.right, x)),
    }
}

pub fn neg(sur: &Surreal) -> Surreal {
    let recurse =
        |v: &Vec<Surreal>| -> Vec<Surreal> { v.clone().into_iter().map(|i| neg(&i)).collect() };

    Surreal {
        left: recurse(&sur.right),
        right: recurse(&sur.left),
    }
}

fn app(x: Vec<Surreal>, y: Vec<Surreal>) -> Vec<Surreal> {
    let mut res = x.clone();

    for i in y {
        res.push(i);
    }

    res
}
