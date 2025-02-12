pub fn window() {

        let number = Some(None); // Option<i32> with Some value
    
        match number {
            Some(value) => println!("The number is: {}", value), // Pattern matching with Some()
            None => println!("No value found"),                 // Handling None case
        }
    
    
}
