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

fn find_max_crossing_subarray(full_array: &[i32]) -> MaxSubarray {
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

fn find_maximum_subarray(full_array: &[i32]) -> MaxSubarray {
    if full_array.len() == 1 {
        return MaxSubarray {
            sum: full_array[0],
            begin_index: 0,
            end_index: 0,
        };
    }

    let middle = full_array.len() / 2;
    // Определим максимальный подмассив слева от середины
    let left_max_subarray = find_maximum_subarray(&full_array[0..middle]);

    // Определим максимальный подмассив справа от середины
    let mut right_max_subarray = find_maximum_subarray(&full_array[middle..]);
    // При правом массиве индексы сбиваются, поэтому поправим
    right_max_subarray.begin_index += middle + 1;
    right_max_subarray.end_index += middle;

    // Определим максимальный подмассив пересекающий середину
    let cross_max_subarray = find_max_crossing_subarray(&full_array);

    if left_max_subarray.sum > right_max_subarray.sum
        && left_max_subarray.sum > cross_max_subarray.sum
    {
        return left_max_subarray;
    }
    if right_max_subarray.sum > cross_max_subarray.sum {
        return right_max_subarray;
    }

    // Значит самый большой подмассив пересекает середину
    return cross_max_subarray;
}

#[cfg(test)]
mod max_subarray_tests;
