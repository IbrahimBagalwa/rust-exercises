struct Book {
    pages: i32,
    ratings: i32,
}

fn display_page_count(book: &Book){
    println!("pages = {:?}", book.pages);
}
fn display_rating(book: &Book){
    println!("ratings = {:?}", book.ratings);
}

fn main(){
    let book  = Book{
        pages: 100,
        ratings: 5,
    };
    display_page_count(&book);
    display_rating(&book);
}