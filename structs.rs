fn main() {
    let book = Book{
        title : String::from("The way of Zen"),
        author : String::from("Alan Wats"),
        publication_year : 1957,  
    };

    println!(
        "The book {} is written by {} in {}.", book.title, book.author, book.publication_year
    );
    
    let mut  book = Book{
        title : String::from("The way of Zen"),
        author : String::from("Alan Wats"),
        publication_year : 1957,  
    };

    book.publication_year = 1989;

    println!(
        "The book {} is written by {} in {}.", book.title, book.author, book.publication_year
    );

    let  book_data = get_book_data(book);

    for data in book_data{
        println!("{data}");
    };

    let _my_book = create_book( "the".to_string(),"simon".to_string(),2023);

    println!("My book is {:?}", _my_book);
    let tuple_book= Tuple_book("some book".to_string(),"simon".to_string(),2024);

    let title = tuple_book.0;
    let author = tuple_book.1;
    let publication_year = tuple_book.2;

    let unit_book = Unit_Book;

    let my_recatangle =Rectangle {
        width:10.0,
        height:5.0,
    };

    let area =my_recatangle.area();
    println!("the area of the rectangle is {}", area);
}

#[derive(Debug)]

struct Book {
    title : String,
    author : String,
    publication_year : u32,
}

struct Tuple_book(String,String,u32);

struct Unit_Book;

fn get_book_data(book : Book) -> [String; 3] {
    let title = book.title;
    let author = book.author;
    let publication_year = book.publication_year;

    let data: [String; 3] = [title, author, publication_year.to_string()];
    data
}

fn create_book(title: String, author: String, publication_year: u32 ) -> Book {
    let book = Book {
        title ,  //which means "title : title". this is a nice short cut.
        author,
        publication_year,
    };
    book
}

struct Rectangle{
    width: f64,
    height: f64,
}
impl Rectangle{
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
