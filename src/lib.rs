use rand::Rng;
use std::io;

pub fn guess_num(secret: Option<u8>) {
    println!("Please guess a number between 1 to 10!");
    println!("Input your number");

    let num_in_mind = match secret {
        Some(value) => value,
        None => rand::thread_rng().gen_range(1..=10),
    };

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Could not read line");

    guess = guess.trim().to_string();

    if guess.trim().is_empty() || guess.eq(":q") {
        println!("Sad to see you go, please come play som time");
        return;
    }

    let guessed_num = guess.parse::<u8>().expect("Invalid number input");

    let is_correct = guessed_num == num_in_mind;

    let verdict = if is_correct {
        "right"
    } else {
        "wrong"
    };

    if verdict == "wrong" {
        println!("You guessed: {}, it is wrong try again!", guessed_num);
        guess_num(Some(num_in_mind));
        return;
    }

    print!("You guessed {}; correct number is {}", verdict, num_in_mind);
}
