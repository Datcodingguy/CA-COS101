
use std::io;

fn main () {
    let item_code = String::new();
    io::stdin().read_line(&mut item_code).expect("Not a valid string");
    let ic:u32 = item_code.trim().parse().expect("Not a valid code");

    let l = laptop = 550_000;
    let m = monitor = 120_000;
    let k = keyboard = 15_000;
    let h = headset = 25_000;

    let l, m k, h = item_code
    println!("Enter item code: {}",item_code);

    let quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Not a valid string");
    let q:u32 = quantity.trim().parse().expect("Not a valid number");
    
    println!("Enter quantity: {}",quantity);
     
     loop {
         println!("Enter item code: {}",item_code);
         
         println!("Enter quantity: {}", quantity);

         if item_code==n {
            break;
         }
     }
     let total_cost = q * item_code;
}