// Define a trait named `Describable`
trait Describable {
    // Define a method signature that types implementing this trait must provide
    fn describe(&self) -> String;
}

// Implement the `Describable` trait for a struct `Person`
struct Person {
    name: String,
    age: u8,
    maried: bool,
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old {}.", self.name, self.age, if self.maried{" he is maried"} else{" he is not married"})
    }
}

// Implement the `Describable` trait for another struct `Book`
struct Book {
    title: String,
    author: String,
    cost: bool,
}

impl Describable for Book {
    fn describe(&self) -> String {
        let is_cur = self.cost;

        // if is_cur{"not a expensive"} else {"expensive"}

        format!("'{}' by {} {}", self.title, self.author, if is_cur{"not a expensive"} else {"expensive"})
    }
}

fn main() {
    let person = Person {
        name: String::from("Magaraja"),
        age: 30,
        maried: false,

    };

    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        cost: true,
    };

    // Call the describe method for both person and book
    println!("{}", person.describe());
    println!("{}", book.describe());

    let numbers = [1, 2, 3, 4, 5];

match numbers {
    [first, .., last] => println!("First: {}, Last: {}", first, last),
        //  println!("Not enough elements"),
}

 
 
}
