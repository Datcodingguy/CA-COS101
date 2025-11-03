 // rust program that takes student's name and test scores to calculate average

 use std::io;
 
 fn main() {  
    //input name
    println!("Enter your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    //input test scores
    println!("Enter first test score:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter second test score: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
   let b:f32 = input2.trim().parse().expect("Not a valid number");

   println!("Enter third test score: ");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not a valid string");
   let c:f32 = input3.trim().parse().expect("Not a valid number");

   let mut average:f32 = (a + b + c) / 3.00;

   if average >= 70.00 {
      println!("{}, your grade is A ", name);
   }
   else if average >= 60.00 && average < 69.00 {
    println!("{}, your grade is B", name);
   }
   else if average >= 50.00 && average < 59.00 {
     println!("{}, your grade is C", name);
   }
   else if average >= 40.00 && average < 49.00 {
    println!("{}, your grade is D", name);
}
else{
    println!("{}, your grade is F", name);
}
}