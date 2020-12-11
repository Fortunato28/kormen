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
    let mut result_begin_index = 0;
    let mut result_end_index = 0;

    let middle = full_array.len() / 2;

    // Сначала разберёмся с левой частью подмассива
    let mut left_max_sum = 0;
    let mut left_current_sum = 0;
    // Итерируемся от середины вниз
    for (index, elem) in full_array[..middle].iter().rev().enumerate() {
        left_current_sum += elem;
        if left_current_sum > left_max_sum {
            left_max_sum = left_current_sum;
            result_begin_index = index;
        }
    }
    // Определим реальный индекс начала максимального подмассива
    result_begin_index = full_array[..middle].len() - 1 - result_begin_index;

    // Теперь разберёмся с правой частью подмассива
    let mut right_max_sum = 0;
    let mut right_current_sum = 0;
    // Итерируемся от середины вверх
    for (index, elem) in full_array[middle..].iter().enumerate() {
        right_current_sum += elem;
        if right_current_sum > right_max_sum {
            right_max_sum = right_current_sum;
            result_end_index = index;
        }
    }

    // Определим реальный индекс конца максимального подмассива
    result_end_index = middle + result_end_index;

    MaxSubarray {
        sum: right_max_sum + left_max_sum,
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
mod max_subarray_tests;
