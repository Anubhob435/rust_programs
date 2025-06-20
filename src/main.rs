use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let n: u8 = rng.gen();
    println!("Random number: {}", n);
}