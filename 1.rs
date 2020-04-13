fn main()
{
    println!("{} , {}!","Hello","World");
    println!("{0} {1}","Hello","World");
    println!("{greeting} {name}",greeting = "Hello",name = "world");

    println!("{:?}",[1,2,3]);
    println!("{:#?}",[1,2,3]);

    let x = format!("{} {}!","Hello","World");

    println!("{}",x);

    let y = String::from("Hello,") + "world";

    println!("{}",y);

    let a = true;
    let b: bool = true;

    let (x,y) = (1,2);

    let mut z = 5;
    z = 6;

    const N: i32 = 5;
    static N2: i32 = 5;
    
}