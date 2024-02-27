use std::io;

fn main() {
    let loan_sum = input_u32("Сумма кредита: ");
    let years = input_u32("Количество лет: ");
    let rate = input_u32("Процентная ставка (%): ") / 100.0;

    let year_payment:f64 = rate * loan_sum / (1.0 - 1.0 / (1.0 + years * rate));
    let month_payment = year_payment / 12.0;
    let all_sum =  years * year_payment; 
    println!("Годовой платеж = {}", year_payment.round());
    println!("Весь долг = {}", all_sum.round());
    // println!("{}", overpayment_coef);

    let months: i32 =(12.0 * years) as i32; 

    println!(
        "{0: <10} | {1: <23} | {2: <18} | {3: <10}",
        "Период", "Долг на начало периода", "Ежемесячный платеж", "Остаток по задолженности"
    );

    for i in 0..months  {
        let year = i / 12;
        let month = i % 12;
        let time = format!("{}.{}", year + 1, month + 1);
        let start = all_sum - (i as f64) * month_payment;

        println!("{0: <10} | {1: <23} | {2: <18} | {3: <10}", time, start.round(), month_payment.round(), (start - month_payment).round());
    }
 
}

fn input_u32(input_message: &str) -> f64 {
    loop {
        let mut guess = String::new();

        println!("{}", input_message);
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: f64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong type!");
                continue;
            }
        };
        return guess;
    }
}
