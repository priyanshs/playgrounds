pub fn run()
{
    // Print to console
    println!("Print from the print.rs file"); 
    
    // basic formatting
    println!("Brad is from {}", "New York");
    
    // positional arguments
    println!("{0} is from {1} and not {0}",
     "Brad", "New York");
    
     // named arguments
    println!("{name} is from {city} and not {name}",
     name = "Brad", city = "New York");
    
     // Placeholder traits
     println!("Binary: {:b} Hex: {:x} Octal: {:o}",10,10,10);

     // Placeholder for debug trait
     // Can put any number of values atfer the curly braces {:?}
     println!("Data {:?}", (12, "Priyansh"));

    // can do basic math too
    println!("{}",10+10);


}