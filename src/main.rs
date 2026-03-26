/*
By: <Mateo machado>
Date: 2026-03-26
Program Details: <fun text-based Magic 8 Ball that uses a function to return a random answer.>
*/

use rand::{Rng, rng, rngs::ThreadRng};

fn main() {
 let mut rng: ThreadRng = rng();
 println!("Enter a question:");
 let mut question = String::new();
 std::io::stdin()
    .read_line(&mut question)
    .expect("Failed to read line");
    let dice: i32 = {
        let this = &mut rng;
        let range = 1..=6;
        this.random_range(range)
    };
    magic_8_ball(dice);}

    fn magic_8_ball(dice: i32) {
    if dice == 1 {
        println!("magic 8ball says: Better not tell you now");
    } else if dice == 2 {
        println!("magic 8ball says: SHUT UP");
    } else if dice == 3 {
        println!("magic 8ball says: my reply is NO");
    } else if dice == 4 {
        println!("magic 8ball says: sure why not what the worst that can happen");
    } else if dice == 5 {
        println!("magic 8ball says: it is decidedly so");
    } else if dice == 6 {
        println!("magic 8ball says: ask again");
    } else {
        println!("Error: Invalid output");
    }}

