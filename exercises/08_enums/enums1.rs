// enums1.rs
//
// No hints this time! ;)

// I AM DONE

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("Hello")));
    println!("{:?}", Message::Move { x: 10, y: 20 });
    println!("{:?}", Message::ChangeColor(0, 160, 255));
}
