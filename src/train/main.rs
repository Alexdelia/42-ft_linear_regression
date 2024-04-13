mod load;

const LEARNING_RATE: f64 = 0.01;

fn estimate(theta0: f64, theta1: f64, x: f64) -> f64 {
    theta0 + (theta1 * x)
}

fn main() {
    println!("Hello, world!");

    load::run("./ressource/data.csv").unwrap();
}
