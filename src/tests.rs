use super::ftos;
use super::stof;
use super::Surreal;

static FAST: i32 = 8;
static SLOW: i32 = 5;
static BROKEN: i32 = 3;

#[test]
fn repr_cmp() {
    let v = day_gen(FAST);

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
fn repr_add() {
    let v = day_gen(SLOW);
    let zero = Surreal::new(vec![], vec![]);

    for i in v.clone() {
        for j in v.clone() {
            let sum = &i + &j;

            if i < zero {
                if j < zero {
                    assert!(i > sum);
                    assert!(j > sum);
                    assert!(sum < zero);
                } else if j > zero {
                    if -&i > j {
                        assert!(sum < zero);
                    } else if -&i < j {
                        assert!(sum > zero);
                    } else {
                        assert!(sum == zero);
                    }
                } else {
                    assert!(i == sum)
                }
            } else if i > zero {
                if j < zero {
                    if i > -&j {
                        assert!(sum > zero);
                    } else if i < -&j {
                        assert!(sum < zero);
                    } else {
                        assert!(sum == zero);
                    }
                } else if j > zero {
                    assert!(i < sum);
                    assert!(j < sum);
                    assert!(sum > zero);
                }
            } else {
                assert!(j == sum)
            }
        }
    }
}

#[test]
fn repr_neg() {
    let v = day_gen(FAST);

    for i in 0..v.len() {
        assert!(-&v[i] == v[v.len() - i - 1]);
    }
}

#[test]
fn conv_stof() {
    let v = day_gen(FAST);
    let w = xtra_gen(FAST);

    for i in 0..v.len() {
        assert!(stof(&v[i]) == w[i]);
    }
}

// tests for functions that only pass up to day 3 because they rely on multiplication

#[test]
fn repr_mul() {
    let v = day_gen(BROKEN);
    let zero = Surreal::new(vec![], vec![]);
    let one = Surreal::new(vec![&zero], vec![]);
    let neg_one = Surreal::new(vec![], vec![&zero]);

    for i in v.clone() {
        assert!(&i * &zero == zero);
        assert!(&i * &one == i);
        assert!(&i * &neg_one == -&i);

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

#[test]
fn conv_ftos() {
    let v = day_gen(3);
    let w = xtra_gen(3);

    for i in 0..v.len() {
        assert!(ftos(w[i]) == v[i]);
    }
}

#[test]
fn repr_div() {
    let v = day_gen(BROKEN);
    let w = xtra_gen(BROKEN);
    let zero = Surreal::new(vec![], vec![]);
    let one = Surreal::new(vec![&zero], vec![]);
    let neg_one = Surreal::new(vec![], vec![&zero]);

    for i in 0..v.len() {
        assert!(&v[i] / &neg_one == -&v[i]);
        assert!(&v[i] / &one == v[i]);
        if v[i] != zero {
            assert!(&zero / &v[i] == zero);
            assert!(&v[i] / &v[i] == one);
        }

        for j in 0..v.len() {
            if w[j] != 0.0 && ((w[i] / w[j]).fract() * 256.0 % 1.0 == 0.0) {
                if (w[i] < 0.0 && w[j] < 0.0) || (w[i] > 0.0 && w[j] > 0.0) {
                    assert!(&v[i] / &v[j] > zero);
                } else if (w[i] < 0.0 && w[j] > 0.0) || (w[i] > 0.0 && w[j] < 0.0) {
                    assert!(&v[i] / &v[j] < zero);
                }
            }
        }
    }
}

// functions to generate test values

fn day_gen(days: i32) -> Vec<Surreal> {
    if days == 1 {
        return vec![Surreal::new(vec![], vec![])];
    }

    let v = day_gen(days - 1);
    let mut w = vec![];
    w.push(Surreal::new(vec![], vec![&v[0]]));

    for i in 0..v.len() {
        w.push(v[i].clone());
        if i != v.len() - 1 {
            w.push(Surreal::new(vec![&v[i]], vec![&v[i + 1]]));
        }
    }

    w.push(Surreal::new(vec![&v[v.len() - 1]], vec![]));
    w
}

fn xtra_gen(days: i32) -> Vec<f32> {
    if days == 1 {
        return vec![0.0];
    }

    let v = xtra_gen(days - 1);
    let mut w = vec![];
    w.push(v[0] - 1.0);

    for i in 0..v.len() {
        w.push(v[i]);
        if i != v.len() - 1 {
            w.push((v[i] + &v[i + 1]) / 2.0);
        }
    }

    w.push(v[v.len() - 1] + 1.0);
    w
}
