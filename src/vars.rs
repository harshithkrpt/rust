// variable hold primitive data or reference to data
// variables are immutable by default
// Rust is a Block Level Scoped Language

pub fn run(){
    let name = "Harshiht";
    let mut age = 37;

    age = 38;

    println!("My Name is {} and I am {}",name,age);

    // Define Const 
    const ID: i32 = 001;
    println!("ID : {}",ID);

    // Assign Multiple Variables
    let (my_name,my_age)= ("Brad",37);
    println!("{} is {}",my_name,my_age);
}