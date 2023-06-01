use rand::seq::SliceRandom;

fn main() {
    loop {
        let computer_choice = choice();
        let user_choice = ask();

        if computer_choice == user_choice {
            println!("Draw");
        } else if (computer_choice == "paper" && user_choice == "rock")
            || (computer_choice == "rock" && user_choice == "scissors")
            || (computer_choice == "scissors" && user_choice == "paper")
        {
            println!("You lose!");
        } else if (computer_choice == "paper" && user_choice == "scissors")
            || (computer_choice == "rock" && user_choice == "paper")
            || (computer_choice == "scissors" && user_choice == "rock")
        {
            println!("You win!");
            break;
        } else {
            println!("Invalid choice, please choose between 'paper', 'rock', or 'scissors'.");
        }
    }
}

fn choice() -> &'static str {
    let choices = ["paper", "rock", "scissors"];
    *choices.choose(&mut rand::thread_rng()).unwrap()
}

fn ask() -> String {
    let mut line = String::new();
    println!("Enter your choice (paper, rock, scissors):");
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_lowercase()
}
