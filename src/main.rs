extern crate rand;

use rand::Rng;

type Point = (f64, f64);

fn main() {
    println!("Hello, world!");

    let mut vec = Vec::new();
    let mut rng = rand::thread_rng();
    
    vec.push(1.0f64);
    vec.push(2.0f64);
    vec.push(3.0f64);
    vec.push(14.0f64);

    for i in 1..10 {
        vec.push(rng.gen::<f64>());
    }
    for i in vec {
        println!("X: {} \t Y: {}", rng.gen::<Point>().0, rng.gen::<Point>().1);
    }
}
