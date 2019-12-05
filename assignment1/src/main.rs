#[derive(Debug)]
struct NewsArticle {
    author: String, 
    content: String
}
struct Tweet {
    username: String,
    content: String,
}

pub trait Summary {
    fn summarizes_author() -> String;
    fn summarizes(&self)->String
    {
        
    }

    
}

impl Summary for Tweet{
    fn summarizes(&self)->String{
        format!("{1}, tweet by {0} ",self.username,self.content)
    }
}
impl Summary for NewsArticle{
    fn summarizes(&self)->String{   
         format!("{1}, by {0} ",self.author,self.content)
    }
}
fn main() {
    let article = NewsArticle{
            author:  String::from("Imran khan"),
            content:  String::from("naya pakistan")
    };

    let twt = Tweet{
            username: String::from("Sarfraz Ahmed"),
            content: String::from( "Dream of victory")

    };
    println!("\nTweet: \t{} \nArticle: {}",twt.summarizes(),article.summarizes())
}
