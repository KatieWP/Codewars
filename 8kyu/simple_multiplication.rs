// DESCRIPTION:
// This kata is about multiplying a given number by eight if it is an even number and by nine otherwise.

//MY SOLUTION
fn simple_multiplication(number: u8) -> u8 {
    if number % 2 == 0 {
        number * 8
    } else {
        number * 9
    }
}
