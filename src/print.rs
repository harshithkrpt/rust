pub fn run() {
    // Print to Console
    println!("Hello from the print.rs File");

    println!("{}",1);

    println!("{0} name is {1}","My","Harshith");
    // named args
    println!("{my} name is {name}",my="My",name="Harshith");
    // placeholder traits
    println!("Binary : {:b} Hex: {:x} Octal: {:o}",10,10,10);
    // placeholder for debug trait
    println!("{:?}",(12,true,"Hello"));

    // Basic Math
    println!("10 + 10 = {}",10+10);
}