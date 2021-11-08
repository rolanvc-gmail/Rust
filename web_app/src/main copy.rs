use rand::prelude::*;
use std::env;


/// This function  generates a float number using a number
/// generator passed into the function. 
/// 
/// # Arguments
/// *generator( &mut ThreadRng): the random number
/// generator to generate the random number
/// 
/// # Returns
/// (f64): random number between 0 -> 10

fn generate_float(generator: &mut ThreadRng)-> f64 {
    let placeholder: f64 = generator.gen();
    return placeholder * 10.0
}

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();

    let random_number = generate_float(&mut rng);
    let args: Vec<String> = env::args().collect();
    let path: &str = &args[0];
    println!("");
    if path.contains("/debug/") {
        println!("****The development app is running****");
    }
    else if path.contains("/release/"){
        println!("The production server is running");
    }else {
        println!("*****The setting is neither debug or release*****");
    }
    println!("{:?}", args);
    println!("A random number is:{}", random_number);
    println!("");
}
