fn main() {
    let mut fibonacci = Fibonacci{current: 0, next:1, };

    //for _ in 0..10{
    //    println!("{}",fibonacci.next().unwrap());
    //}


    //-------------------------------------------------------------

    let mut vec = vec![1, 2, 3, 4, 5];

    for item in vec.iter(){
        println!("{}",item);
    }
    

    /*for item in vec.iter_mut(){
        *item *=2;
        println!("{}",item);
    }
    */
    
    /*for item in vec.into_iter(){  //into_iter: This iterator type is used when you want to iterate over a collection by consuming it. With into_iter, you can access the elements in a collection and consume them, which means that the collection will no longer be available after the iteration.
        println!("{}",item);
    }
    */  
    

}

struct Fibonacci {
    current: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current =self.current;
        self.current = self.next;
        self.next = current + self.next;

        Some(current)
    }
}