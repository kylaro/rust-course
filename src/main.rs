use std::io;

fn main() {
    // Note, the course is specifying weight in kg, but I think it's actually mass
    println!("Enter your weight (kg): ");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();

    println!("Weight on mars: {}kg", calculate_weight_on_mars(weight));

}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}