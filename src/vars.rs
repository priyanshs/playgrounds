/* Variable hold primittive ate or reference to data 
Variables are immutable by default 
Rust is a block scoped language */

pub fn run()
{
    // let name:&str = "Priyansh";
    // let age = 28;
    // println!("My name is {} and I am {}", name, age);
    // change the value of age
    // age = 29;
    // cannot assign twice to immutable variable
    
    let name:&str = "Priyansh";
    let mut age = 28;
    println!("My name is {} and I am {}", name, age);
    age = 29;
    println!("My name is {} and I am {}", name, age);

    // Define constant 
    const ID: i32 = 031;
    println!("ID is {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Priyansh", 28);
    println!("{} is {}", my_name, my_age);

}