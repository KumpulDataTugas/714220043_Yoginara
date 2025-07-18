
// hour8_drop.rs
struct Game {
    number: i32,
}

impl Drop for Game {
    fn drop(&mut self) {
        println!("The #{} Winner.", self.number);
    }
}

fn main() {
    let _baseball = Game { number: 3 };
    let _football = Game { number: 2 };
    let _basketball = Game { number: 1 };
}
