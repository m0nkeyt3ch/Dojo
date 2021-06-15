fn main() {
    assert_eq!(bmi(50, 1.80), "Underweight");
    assert_eq!(bmi(80, 1.80), "Normal");
    assert_eq!(bmi(90, 1.80), "Overweight");
    assert_eq!(bmi(110, 1.80), "Obese");
}

fn bmi(weight: u32, height: f32) -> &'static str {
    //Best Practice
    // let index = weight as f32 / height.powi(2);
    // match index {
    //     index if index <= 18.5 => "Underweight",
    //     index if index <= 25.0 => "Normal",
    //     index if index <= 30.0 => "Overweight",
    //     _ => "Obese"
    // }
    let x: f32 = weight as f32 / (height * height);
    if x <= 18.5 {
        return "Underweight";    
    } else if x <= 25.0 {
        return "Normal";    
    } else if x <= 30.0{
        return "Overweight";
    } else {
        return "Obese";
    }   
}