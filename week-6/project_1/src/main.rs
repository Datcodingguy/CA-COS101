use std::io;

fn main() {
    println!("===========================");
    println!("     WELCOME TO TASTY HUB   ");
    println!("===========================");
    println!("Here’s our Menu:");
    println!("P = Poundo Yam & Egusi Soup  - ₦3,200");
    println!("F = Fried Rice & Chicken     - ₦3,000");
    println!("A = Amala & Ewedu Soup       - ₦2,500");
    println!("E = Eba & Egusi Soup         - ₦2,000");
    println!("W = White Rice & Stew        - ₦2,500");
    println!("===========================");

    // Get user input
    println!("Enter the letter of your choice (P/F/A/E/W): ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice = choice.trim().to_uppercase();

    // Match choice with prices
    let (item, price) = match choice.as_str() {
        "P" => ("Poundo Yam & Egusi Soup", 3200),
        "F" => ("Fried Rice & Chicken", 3000),
        "A" => ("Amala & Ewedu Soup", 2500),
        "E" => ("Eba & Egusi Soup", 2000),
        "W" => ("White Rice & Stew", 2500),
        _ => {
            println!("Invalid choice! Please select a valid menu item.");
            return;
        }
    };

    // Display result
    println!("\nYou ordered: {}", item);
    println!("Price: ₦{}", price);
    println!("===========================");
    println!("Thank you for your order!");
}

