// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;

#[test]
fn brute_force_positive() {
    let full_array = vec![
        13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
    ];

    let result = brute_force(&full_array);
    let needed_value = MaxSubarray {
        sum: 43,
        begin_index: 7,
        end_index: 10,
    };

    assert_eq!(result, needed_value);
}

#[test]
fn max_crossing_subarray() {
    let full_array = vec![
        13, -3, -25, 20, -3, 16, -2, 18, 20, -7, 12, -5, -22, 15, -4, 7, -5,
    ];

    let result = find_max_crossing_subarray(&full_array);
    let needed_value = MaxSubarray {
        sum: 74,
        begin_index: 3,
        end_index: 10,
    };

    assert_eq!(result, needed_value);
}

#[test]
fn find_maximum_subarray_cross() {
    let full_array = vec![
        13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
    ];

    let result = find_maximum_subarray(&full_array);
    let needed_value = MaxSubarray {
        sum: 43,
        begin_index: 7,
        end_index: 10,
    };

    assert_eq!(result, needed_value);
}

#[test]
fn find_maximum_subarray_left() {
    let full_array = vec![
        -3, -25, 20, -3, 6, 23, -18, -20, -7, -12, -5, -22, -15, -4, 7,
    ];

    let result = find_maximum_subarray(&full_array);
    let needed_value = MaxSubarray {
        sum: 46,
        begin_index: 2,
        end_index: 5,
    };

    assert_eq!(result, needed_value);
}

#[test]
fn find_maximum_subarray_right() {
    let full_array = vec![23, -18, -20, -7, -12, -5, -22, 15, 40, 7];

    let result = find_maximum_subarray(&full_array);
    let needed_value = MaxSubarray {
        sum: 62,
        begin_index: 7,
        end_index: 9,
    };

    assert_eq!(result, needed_value);
}

#[test]
fn find_maximum_subarray_right_second() {
    let full_array = vec![23, -18, -20, -7, 12, 5, 22, 15, 40, 7];

    let result = find_maximum_subarray(&full_array);
    let needed_value = MaxSubarray {
        sum: 101,
        begin_index: 4,
        end_index: 9,
    };

    assert_eq!(result, needed_value);
}
