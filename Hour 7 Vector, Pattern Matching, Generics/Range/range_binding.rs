
// hour7_range_binding.rs
fn main() {
    let x = 5;
    match x {
        var @ 2..=6 => println!("{}", var),
        _ => println!("others"),
    }
}
