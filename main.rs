fn main() {
    let islem = Operation::Add(5.0,1.0);
   process_message(islem);
}

enum Operation{
    Add(f64,f64), 
    Subtract(f64,f64), 
    Multiply(f64,f64),
    Divide(f64,f64),
}

impl Operation{
    fn calculate(&self) -> f64{
        match self {
            Operation:: Add(x,y) => x+y,
            Operation:: Subtract(x,y) => x-y,
            Operation:: Multiply(x,y) => x*y,
            Operation:: Divide(x,y) => x/y,
            
        }
    }
}

fn process_message(islem:Operation) {
    match islem {
        Operation:: Add(x,y) =>  println!("{} + {} = {}",x, y, x+y),
        Operation:: Subtract(x,y) =>println!("{} + {} = {}",x, y, x-y),
        Operation:: Multiply(x,y) =>println!("{} + {} = {}",x, y, x*y),
        Operation:: Divide(x,y) => println!("{} + {} = {}",x, y, x/y),
    }
}


