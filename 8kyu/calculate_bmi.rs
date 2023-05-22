// DESCRIPTION:
// Write function bmi that calculates body mass index (bmi = weight / height2).

// if bmi <= 18.5 return "Underweight"
// if bmi <= 25.0 return "Normal"
// if bmi <= 30.0 return "Overweight"
// if bmi > 30 return "Obese"

// MY SOLUTION
fn bmi(weight: u32, height: f32) -> &'static str {
    let bmi: f32 = weight as f32 / (height.powi(2));

    if bmi <= 18.5 {
        "Underweight"
    } else if bmi > 18.5 && bmi <= 25.0 {
        "Normal"
    } else if bmi > 30.0 {
        "Obese"
    } else {
        "Overweight"
    }
}
