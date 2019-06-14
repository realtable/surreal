use super::Surreal;

/// Converts surreal number with non-infinite sets to floating-point number.
///
/// # Panics
///
/// Given that all surreal numbers with non infiniter sets match to a finaite binary representable
/// number, this function covers all possible values of `Surreal` and will not panic
///
/// # Examples
///
/// ```
/// let zero = surreal::Surreal::new(vec![], vec![]);
/// let one = surreal::Surreal::new(vec![&zero], vec![]);
/// 
/// assert!(surreal::stof(&zero) == 0.0);
/// assert!(surreal::stof(&one) == 1.0);
/// ```
pub fn stof(sur: &Surreal) -> f32 {
    let mut left: Vec<f32> = vec![];
    let mut right: Vec<f32> = vec![];
    
    for xl in sur.left() {
        left.push(stof(&xl));
    }
    
    for xr in sur.right() {
        right.push(stof(&xr));
    }
    
    logic(&mut left, &mut right)  // <f|f> or <|f> or <f|> or <|>
}

fn logic(left: &mut Vec<f32>, right: &mut Vec<f32>) -> f32 {
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