extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    println!("GUESS GAME!");
    let secret_number = rand::thread_rng().gen_range(0..=5);
    
    loop{
        println!("Enter secret number: ");
        let mut guess = String::new();
        let msg = "";
        
        io::stdin().read_line(&mut guess).expect(msg);
        
        let guess: u32 = guess.trim().parse().expect(msg);
        
        println!("Secret number is {}", secret_number);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low :( "),
            Ordering::Equal => {
                println!("You Win");
                break;
            },
            Ordering::Greater => println!("Too high"),
        }
    }

}
