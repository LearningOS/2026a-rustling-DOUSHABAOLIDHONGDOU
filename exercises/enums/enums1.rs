// enums1.rs
//
// No hints this time! ;)



// enum 枚举只能包含一个类型的值，不能包含多个类型的值，其值只能是里面列出的某一个变体

#[derive(Debug)]
enum Message
 {
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
