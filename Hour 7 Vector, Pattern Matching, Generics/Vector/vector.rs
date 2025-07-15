
// hour7_vector.rs
fn main() {
    let v = vec![100, 200, 300, 400];
    println!("First element is: {}", v[0]);
    println!("Second element is: {}", v[1]);
    println!("Third element is: {}", v[2]);
    println!("Fourth element is: {}", v[3]);

    let v2 = vec![8; 3];
    println!("Repeated values: {} {} {}", v2[0], v2[1], v2[2]);

    let mut v3 = Vec::new();
    v3.push('R');
    v3.push('U');
    v3.push('B');
    v3.push('Y');
    for n in &v3 {
        print!("{}", n);
    }
}
