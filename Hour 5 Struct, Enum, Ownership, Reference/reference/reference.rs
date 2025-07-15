
// hour5_reference.rs
fn main() {
    let s = String::from("Rust reference");
    let n = cal(&s);
    println!("Value: {}", s);
    println!("Length: {}", n);
}

fn cal(s: &String) -> usize {
    s.len()
}
