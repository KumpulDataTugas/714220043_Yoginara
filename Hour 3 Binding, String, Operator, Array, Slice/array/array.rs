fn main() {
    // Array dengan nilai default
    let mut arr: [i32; 4] = [8; 4];
    arr[1] = 10;
    arr[2] = 20;

    println!("Array: {} {} {} {}", arr[0], arr[1], arr[2], arr[3]);

    // Array dengan nilai manual
    let arr2: [f32; 4] = [0.1, 0.2, 0.3, 0.4];
    println!("Array manual: {} {} {} {}", arr2[0], arr2[1], arr2[2], arr2[3]);
}
