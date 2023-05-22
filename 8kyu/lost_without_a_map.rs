// DESCRIPTION:
// Given an array of integers, return a new array with each value doubled.

// For example:

// [1, 2, 3] --> [2, 4, 6]

//MY SOLUTION
fn maps(values: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    result.extend(values.iter().map(|&x| x * 2));

    result
}
