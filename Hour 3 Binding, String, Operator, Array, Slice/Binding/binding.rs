fn main() {
    // Binding biasa (immutable)
    let (x, y) = (100, 200);
    println!("x = {}, y = {}", x, y);

    // Mutable
    let mut a = 10;
    a += 5;
    println!("a setelah ditambah 5 = {}", a);
}
