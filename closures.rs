fn main() {
    let my_closure = || println!("Defining clousers");

    my_closure();

    let even_numbers = |x: i32| ->bool {x % 2 == 0}; 

    let first = even_numbers(4);
    let second = even_numbers(5);

    println!("first number is even {}",first);
    println!("second number is evenc {}",second);
    
}
