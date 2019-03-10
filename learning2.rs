//learnign this short code here, to make it run
//I am happy with this cool super fast language. 
//I like that I can make executables
use std::io::{stdin,stdout,Write};

fn main(){
println!("Hello how are you??");
let mut risposta = String::new();
println!("Put an answer");
stdin().read_line(&mut risposta).expect("Did not enter a correct string");
println!("Your element was {}", risposta);


}



