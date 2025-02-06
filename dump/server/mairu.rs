fn main() {
    let mut name = String::from("welcome to the world");

    {
        let name1 = &mut name; // Create a mutable reference
        name1.push_str(" fucking ");
        println!("{}", name1);
    } // `name1` goes out of scope here

    // Now you can use and print `name` safely
    println!("{}", name); // Prints: welcome to the world fucking

    let mut name = String::from("welcome to the world");
    modify_string(&mut name);

    println!("{}", name); // Prints: welcome to the world fucking

    struct Point<T, A> {
        x: T,
        y: A,
    }
    
    
        let point = Point { x: 5, y: true };
        println!("Point: ({}, {})", point.x, point.y);
    
    
}


fn modify_string(name: &mut String) {
    name.push_str(" fucking ");
}


