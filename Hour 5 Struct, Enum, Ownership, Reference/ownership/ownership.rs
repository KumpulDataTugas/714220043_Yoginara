
// hour5_ownership.rs
fn main() {
    let s = String::from("Rust ownership");
    let n = cal(s);
    // println!("Value: {}", s); // Error! s is moved
    println!("Length: {}", n);
}

fn cal(s: String) -> usize {
    s.len()
}
