use rand::{Rng, thread_rng};
use std::env;
use std::io;
use std::process::exit;


fn main() {
    let mut rng = thread_rng();
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Wrong argument count!");
        exit(1);
    }

    let max_num_for_random = args[1].parse().expect("May Range was not parsable!");
    let num_to_guess = rng.gen_range(0..=max_num_for_random);
    let tries = args[2].parse().expect("Tries not parsable");

    for i in 1..=tries {
        let mut buf = String::new();
        let _read = io::stdin().read_line(&mut buf).expect("Could not read from cmd");
        let user_input: u32 = buf.trim().parse().expect("Colud not pars number");

        if user_input > max_num_for_random {
            println!("You enterd a number outside of the range!\nRange is from 0 to {}\
              -- you enterd {}!\nYou wasted one try! Tries left: {} out of {}.",
                     max_num_for_random, user_input, (tries - i), tries);
            continue;
        }

        let msg = match user_input.cmp(&num_to_guess) {
            std::cmp::Ordering::Less => "Too low",
            std::cmp::Ordering::Equal => {
                println!("Perfect!\nThe number we looked for was: {}!\n\
                It took you {} tries.", num_to_guess, i);
                break;
            },
            std::cmp::Ordering::Greater => "Too high"
        };

        println!("{} ({} tries left out of {})", msg, (tries - i), tries);
    }

}
