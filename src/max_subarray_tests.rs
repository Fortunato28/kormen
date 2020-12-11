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
#[ignore]
fn divide_and_rule_positive() {
    let full_array = vec![
        13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
    ];

    let result = devide_and_rule(&full_array);
    let needed_value = MaxSubarray {
        sum: 43,
        begin_index: 7,
        end_index: 10,
    };

    assert_eq!(result, needed_value);
}
