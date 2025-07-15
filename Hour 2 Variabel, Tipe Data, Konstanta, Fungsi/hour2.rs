// HOUR 2: Variables, Constants, Type Conversion, Functions

// 1. Constant
const PI: f64 = 3.14;

// 2. Fungsi tanpa return (void)
fn print_greeting() {
    println!("Selamat datang di praktikum Rust Hour 2!");
}

// 3. Fungsi dengan parameter dan return
fn tambah(a: i32, b: i32) -> i32 {
    a + b
}

// 4. Fungsi dengan return tipe boolean
fn is_positive(x: i32) -> bool {
    x > 0
}

fn main() {
    // 5. Variabel biasa
    let name = "Rust";
    let umur: i32 = 8;

    // 6. Output format
    println!("Belajar {} selama {} jam", name, umur);

    // 7. Konstanta
    println!("Nilai konstanta PI adalah {}", PI);

    // 8. Konversi tipe data
    let nilai_float: f32 = 99.9;
    let nilai_bulat: i32 = nilai_float as i32;
    println!("Float: {}, setelah dikonversi: {}", nilai_float, nilai_bulat);

    // 9. Fungsi return nilai
    let hasil = tambah(10, 20);
    println!("Hasil penjumlahan 10 + 20 = {}", hasil);

    // 10. Fungsi return boolean
    let angka = -5;
    println!("Apakah {} bilangan positif? {}", angka, is_positive(angka));

    // 11. Fungsi tanpa return
    print_greeting();
}
