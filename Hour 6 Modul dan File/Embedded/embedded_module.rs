
// hour6_embedded_module.rs
mod m1 {
    pub fn a() {
        println!("m1 module");
    }

    pub mod m2 {
        pub fn b() {
            println!("m2 module");
        }
    }
}

fn main() {
    m1::a();
    m1::m2::b();
}
