fn main() {
    /*  let _message = "Hello, world!";
    let _x: i32=42;
    let _pi: f64= 3.14;
    let _is_rust_fun: bool = true;
    let _letter_a: char = 'a';

    fn _add(x: i32, y: i32) -> i32 {
        x+y //return x+y 
    }

    let x=4;

    if x>=0 {
        println!{"x is not negative"}
    } else{
        println!("x is negative")
    }
    let mut i=1;

    while i<=5{
        println!("{}",i);
        i +=1;
    }
//Strings 
    let _mesage = "hi, simon";
    let _my_string = String::from("Hi, Simon");

// array 
let _days_of_the_week: [&str; 7]= [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday"
];

let _first_element = _days_of_the_week[0];
let _last_element = _days_of_the_week[_days_of_the_week.len()-1];

//slices
let slice = &_days_of_the_week[1..3]; //1 den 3 e kadar olan indexleri alıyor yani 1. ve 2. indeksi.
let _first_element_of_slice = slice[0];

//Tuples
let person = ("Alice", 30);

let _name=person.0;
let _age = person.1;

//unit type

let _unit_type = ();
*/
//--------------------------------------------------------------------------------------------------
/*VARIABLE

let mut _num =5;
_num= 6;

let sum = addy(3,5);
println!("the sum is {}",sum);
//----------------------------------------------
let day_of_the_week= "sunday";

    if day_of_the_week == "sunday"{
        println!("The race day !!");
    } else if day_of_the_week == "saturday"{
        println!("Qualifiyng today!");
    } else {
        println!("Patiently wait for the race day");
    }
*/
//--------------------------------------------------------------------------------------------------
/* WHİLE

let mut counter =0;

while counter<5 {
    println!("Counter value is {}",counter);
    counter+=1;
}
*/
//--------------------------------------------------------------------------------------------------
/* FOR LOOP 

for number in 1..5 { //5 not included if 1..=5 now 5 inclueded.
    println!("Number is {}", number);
}
    /*
    let numbers[i32; 5]= [1, 2, 3, 4, 5];
    for number in numbers { 
        println!("Number is {}", number); 
    }*/
*/
//--------------------------------------------------------------------------------------------------
/*LOOP

counter =0;

loop{
    println!("Counter calue is {}", counter);
    counter+=1;

    if counter==6 {
        break;
    }
}
*/
//--------------------------------------------------------------------------------------------------
/*  LIKE SWITCH

let num =5 ;

match num {
    1=> {  // => case gibi kullanılıyor.
        println!("The number is one");
        println!("This is the frist macth arm");
    }
    2 =>  println!("The number is two"),
    3 =>  println!("The number is three"),
    _ =>  println!("The number is something else"), // else için '_' kullandık.
}

let result = match num{
    1 => "The number is one",  //this is return statement. 
    2 => "The number is two",
    3 => "The number is one",
    _ => "The number is something else",
};

println!("The result is {}", result);
*/
//--------------------------------------------------------------------------------------------------
/* OWNERSHİP

let s1 = String::from("Hello");
let s2 = s1;
// println!("value of s1 is {}",s1); HATA ALDIK ÇÜNKÜ OWNERSHİP ARTIK S2 DA.
    
let x= 5;
let y=String::from("patika");
let z= y;
println!("value of x is {}, and value of z is {}", x,z);
*/

/*  BORROWİNG AND REFERENCES

let my_string =String::from("hello world!");
let my_ref= &my_string;
//println!("My reference is {}",my_ref);

let my_string =String::from("hello world!");
//print_string(&my_string);
//println!("I still got my string {}", my_string);

let mut my_string = String::from("hello");
change_string(&mut my_string);

//println!("{}", my_string);

*/
/* 
let new_s= String::from("new String");
let new_s_ref = return_ref(&new_s);

println!("new string {}",new_s_ref);

let newer_s = new_s;

println!("new string referance {}", new_s_ref); // we got an error because now other variable allocated own the heap.
*/

}

/* fn change_string(s: &mut String){
    s.push_str(" world");
}
fn print_string(s: &String){
    println!("{}", s);
}
*/

/*fn addy(x:i32, y:i32) -> i32 {
    let result = x+y;
    return result;
}
*/

/* 
fn _no_param(){
    println!("this also works!!!")
}
*/

/*fn return_ref(s: &String) -> &String{
    s
}
*/

  
