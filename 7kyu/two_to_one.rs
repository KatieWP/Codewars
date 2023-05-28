// DESCRIPTION:
// Take 2 strings s1 and s2 including only letters from a to z. Return a new sorted string, the longest possible, containing distinct letters - each taken only once - coming from s1 or s2.

// Examples:
// a = "xyaabbbccccdefww"
// b = "xxxxyyyyabklmopq"
// longest(a, b) -> "abcdefklmopqwxy"

// a = "abcdefghijklmnopqrstuvwxyz"
// longest(a, a) -> "abcdefghijklmnopqrstuvwxyz"

//MY SOLUTION:
use itertools::Itertools;
fn longest(a1: &str, a2: &str) -> String {
    //Joing both strings
    //Convert to char to use sorted()
    //dedup() removes consecutive repeated elements from the source
    //Transform back to string with collect
    format!("{}{}", a1, a2)
        .chars()
        .sorted()
        .dedup()
        .collect::<String>()
}
