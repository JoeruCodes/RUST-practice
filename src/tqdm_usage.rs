use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::time::Duration;
use std::thread;
use indicatif::{ProgressBar, ProgressStyle};

fn main(){
    const DELAY:u64 = 10;
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    println!("Please input your guess.");


    loop {

        let mut guess = String::new();

        
        io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
        
        let guess:u32 = guess.trim().parse().expect("nope!");

    
    
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                create_progress_bar(DELAY);
                println!("Too small! \n");
                continue;
            },
            Ordering::Greater => {
                create_progress_bar(DELAY);
                println!("Too big! \n");
                continue;
            },
            Ordering::Equal => {
                create_progress_bar(DELAY);
                println!("You win! \n");
                break;
            },
        }
    }
    println!("You guessed it right! \n")

}

fn create_progress_bar(delay: u64) {

    let spinner = ProgressStyle::default_spinner()
        .template("[{spinner}] Processing! ")
        .expect("Failed to create spinner style")
        .tick_strings(&["-", "/", "|", "\\"]);

    let pb = ProgressBar::new_spinner();
    pb.set_style(spinner);

    for _ in 0..delay {
        pb.inc(1);
        thread::sleep(Duration::from_millis(100));
    }

    pb.finish_with_message("Task completed!");
}
