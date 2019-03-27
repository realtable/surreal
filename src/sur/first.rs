use super::Surreal;

pub fn leq(x: &Surreal, y: &Surreal) -> bool {
    for item in x.left.iter() {
        if leq(&y, &item) {
            return false;
        }
    }

    for item2 in y.right.iter() {
        if leq(item2, &x) {
            return false;
        }
    }

    true
}

pub fn cnv(sur: Vec<&Surreal>) -> Vec<Surreal> {
    sur.into_iter()
        .map(|n| Surreal {
            left: n.left.clone(),
            right: n.right.clone(),
        })
        .collect()
}
