use std::io;
use rand::Rng;
use std::cmp::Ordering;

/// main function
fn gess() {
    let secret = rand::thread_rng().gen_range(1, 101);
    println!("Guessing game :)\n----------------");

    loop {
        println!("give a number to test :)");
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("there is  an error caatching the number");

        let input: u32 = match input.trim().parse() {
            Ok(num)=> num,
            Err(_)=> continue,
        };
        match input.cmp(&secret){
                Ordering::Less => println!("too small"),
                Ordering::Greater => println!("too big"),
                Ordering::Equal => {
                    println!("you gessed right !");
                    break;
                }

        }
    }

}