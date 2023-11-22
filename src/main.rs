extern  crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the Number:-");

    let secret_number = rand::thread_rng().gen_range(1,101);
    
    loop{
        println!("Please input your number");

        let mut guess = String::new();        

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading message");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("your num is {}",guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Equal => 
            {
                println!("You win");
                break;
            },
            Ordering::Greater => println!("Too big")
        }
    }
    
}
