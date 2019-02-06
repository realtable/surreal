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

#[test]
fn test_ops() {
    let v = day_gen(4);
    let w = day_gen(2);

    let neg_one = Surreal {
        left: vec![],
        right: vec![Surreal {
            left: vec![],
            right: vec![],
        }],
    };
    
    let zero = Surreal {
        left: vec![],
        right: vec![],
    };
    
    let one = Surreal {
        left: vec![Surreal {
            left: vec![],
            right: vec![],
        }],
        right: vec![],
    };
    
    for i in 0..v.len() {
        assert!(-&v[i] == v[v.len() - i - 1]);
        assert!(&v[i] + &v[v.len() - i - 1] == zero);
        assert!(&v[i] + &zero == v[i]);
    
        assert!(&v[i] * &neg_one == -&v[i]);
        assert!(&v[i] * &zero == zero);
        assert!(&v[i] * &one == v[i]);
    }

    for i in 0..w.len() {
        for j in 0..w.len() {
            if (i != w.len() / 2) && (j != w.len() / 2) {
                if (i < w.len() / 2) && (j < w.len() / 2) {
                    assert!(&w[i] + &w[j] < zero);
                    assert!(&w[i] * &w[j] > zero);
                } else if (i > w.len() / 2) && (j > w.len() / 2) {
                    assert!(&w[i] + &w[j] > zero);
                    assert!(&w[i] * &w[j] > zero);
                } else {
                    assert!(&w[i] * &w[j] < zero);
                }
            }
        }
    }
}

fn day_gen(days: i32) -> Vec<Surreal> {
    if days == 0 {
        return vec![Surreal {
            left: vec![],
            right: vec![],
        }];
    }

    let v = day_gen(days - 1);
    let mut res = vec![];
    let c = |sur: &Surreal| Surreal {
        left: sur.left.clone(),
        right: sur.right.clone(),
    };

    for i in 0..v.len() {
        res.push(c(&v[i]));
        if i != v.len() - 1 {
            res.push(Surreal {
                left: vec![c(&v[i])],
                right: vec![c(&v[i + 1])],
            });
        }
    }

    res.insert(
        0,
        Surreal {
            left: vec![],
            right: vec![c(&v[0])],
        },
    );

    res.push(Surreal {
        left: vec![c(&v[v.len() - 1])],
        right: vec![],
    });

    res
}
