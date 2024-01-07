use std::io; //Standard input/output
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    //Name of the game gets printed
    println!("Guess the number!");

    //Generating Random Number
    let secret_number=rand::thread_rng().gen_range(1,101);
    //println!("The secret number is: {}",secret_number);

    loop{
        //Asking for user input
        println!("Please input your guess.");

        //Creating a mutable variable
        let mut guess=String::new();

        //Using standard input to take input and passing it to guess(mutable refrence)
        io::stdin().read_line(&mut guess)
        //Using error handling by expect
            .expect("Failed to read line");

        //Converting the guess{string} to guess{number:u32}
        let guess:u32= match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
           // .expect("Please type a number!");

        //Printing the guessed value by using placeholder
        println!("You guessed: {}",guess);

        //matching the output of comparison between the guess and secret_number
        match guess.cmp(&secret_number){
            Ordering::Less=>println!("Too small!"),
            Ordering::Greater=>println!("Too big!"),
            Ordering::Equal=>{
                println!("You win!");
                break;
            }
        }
    }
}
