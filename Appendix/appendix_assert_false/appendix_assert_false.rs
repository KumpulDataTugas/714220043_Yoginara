
// appendix_assert_false.rs
fn main() {
    let check: bool = false;
    assert!(check == true);
    println!("{}", check); // This line won't be reached
}
