fn main() {
    let dog = Dog {
        name: "Rudolf".to_string(),
    };
    //dog.speak();

    let cow = Cow {
        name: "Milka".to_string(),
    };

    //cow.speak();

    let original_str = String::from("This is the original.");
    let copy_str = original_str.display();

    //println!("{}",copy_str);

    animal_speak(&cow);
    animal_speak(&dog);

    let cat = Cat;

    cat.make_sound();
    cat.walk();
    cat. sleep();
}   


trait Speak {
    fn speak(&self);
}

struct Dog {
    name: String,
}

struct Cow {
    name: String,
}

impl Speak for Dog{
    fn speak(&self) {
        println!("{} says: woof",self.name);
    }
}

impl Speak for Cow{
    fn speak(&self){
        println!("{} says: mooo",self.name);
    }
}

trait Display {
    fn display(&self) -> String;
}

impl Display for String{
    fn display(&self) -> String {
        self.clone()
    }
}

fn animal_speak<T:Speak>(animal: &T) {
    animal.speak();
}

trait Animal{
    fn make_sound(&self);

    fn sleep(&self){
        println!("Animal is sleepin...");
    }
}

trait Mammal: Animal {
    fn walk(&self);
}

trait Bird: Animal {
    fn fly(&self);
}

struct Cat;

impl Animal for Cat{
    fn make_sound(&self){
        println!("Miyaaaav");
    }
}

impl Mammal for Cat{
    fn walk(&self){
        println!("The cat is walking");
    }
}