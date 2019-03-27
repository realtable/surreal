use super::Surreal;

static TESTS: i32 = 5;
static BROKEN: i32 = 3;

#[test]
fn test_gt_eq_lt() {
    let v = day_gen(TESTS);

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
fn test_add_neg() {
    let v = day_gen(TESTS);

    let zero = Surreal {
        left: vec![],
        right: vec![],
    };

    for i in 0..v.len() {
        assert!(-&v[i] == v[v.len() - i - 1]);
        assert!(&v[i] + &v[v.len() - i - 1] == zero);
        assert!(&v[i] + &zero == v[i]);
    }

    for i in v.clone() {
        for j in v.clone() {
            if i < zero && j < zero {
                assert!(&i + &j < zero);
            } else if i > zero && j > zero {
                assert!(&i + &j > zero);
            }
        }
    }
}

#[test]
fn test_mul() {
    let v = day_gen(BROKEN);

    let zero = Surreal {
        left: vec![],
        right: vec![],
    };

    let one = Surreal {
        left: vec![zero.clone()],
        right: vec![],
    };

    let neg_one = Surreal {
        left: vec![],
        right: vec![zero.clone()],
    };

    for i in v.clone() {
        assert!(&i * &neg_one == -&i);
        assert!(&i * &zero == zero);
        assert!(&i * &one == i);
    }

    for i in v.clone() {
        for j in v.clone() {
            if (i < zero && j < zero) || (i > zero && j > zero) {
                assert!(&i * &j > zero);
            } else if (i < zero && j > zero) || (i > zero && j < zero) {
                assert!(&i * &j < zero);
            } else {
                assert!(&i * &j == zero);
            }
        }
    }
}

fn day_gen(days: i32) -> Vec<Surreal> {
    if days == 1 {
        return vec![Surreal {
            left: vec![],
            right: vec![],
        }];
    }

    let v = day_gen(days - 1);
    let mut w = vec![];

    w.push(Surreal {
        left: vec![],
        right: vec![v[0].clone()],
    });

    for i in 0..v.len() {
        w.push(v[i].clone());
        if i != v.len() - 1 {
            w.push(Surreal {
                left: vec![v[i].clone()],
                right: vec![v[i + 1].clone()],
            });
        }
    }

    w.push(Surreal {
        left: vec![v[v.len() - 1].clone()],
        right: vec![],
    });

    w
}
