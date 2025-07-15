
// hour8_closure.rs
fn main() {
    let my_closure = |num: i32| { num + 200 };
    let num = 100;
    println!("{}", my_closure(num));

    let mut capacity = "Hard disk capacity: 5000".to_string();
    {
        let mut add_unit = |c: char| { capacity.push(c); };
        add_unit('G');
    }
    println!("{:?}", capacity);
}
