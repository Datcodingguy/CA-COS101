use std::io;
use std::f32; // for floating-point operations

fn main() {
    // Prompt for coefficients
    println!("Enter coefficients a, b, and c:");

    // Read user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split and parse inputs
    let parts: Vec<f32> = input
        .split_whitespace()
        .map(|s| s.parse::<f32>().expect("Please enter valid numbers"))
        .collect();

    if parts.len() != 3 {
        println!("Please enter exactly three numbers (a, b, and c).");
        return;
    }

    let (a, b, c) = (parts[0], parts[1], parts[2]);

    // Calculate discriminant
    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        // Two distinct real roots
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two real roots: x₁ = {:.3}, x₂ = {:.3}", root1, root2);
    } else if discriminant == 0.0 {
        // One real root
        let root = -b / (2.0 * a);
        println!("One real root: x = {:.3}", root);
    } else {
        // Complex roots
        let real_part = -b / (2.0 * a);
        let imag_part = (-discriminant).sqrt() / (2.0 * a);
        println!("Complex roots: x₁ = {:.3} + {:.3}i, x₂ = {:.3} - {:.3}i", real_part, imag_part, real_part, imag_part);
    }
}


