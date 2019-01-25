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
    let zero = Surreal {
        left: vec![],
        right: vec![],
    };

    for i in 0..v.len() {
        assert!(-&v[i] == v[v.len() - i - 1]);
        assert!(&v[i] + &v[v.len() - i - 1] == zero);
        assert!(&v[i] + &zero == v[i]);

        for j in 0..v.len() {
            if (i < v.len() / 2) && (j < v.len() / 2) {
                assert!(&v[i] + &v[j] < zero);
            } else if (i > v.len() / 2) && (j > v.len() / 2) {
                assert!(&v[i] + &v[j] > zero);
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
