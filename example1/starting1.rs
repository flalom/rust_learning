//interesting programing language. It is a non-GC (garbage collector) statically typed language
//it is running as fast as C
use std::io;

fn main() {
    println!("Pippo how are you??");
    println!("How many pila ave lu poccu??");
    let mut pila = String::new();
    io::stdin().read_line(&mut pila).expect("Error: unable to read user input");
}
