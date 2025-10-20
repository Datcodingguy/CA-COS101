 // Rust program to output name and age

 use std::io;

 fn main() {
     println!("\nStudent Information Management System!");

     // input name
     println!("\nDilibe Aroh.");
     let mut name = String::new();
          io::stdin()
          .read_line(&mut name)
          .expect("Failed to read input");
     println!("Your name is: {}",name);

     // input age
     println!("\n16");
     let mut age = String::new();
          io::stdin().read_line(&mut age).expect("Failed to read input");
     let age:f32 = age.trim().parse().expect("Input not an integer");
     println!("Your age is: {}", age);     

 }

