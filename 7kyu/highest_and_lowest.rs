// DESCRIPTION:
// In this little assignment you are given a string of space separated numbers, and have to return the highest and lowest number.

// Examples
// high_and_low("1 2 3 4 5")  // return "5 1"
// high_and_low("1 2 -3 4 5") // return "5 -3"
// high_and_low("1 9 3 4 -5") // return "9 -5"
// Notes
// All numbers are valid Int32, no need to validate them.
// There will always be at least one number in the input string.
// Output string must be two numbers separated by a single space, and highest number is first.

//MY SOLUTION
fn high_and_low(numbers: &str) -> String {
    let mut result = String::new();

    let v: Vec<i32> = numbers.split(' ').map(|x| x.parse().unwrap()).collect();

    result.push_str(v.iter().max().unwrap().to_string().as_str());
    result.push(' ');
    result.push_str(v.iter().min().unwrap().to_string().as_str());

    result
}
