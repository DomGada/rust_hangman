use std::io;
use rand::Rng;


fn main() {
    println!("Welcome to Hangman!");
    

    let mut words = ["complex", "rejected", "paralyzed", "cripple", "stagnate"];
    let wordIndex = rand::thread_rng().gen_rang(0..words.len());
    

    let word = words[wordIndex];
    let mut guessNum = 0;

    while guestNum <= 6{
        io::stdin()
            .read_line()
    }
    
    
    
}

fn guessPass(word::String){
   
}

