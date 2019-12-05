#[derive(Debug)]
struct Student{
    name: String,
}
impl Student{
    fn instance(name: String)->Student{
        Student{
            name,
        }
    }
}
fn main() {

    let student1=Student::instance(String::from("Muhammad Waqar")); //making instance with associated function

    let student2=Student{
        
        name: String::from("Uzair Ahmed"),
    
    };// making instance directly 

    //  -> As we seen here, when we creates the instance by associate function we just have to pass the data/value in fuction.  
    //     but when we create instance directly we have to write the extra things. so the first benefit is rid of extra coding.
    //  -> And the second thing is data hiding or privates the data. that will be benefitent when we work in team and we don't 
    //     want to give data permission by Someone. 

    // here we print the instances.
println!("{:?}", student1);
println!("{:?}", student2);
    
}
