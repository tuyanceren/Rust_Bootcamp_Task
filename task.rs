fn main(){
    let original_string = String::from("Hello, world!");
    let cloned_string = original_string.clone();
 
    println!("original string : {}", original_string);
    println!("cloned string : {}", cloned_string);

    let original_string = String::from(" String");
    let modified_string = modify_string(&original_string);

    println!("original string : {}", original_string);
    println!("modified string : {}", modified_string);
}
 
 fn modify_string(s: &String) -> String {
    let mut cloned_string = s.clone();
    cloned_string.push_str(" modified");
    cloned_string
 } 