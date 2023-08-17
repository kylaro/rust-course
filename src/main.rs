fn main() {
    let num = 25;
    let mut mars_weight = calculate_weight_on_mars(100.0);
    mars_weight = mars_weight * 1000.0;

    println!("Weight on mars: {}kg", calculate_weight_on_mars(100.0));

}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}