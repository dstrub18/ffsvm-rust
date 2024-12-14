use std::{cmp::PartialOrd, marker::Copy};

/// Sets all items of a mutable vector to the given value.
pub fn set_all<T>(vector: &mut [T], value: T)
where
    T: Copy,
{
    for item in vector.iter_mut() {
        *item = value;
    }
}

/// Finds the item with the maximum index.
pub fn find_max_index<T>(array: &[T]) -> usize
where
    T: PartialOrd,
{
    let mut vote_max_idx = 0;

    for i in 1..array.len() {
        if array[i] > array[vote_max_idx] {
            vote_max_idx = i;
        }
    }

    vote_max_idx
}

/// As implemented in `libsvm`.
pub fn sigmoid_predict(decision_value: f64, a: f64, b: f64) -> f64 {
    let fapb = decision_value.mul_add(a, b);

    // Citing from the original libSVM implementation:
    // "1-p used later; avoid catastrophic cancellation"
    if fapb >= 0_f64 {
        (-fapb).exp() / (1_f64 + (-fapb).exp())
    } else {
        1_f64 / (1_f64 + fapb.exp())
    }
}

/// As implemented in `libsvm`.
pub fn powi(base: f64, times: u32) -> f64 {
    let mut tmp = base;
    let mut ret = 1.0;
    let mut t = times;

    while t > 0 {
        if t % 2 == 1 {
            ret *= tmp;
        };

        tmp = tmp * tmp;
        t /= 2;
    }

    ret
}
