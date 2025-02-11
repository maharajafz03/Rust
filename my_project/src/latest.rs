//Version 1: If you want to work with Options
pub fn print_name(name: Option<String>) {
    match name {
        Some(value) => println!("Name is: {}", value),
        None => println!("No name provided"),
    }
}

pub fn latest() { 
    let name = Some("Alice".to_string());
    let empty_name: Option<String> = None;

    print_name(name);
    print_name(empty_name);
    println!("hello from latest");
}
