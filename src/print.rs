pub fn run(){
    //Print to console

    println!("Hello from other file!");
    println!("Number: {}", 1);
    println!("{0} is from {1} and {0} likes to {2}", "Alex", "Indaiatuba", "code");


    //Named Arguments

    println!("{name} likes to play {activity}", name = "John", activity = "Baseball");



    // Placeholder traits

    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait

    println!("{:?}", (12, true, "hello"));


    //basic math

    println!("10 + 10 = {}", 10 + 10)

}