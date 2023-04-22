fn main() {
    let stdin = std::io::stdin();
    let mut str = String::new();

    println!("Enter your weight");
    stdin.read_line(&mut str).unwrap();
    let weight = str.trim().parse::<f64>().unwrap();
    println!("Enter your height");
    str.clear();
    stdin.read_line(&mut str).unwrap();
    let height = str.trim().parse::<f64>().unwrap();
    let bmi = calculate_bmi(height, weight);
    println!("Your BMI is: {bmi}");
}

fn calculate_bmi(height: f64, weight: f64) -> f64 {
    weight / (height * height)
}
