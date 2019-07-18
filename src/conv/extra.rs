pub fn logic(left: &mut Vec<f32>, right: &mut Vec<f32>) -> f32 {
    let res: f32;

    if left.len() == 0 {
        if right.len() == 0 {
            res = 0.0;
        } else {
            right.sort_by(|a, b| a.partial_cmp(b).unwrap());
            res = right[0] - 1.0;
        }
    } else {
        if right.len() == 0 {
            left.sort_by(|a, b| a.partial_cmp(b).unwrap());
            res = left.pop().unwrap() + 1.0;
        } else {
            left.sort_by(|a, b| a.partial_cmp(b).unwrap());
            right.sort_by(|a, b| a.partial_cmp(b).unwrap());
            res = (left.pop().unwrap() + right[0]) / 2.0
        }
    }

    res
}
