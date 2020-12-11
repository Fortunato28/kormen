fn main() {
    println!("Hello, world!");

    //let nums = vec![7, 13, 9, 10];
    //let nums = vec![2, 8, 1, 5, 4, 6, 3, 7];
    let nums = vec![25, 726, 31, 41, 88, 14, 555, 59, 12, 26, 3, 41, 58, 1, -12];

    //insertion_sort(nums);
    //reverse_insertion_sort(nums);
    //selection_sort(nums);
    //merge_sort(&nums);
    let mut total_inversion_amount = 0;
    get_inversions_amount(&nums, &mut total_inversion_amount);
}

#[allow(dead_code)]
fn get_inversions_amount(nums: &[i32], mut total_inversion_amount: &mut usize) -> Vec<i32> {
    let begin = 0;
    let end = nums.len();

    let mut result = vec![];

    if end == 1 {
        result.push(nums[0]);
        return result;
    }

    if begin < end {
        let middle = end / 2;
        let first_half = get_inversions_amount(&nums[begin..middle], &mut total_inversion_amount);
        let second_half = get_inversions_amount(&nums[middle..end], &mut total_inversion_amount);
        result = calculate_inversion_amount(&first_half, &second_half, &mut total_inversion_amount);
    }

    dbg!(&total_inversion_amount);
    result
}

#[allow(dead_code)]
fn calculate_inversion_amount(
    first: &[i32],
    second: &[i32],
    total_inversion_amount: &mut usize,
) -> Vec<i32> {
    let mut merged = vec![];

    //dbg!(&first);
    //dbg!(&second);

    // Determine what array is longer
    let first_len = first.len();
    let second_len = second.len();
    let max_length = first_len + second_len;

    let mut first_index = 0;
    let mut second_index = 0;
    for _i in 0..max_length {
        // No more elements in one of arrays
        if first_index == first_len || second_index == second_len {
            break;
        }

        if first[first_index] <= second[second_index] {
            merged.push(first[first_index]);
            first_index += 1;
        } else {
            merged.push(second[second_index]);
            second_index += 1;
            *total_inversion_amount += first.len() - first_index;
        }
    }

    for i in first_index..first.len() {
        merged.push(first[i]);
    }

    for i in second_index..second.len() {
        merged.push(second[i]);
    }

    dbg!(&merged);
    merged
}

#[allow(dead_code)]
fn merge_sort(nums: &[i32]) -> Vec<i32> {
    let begin = 0;
    let end = nums.len();

    let mut result = vec![];

    if end == 1 {
        result.push(nums[0]);
        return result;
    }

    if begin < end {
        let middle = end / 2;
        let first_half = merge_sort(&nums[begin..middle]);
        let second_half = merge_sort(&nums[middle..end]);
        result = merge_vecs(&first_half, &second_half);
    }
    result
}

#[allow(dead_code)]
fn merge_vecs(first: &[i32], second: &[i32]) -> Vec<i32> {
    let mut merged = vec![];

    // Determine what array is longer
    let first_len = first.len();
    let second_len = second.len();
    let max_length = first_len + second_len;

    let mut first_index = 0;
    let mut second_index = 0;
    for _i in 0..max_length {
        // No more elements in one of arrays
        if first_index == first_len || second_index == second_len {
            break;
        }

        if first[first_index] <= second[second_index] {
            merged.push(first[first_index]);
            first_index += 1;
        } else {
            merged.push(second[second_index]);
            second_index += 1;
        }
    }

    for i in first_index..first.len() {
        merged.push(first[i]);
    }

    for i in second_index..second.len() {
        merged.push(second[i]);
    }

    merged
}

#[allow(dead_code)]
fn insertion_sort(mut nums: Vec<i32>) {
    for j in 1..nums.len() {
        let mut i = j;
        while i > 0 && nums[i - 1] > nums[i] {
            nums.swap(i, i - 1);
            i -= 1;
        }
    }

    dbg!(&nums);
}

#[allow(dead_code)]
fn reverse_insertion_sort(mut nums: Vec<i32>) {
    for j in (0..(nums.len() - 1)).rev() {
        let mut i = j;
        while i < (nums.len() - 1) && nums[i + 1] > nums[i] {
            nums.swap(i, i + 1);
            i += 1;
        }
    }
    dbg!(&nums);
}

#[allow(dead_code)]
fn selection_sort(mut nums: Vec<i32>) {
    for i in 0..nums.len() - 1 {
        let minimal_number_index = get_minimal(&nums[i..nums.len()]);
        nums.swap(i, minimal_number_index + i);
    }
    dbg!(&nums);
}

#[allow(dead_code)]
// Return minimal value from Vec and its index
fn get_minimal(nums: &[i32]) -> usize {
    // Zero element
    let min = nums[0];
    let mut index = 0;

    for i in 0..nums.len() {
        if nums[i] < min {
            index = i;
        }
    }
    index
}
