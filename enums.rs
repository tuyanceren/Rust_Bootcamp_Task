fn main() {
    let current_weather = Weather::Sunny;

    let msg = Message::Write(String::from("Hello, Rust!"));

    process_message(msg);

    let my_pet = Animal::Cat("Cansu".to_string());

    if let Animal::Cat(name) = my_pet{
        println!("My cat name is {}.", name);
    } else{
        println!("My pet is not a cat.")
    }

    let msg= Message::Write(String::from("Cansu is sleeping"));

    msg.call();
}   

enum Weather{
    Sunny,
    Couldy,
    Rainy,
    Snowy,
}

enum Message{
    Quit,
    Move{ x:i32, y:i32 },
    Write(String),
    ChangeColor(i32,i32,i32),
}
impl Message{
    fn call(&self){
        match self {
            Message:: Quit => println!("Quit"),
            Message:: Move {x,y}=> println!("Move to x: {}, y: {}",x,y),
            Message:: Write(s) => println!("Write: {}",s),
            Message:: ChangeColor(r,g,b) => println!("Change the color to R : {}, G: {}, B: {}", r, g, b),
        }
    }
}

fn process_message(msg:Message) {
    match msg {
        Message::Quit => {
            println!("The quit variant has no data.");
        }
        Message::Move{x,y} => {
            println!("Move to coordinatess x: {}, y: {}",x,y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r,g,b) => {
            println!("Change the color to red : {}, green: {}, blue: {}", r, g, b);
        }

    }
}

enum Animal {
    Dog(String),
    Cat(String),
    Bird(String),
}