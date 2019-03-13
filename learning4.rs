////Now that i got the basic nuances of the language, maybe
//i should move in doing something a little more complex
//////
//#########################################################
use::std::io;

fn main() {
///This functione will define the main entrance of the program///
println!("How many conditions do you need to do?");
let mut conditions = String::new(); 
io::stdin().read_line(&mut conditions).expect("There was something very wrong!!");

println!("How many replicates do you need? ");
let mut replicates = String::new();
io::stdin().read_line(&mut replicates).expect("There was something very wrong!!");

println!("What volume (in mL) do we need in each of the conditions?");
let mut volumes = String::new();
io::stdin().read_line(&mut volumes).expect("There was something very wrong!!");

//2 possibilities for this code, one not solid the other one is:
let conditions: f32 = conditions.trim().parse().expect("Problems in parsing the number of conditions");
let replicates: f32 = replicates.trim().parse().expect("Problems in parsing the replciates");
let volumes: f32 = volumes.trim().parse().expect("Problems in parsing the volumes");
//I deleted the match statement (put it before conditions, then instead of expect

//you open a { Ok(num) => num, Err(_) => break, }; 
//let conditions: u32 = match conditions.trim().parse() {
//Ok(num) => num,
//Err(_) => print!("Please insert a number!")};

println!("You want {} conditions with {} replicates with {} mL of volume", conditions, replicates, volumes);

let results : f32;
results = conditions * volumes * replicates;
println!("You need at least {:.2} mL of medium", results);
println!("You will need {} mL of parasites", conditions * replicates * (volumes/2.00));
println!("Assuming that you want a 1:2 dilution and that your initial stock is 10 mM and that you will use a 
1 NTS/1 uL concentration...caclulating...");

//let concentration : f32;
let dil_calculation : f32;

println!("Input the final concentration that you need in your assay in uM");

let mut concentration = String::new();
io::stdin().read_line(&mut concentration).expect("There was something very wrong in the caluclation of the dilution!!");
let concentration: f32 = concentration.trim().parse().expect("Problems in parsing the number of conditions");

//printing out the concentration needed in a 2 times dilution;
dil_calculation = (((volumes * concentration) / 10000.00) * 2.00 * 1000.00);
println!("You will need to add {} uL of 10 mM stock into your {} mL and then add {} mL of parasites. 
This solution will have a cocnetration of DMSO = {:.2}% at the highest drug concentration", 
dil_calculation, volumes - (dil_calculation/1000.00), volumes/2.00, ((dil_calculation/1000.00)/volumes) *50.00);

if dil_calculation <= 0.5 {
println!("However, the volume to take from the drug it's too low consider a 10-fold prediution, 
so you can use {} uL", dil_calculation * 10.00)
}; 

}
