use std::io;
fn main() {
    let a = input_float("Enter coef a: ");
    let b = input_float("Enter coef b: ");
    let c = input_float("Enter coef c: ");

    println!("Equationd: {} * X^2 + ({}) * X + ({}) = 0", a, b, c);

    if a != 0.0 {
        let d = b * b - 4.0 * a * c;
        
        if d < 0.0 {
            println!("Solution is a complex number")
        } else {
            let x1 =  ((-b) - d.sqrt()) / 2.0 * a; 
            let x2 =  ((-b) + d.sqrt()) / 2.0 * a; 

            println!("Solution is X1 = {x1}");

            if d != 0.0 {
                println!("and X2 = {x2}");
            }
        }
        return;
    }

    if b != 0.0 {
        let x = -c / b;
        println!("X is {}", x);
        return;
    }

    if c != 0.0 {
        println!("No solution")
    } else {
        println!("X is any number")
    }
}

fn input_float(input_message: &str) -> f32 {
    loop {
        let mut guess = String::new();

        println!("{}", input_message);
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: f32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong type!");
                continue
            },
        };
        return guess;
    }
}