use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    loop {
        print!("Your Guess : ");
        io::Write::flush(&mut io::stdout()).expect("flush failed");
        let secret_number = rand::thread_rng().gen_range(1..11);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid Input");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match input.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small".red()),
            Ordering::Greater => println!("{}", "Too Big".red()),
            Ordering::Equal => {
                println!("{}", "You Win".green());
                break;
            }
        }
    }
}
