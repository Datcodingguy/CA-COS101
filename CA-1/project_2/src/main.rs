// Rust program to calculate compound interest and amount

use std::io;

fn main() {
    let mut principal = String::new();
    let mut rate = String::new();
    let mut time = String::new();

    println!("Enter principal: ");
    io::stdin().read_line(&mut principal).expect("Not a valid string");
    let p:f32 = principal.trim().parse().expect("Not a valid number");

   println!("Enter rate: ");
    io::stdin().read_line(&mut rate).expect("Not a valid string");
    let r:f32 = rate.trim().parse().expect("Not a valid number");
   
   println!("Enter time: ");
    io::stdin().read_line(&mut time).expect("Not a valid string");
    let t:f32 = time.trim().parse().expect("Not a valid number");

    let a:f32 = p * (1.00 + r/100.00) * t;
        println!("Amount is: {}",a);

    let ci:f32 = a - p;
        println!("Compound interest is: {}",ci);
}