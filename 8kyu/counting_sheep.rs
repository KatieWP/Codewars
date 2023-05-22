// DESCRIPTION:
// Consider an array/list of sheep where some sheep may be missing from their place. We need a function that counts the number of sheep present in the array (true means present).

// For example,

// &[true,  true,  true,  false,
//   true,  true,  true,  true ,
//   true,  false, true,  false,
//   true,  false, false, true ,
//   true,  true,  true,  true ,
//   false, false, true,  true]
// The correct answer would be 17.

// Hint: Don't forget to check for bad values like null/undefined

//MY SOLUTION
fn count_sheep(sheep: &[bool]) -> u8 {
    let mut count: u8 = 0;
    for &s in sheep.iter() {
        if s as i32 == 1 {
            count += 1
        }
    }
    count
}
