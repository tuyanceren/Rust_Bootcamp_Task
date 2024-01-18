use std::collections::HashMap;

fn main() {

    let mut my_map = HashMap::new();

    my_map.insert("Alice".to_string(),10);
    my_map.insert("Bob".to_string(),20);

    for (key,value) in my_map.iter() {
        println!("{} has {}", key, value);
        }
    

    let number = vec![1, 2, 3, 4, 5];

    let doubled : Vec<i32> = number.iter().map(|x| x*2).collect(); //fn multiply(x: i32) -> i32 {x*2}
    println!("{:?}",doubled);

/* 
    let even_numbers : Vec<i32> = number.into_iter().filter(|x|x%2==0).collect();
    println!("{:?}",even_numbers);
*/
    let sum : i32 = number.iter().fold(0, |acc, x| acc + x);
    println!("{}", sum);

/* 
    let chained: Vec<i32> = number.into_iter().filter(|n| n%2==0).map(|x| x*2).collect();
    println!("{:?}",chained);
*/
    let squared_number: HashMap<_, _> = number.iter().map(|a| (a, a*2)).collect();
    println!("{:?}",squared_number);
}



