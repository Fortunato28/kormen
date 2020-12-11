use std::fmt;

#[derive(PartialEq, Debug)]
struct MaxSubarray {
    sum: i32,
    begin_index: usize,
    end_index: usize,
}

impl fmt::Display for MaxSubarray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Sum = {},\nBegin index = {},\nEnd index = {}",
            self.sum, self.begin_index, self.end_index
        )
    }
}

fn main() {
    let full_array = vec![
        13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
    ];

    let max_subarray = brute_force(&full_array);

    println!("{}", max_subarray);

    println!("Hello, world!");
}

fn brute_force(full_array: &Vec<i32>) -> MaxSubarray {
    let mut result_sum = 0;
    let mut result_begin_index = 0;
    let mut result_end_index = 0;

    for i in 0..full_array.len() {
        let mut current_sum = 0;
        for j in i..full_array.len() {
            current_sum += full_array[j];

            if current_sum > result_sum {
                result_sum = current_sum;
                result_begin_index = i;
                result_end_index = j;
            }
        }
    }

    MaxSubarray {
        sum: result_sum,
        begin_index: result_begin_index,
        end_index: result_end_index,
    }
}

fn find_max_crossing_subarray(full_array: &Vec<i32>) -> MaxSubarray {
    let mut result_sum = 0;
    let mut result_begin_index = 0;
    let mut result_end_index = 0;

    MaxSubarray {
        sum: result_sum,
        begin_index: result_begin_index,
        end_index: result_end_index,
    }
}

fn devide_and_rule(full_array: &Vec<i32>) -> MaxSubarray {
    let mut result_sum = 0;
    let mut result_begin_index = 0;
    let mut result_end_index = 0;

    MaxSubarray {
        sum: result_sum,
        begin_index: result_begin_index,
        end_index: result_end_index,
    }
}

#[cfg(test)]
mod tests {
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
            13, -3, -25, 20, -3, 16, -2, 18, 20, -7, 12, -5, -22, 15, -4, 7,
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
}
