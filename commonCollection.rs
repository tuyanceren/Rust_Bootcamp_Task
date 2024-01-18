use std::collections::HashMap;

fn main() {
    //VECTOR

    let mut numbers = vec![1, 2, 3, 4];

    let mut names: Vec<String> = Vec::new();

    names.push(String::from("Alice")); // [Alice]
    names.push(String::from("Bob")); // [Alice, Bob]

    let first_name = &names[0];
    let second_name = &names[1];
    
    println!("first name is {}. second name is {}.",
    first_name, second_name);

    names.pop(); // [Alice]

    for number in numbers{
        println!("The number is {}", number);
    }

    //let slice = &numbers[1..3]; //[2, 3] 
    
    //STRING

    let mut my_string = String::from("my");
    let mut second_string = "second string ".to_string();

    my_string.push_str(" string");
    
    println!("{}", my_string);

    for c in my_string.chars(){
        println!("{}",c);

    }

    for b in my_string.bytes(){
        println!("{}",b);

    }
    //HASHMAP
    let mut scores = HashMap::new();

    scores.insert(String::from("Alara"),10); //[Alara: 10]
    scores.insert(String::from("Boby"),20); //[Alara: 10,Boby: 20]

    let Alara_score = scores.get(&String::from("Alice"));

    // println!("{}",Alara_score); // we get an error because alara dosent return the value we put in there, it return an option that holds are value.
    println!("{:?}",Alara_score);  // {:?} this is solve the problem.
    println!("{:?}", scores);
    
    scores.remove(&String::from("Boby"));

    println!("{:?}", scores);

    for(key, value ) in &scores{
        println!("{} {}",key ,value);
    }
}
