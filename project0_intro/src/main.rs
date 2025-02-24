/*
    Author:Steven Yu
    Date: 2025/2/23
*/
/*
    Steven Yu's  First Rust Program：guess_number
    generate a random number between 1 and 100
    prompt the user to guess the number
    tell the user whether the guess is too big, too small, or correct
    exit when the user guesses the number
*/
mod games;

fn main() {
    println!("Guess the number!");
    let number = games::guess_number::init();
    println!("Generated a random natural number");
    println!("Please guess a natural number between 1 and 100");
    let mut result:bool = false;
    loop{
        if result == true {break;}

        println!("input your guess:");

        let mut guess = String::new();

        let mut parse_num;
        let guess_num: u8 = loop{
            guess = "".to_string();//Flush stdin buffer
            std::io::stdin()
                .read_line(&mut guess);
            //println!("{guess}");
            parse_num = guess.trim().parse::<u8>().unwrap_or_default();
            if parse_num != 0  && parse_num <= 100{
                println!("You guessed: {guess}");
                break parse_num;
            }else {
                println!("Parse Error!Check your input and retry");
                continue;
            }
        };

        result = games::guess_number::guess(guess_num,number);
    }
}
