#[derive(Debug)]
struct Book{
    author: String,
    name:String,
}
impl Book{
    fn instance_of_book(author:String,name:String)->Book{
        Book{
            author,
            name,
        }

    }
}
trait BookInformation{
fn info(&self)->String;
}
impl BookInformation for Book{
    fn info(&self)->String{
        format!("{1} by {0}",self.author,self.name)
    }
}
fn main() {
let book1 = Book::instance_of_book(
    String::from("Rahmat Ali Khan"), 
    String::from("Rahmat ka Faristy")
        );
print!("{}",book1.info());
}
