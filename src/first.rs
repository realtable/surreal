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
} // rule 2

pub fn bbl(sur: &Vec<Surreal>) -> Vec<Surreal> {
    let mut res = cpy(sur);
    
    for j in 0..res.len() {
        for k in 0..j {
            if res[j] > res[k] {
                res.swap(j, k);
            }
        }
    }
    
    res
} // bubble sort

pub fn cpy(sur: &Vec<Surreal>) -> Vec<Surreal> {
    let mut res = vec![];
    
    for j in sur {
        if j.left.is_empty() && j.right.is_empty() {
            res.push(Surreal { left: vec![], right: vec![] });
        } else {
            res.push(Surreal { left: cpy(&j.left), right: cpy(&j.right) });
        }
    }
    
    res
}
