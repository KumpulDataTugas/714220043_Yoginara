fn main() {
    let arr = [0, 10, 20, 30, 40, 50];
    let slice = &arr[2..5]; // ambil elemen indeks 2–4

    println!("Isi slice:");
    println!("{}", slice[0]); // 20
    println!("{}", slice[1]); // 30
    println!("{}", slice[2]); // 40
}
