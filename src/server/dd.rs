fn main() {
    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.pop();
    numbers.push(3);

    println!("{:?}", numbers); // Prints: [1, 2, 3]

    let mut fruits = vec!["apple", "banana", "cherry"];
    fruits.push("date");
    fruits.pop();
    for fruit in &fruits {
        println!("{}", fruit);
    }


    let mut maga = String::from("hello world");
    let raja = &mut maga;
    raja.push_str(" fixed ");


    // println!("{}", maga);
    println!("{}", raja);

    println!("------------------------------------------------------");


    let yamuna = String::from("my_grilfriend");
    let mut kiran = yamuna;
    kiran.push_str(" for_me");
    
    println!("{}", kiran);
    
    println!("------------------------------------------------------");

    let mut data = String::from("Hello, world!");

    // Mutable borrow of `data`
    let borrowed_data = &mut data;

    borrowed_data.push_str(" Updated!");

    println!("Borrowed data: {}", borrowed_data);
    


    
}



