use std::io;

fn main() {
    println!("Enter your weight(kg):");
    let weight = read_weight_from_stding();
    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars is: {} kg", mars_weight);
}

fn read_weight_from_stding() -> f32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    weight
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}