use std::io;

fn main() {
    // Get experience input
    println!("Is the employee experienced? (yes/no): ");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience = experience.trim().to_lowercase();

    // Get age input
    println!("Enter the employee's age: ");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: u32 = age_input.trim().parse().expect("Please enter a valid age");

    // Determine incentive
    if experience == "yes" {
        if age >= 40 {
            println!("Annual incentive: ₦1,560,000");
        } else if age >= 30 && age < 40 {
            println!("Annual incentive: ₦1,480,000");
        } else if age < 28 {
            println!("Annual incentive: ₦1,300,000");
        } else {
            println!("Annual incentive: ₦0 (no specific rule for this age range)");
        }
    } else if experience == "no" {
        println!("Annual incentive: ₦100,000");
    } else {
        println!("Invalid input. Please type 'yes' or 'no' for experience.");
    }
}

