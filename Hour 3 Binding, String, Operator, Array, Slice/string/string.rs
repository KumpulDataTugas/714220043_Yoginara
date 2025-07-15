fn main() {
    let a = "hello".to_string();             // cara 1
    let b = String::from("world");           // cara 2
    let c: &str = "rustacean";               // cara 3

    println!("{} {} {}", a, b, c);
}
