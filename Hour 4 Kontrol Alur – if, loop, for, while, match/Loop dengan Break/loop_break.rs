fn main() {
    let mut num = 5;

    loop {
        println!("Rust di jam {}!", num);
        if num == 8 {
            break;
        }
        num += 1;
    }
}
