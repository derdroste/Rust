// Variables hold primitive data or references to data
// Variables are immutable by default, so ->const
// Rust is a block-scoped language


pub fn run() {
    let name = "Lukas";
    let mut age = 29;
    println!("My name is {} and i am {}", name, age);

    age = 30;

    println!("My name is {} and i am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let ( my_name, my_age ) = ("Lukas", 29);
    println!("{} is {}", my_name, my_age);
}
