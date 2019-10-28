use super::Surreal;

pub fn logic(left: &mut Vec<f32>, right: &mut Vec<f32>) -> f32 {
    let res: f32;

    if left.is_empty() {
        if right.is_empty() {
            res = 0.0;
        } else {
            right.sort_by(|a, b| a.partial_cmp(b).unwrap());
            res = right[0] - 1.0;
        }
    } else if right.is_empty() {
        left.sort_by(|a, b| a.partial_cmp(b).unwrap());
        res = left.pop().unwrap() + 1.0;
    } else {
        left.sort_by(|a, b| a.partial_cmp(b).unwrap());
        right.sort_by(|a, b| a.partial_cmp(b).unwrap());
        res = (left.pop().unwrap() + right[0]) / 2.0
    }

    res
}

pub fn is_pseudo(sur: &Surreal) -> bool {
    let mut res = false;

    for xl in sur.left() {
        for xr in sur.right() {
            if xr < xl {
                res = true;
            }
        }
    }

    res
}
