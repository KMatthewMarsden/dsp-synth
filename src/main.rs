mod outwav;

fn main() {
    println!("Hello, world!");
    for n in 1..256 {
        let mut x = outwav::get_oscillator(n as f32, 256 as f32);
        println!("{:#?}", x);
    }
}
