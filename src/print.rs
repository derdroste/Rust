pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("{} is from {}", "Lukas", "Soest");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}",
             "Lukas",
             "Soest", "code"
    );
    // Named Arguments
    println!("{name} likes to play {activity}",
             name = "Lukas",
             activity = "ALL THE DRINKING GAMES!!!111"
    );

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}, wtf thats not the result of 10 +10?", 10 + 100);

}
