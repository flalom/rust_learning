////Now that i got the basic nuances of the language, maybe
//i should move in doing something a little more complex
//////
//#########################################################
use::std::io;

fn main() {
///This functione will define the main entrance of the program///
println!("How many wells do you need to do?");
let mut wells = String::new(); 
io::stdin().read_line(&mut wells).expect("There was something very wrong!!");

println!("How many replicates do you need? ");
let mut replicates = String::new();
io::stdin().read_line(&mut replicates).expect("There was something very wrong!!");

println!("What volume (in mL) do we need in each of the wells?");
let mut volumes = String::new();
io::stdin().read_line(&mut volumes).expect("There was something very wrong!!");

//2 possibilities for this code, one not solid the other one is:
let wells: f32 = wells.trim().parse().expect("Problems in parsing the number of wells");
let replicates: f32 = replicates.trim().parse().expect("Problems in parsing the replciates");
let volumes: f32 = volumes.trim().parse().expect("Problems in parsing the volumes");
//I deleted the match statement (put it before wells, then instead of expect

//you open a { Ok(num) => num, Err(_) => break, }; 
//let wells: u32 = match wells.trim().parse() {
//Ok(num) => num,
//Err(_) => print!("Please insert a number!")};

println!("You want {} wells with {} replicates with {} mL of volume", wells, replicates, volumes);

let results : f32;
results = wells * volumes * replicates;
println!("You need at least {:.2} mL of medium", results);

}
