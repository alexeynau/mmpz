use std::io;

fn main() {
    let a: f64 = input_u32("Enter a: ").into();

    if a <= 0.0 {
        println!("a can't be less or equal zero")
    }
    let n = a.log(3.0);
    // println!("n = {}", n);
    // println!("n.round() = {}", n.round());
    // println!("n - n.round() = {}", n - n.round());
    // println!("4.0*f64::EPSILON = {}", (n.round() - 1.0) * f64::EPSILON);
    if (n - n.round()).abs() <= (n.round() - 1.0) * f64::EPSILON {
        println!("{} is 3^{}", a, n.round());
    } else {
        println!("{} is not a degree of 3", a);
    }
}

fn input_u32(input_message: &str) -> u32 {
    loop {
        let mut guess = String::new();

        println!("{}", input_message);
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong type!");
                continue;
            }
        };
        return guess;
    }
}
