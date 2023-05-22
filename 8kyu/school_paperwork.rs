// DESCRIPTION:
// Your classmates asked you to copy some paperwork for them. You know that there are 'n' classmates and the paperwork has 'm' pages.

// Your task is to calculate how many blank pages do you need. If n < 0 or m < 0 return 0.

// Example:
// n= 5, m=5: 25
// n=-5, m=5:  0

// MY SOLUTION
fn paperwork(n: i16, m: i16) -> u32 {
    if n < 0 || m < 0 {
        0 as u32
    } else {
        (n.abs() as u32) * (m.abs() as u32)
    }
}
