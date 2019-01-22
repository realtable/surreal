use super::*;

#[test]
fn test_cmp() {
    let v = day_gen(5);

    for i in 0..v.len() {
        for j in 0..v.len() {
            if i < j {
                assert!(v[i] < v[j]);
            } else if i > j {
                assert!(v[i] > v[j]);
            } else {
                assert!(v[i] == v[j]);
            }
        }
    }
}

fn day_gen(days: i32) -> Vec<Surreal> {
    if days == 0 {
        return vec![Surreal { left: vec![], right: vec![] }];
    }

    let v = day_gen(days - 1);
    let mut res = vec![];
    let c = |sur: &Surreal| {
        Surreal { left: first::cpy(&sur.left), right: first::cpy(&sur.right) }
    };
    
    for i in 0..v.len() {
        res.push(c(&v[i]));
        if i != v.len() - 1 {
            res.push(Surreal { left: vec![c(&v[i])], right: vec![c(&v[i + 1])] });
        }
    }

    res.insert(0, Surreal { left: vec![], right: vec![c(&v[0])] });
    res.push(Surreal { left: vec![c(&v[v.len() - 1])], right: vec![] });
    res
}
