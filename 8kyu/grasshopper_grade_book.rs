//DESCRIPTION

// Grade book
// Complete the function so that it finds the average of the three scores passed to it and returns the letter value associated with that grade.

// Numerical Score	Letter Grade
// 90 <= score <= 100	'A'
// 80 <= score < 90	'B'
// 70 <= score < 80	'C'
// 60 <= score < 70	'D'
// 0 <= score < 60	'F'
// Tested values are all between 0 and 100. Theres is no need to check for negative values or values greater than 100.

//MY SOLUTION
fn get_grade(s1: u16, s2: u16, s3: u16) -> char {
    match [s1, s2, s3].iter().sum::<u16>() / 3 {
        0..=59 => 'F',
        60..=69 => 'D',
        70..=79 => 'C',
        80..=89 => 'B',
        _ => 'A' //Since there is no need to check for values greater than 100 usin _ is ok,if not this would be 90..=100.
    }
}