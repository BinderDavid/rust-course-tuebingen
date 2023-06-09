fn main() {
    println!("Enter your weight");
    let weight = Weight(read_f64());

    println!("Enter your height");
    let height = Height(read_f64());

    let (bmi, score) = calculate_bmi(height, weight);
    println!("Your BMI is: {bmi}");
    match score {
        Score::Under => println!("You are underweight"),
        Score::Normal => println!("Your weight is in the normal range"),
        Score::Over => println!("You are overweight"),
    }
}

fn read_f64() -> f64 {
    let stdin = std::io::stdin();
    let mut str = String::new();
    loop {
        stdin.read_line(&mut str).unwrap();
        let res = str.trim().parse::<f64>();
        match res {
            Ok(res) => return res,
            Err(_) => {
                str.clear();
                println!("Try again.")
            }
        }
    }
}

#[repr(transparent)]
struct Height(f64);

#[repr(transparent)]
struct Weight(f64);

#[derive(PartialEq, Debug)]
enum Score {
    Under,
    Normal,
    Over,
}

fn calculate_bmi(height: Height, weight: Weight) -> (f64, Score) {
    let bmi = weight.0 / (height.0 * height.0);
    let score = match bmi {
        x if x < 18.4 => Score::Under,
        x if x < 24.9 => Score::Normal,
        x if x < f64::MAX => Score::Over,
        _ => panic!("invalid bmi"),
    };
    (bmi, score)
}

#[cfg(test)]
mod tests {

    #[test]
    fn calculate_bmi_1() {
        assert_eq!(
            crate::calculate_bmi(crate::Height(2.0), crate::Weight(100.0)),
            (25.0, crate::Score::Over)
        )
    }
}
