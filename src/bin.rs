use baps::{random_permutation, factor, to_instructions};
use rand::thread_rng;
use std::io;

fn get_card_count() -> usize {
    let mut n = 0;
    let mut got_num = false;
    let mut input = String::new();
    while !got_num {
        println!("How many cards in your deck?");
        match io::stdin().read_line(&mut input) {
            Err(error) => println!("error: {}", error),
            Ok(_nbytes) => {
                match input.trim_matches('\n').parse() {
                    Err(error) => println!("error: {}", error),
                    Ok(num) => {
                        got_num = true;
                        n = num;
                    }
                }
            }
        }

    }
    n
}

fn print_instructions(instructions: &[usize]) {
    for i in instructions {
        println!("Put top card in pile {}", i + 1);
    }
    match instructions.iter().max() {
      Some(m) => println!("Stack the piles with 0 on top and {} on bottom", m),
      None => println!("Empty set of piles.")
    }
}

pub fn main() {
    let mut rng = thread_rng();
    let n = get_card_count();
    let p = random_permutation(&mut rng, n);
    let (q, r) = factor(&p);
    print_instructions(&to_instructions(&q));
    print_instructions(&to_instructions(&r));
}
