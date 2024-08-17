// Define a trait with a method
trait Describe {
    fn describe(&self) -> String;
}

trait Contract {
   fn contract(&self) -> String;
}

// Implement the trait for a struct
struct Person {
    name: String,
    age: u32,
}

struct Son {
    name: String,
    age: u32,
}

impl Describe for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}



impl Contract for Son {
    fn contract(&self) -> String {
        format!("{} is  a fucker and he was raped {} years old girl", self.name, self.age)
    }
}


fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 50,
    };


    let son = Son {
        name: String::from("Ace"),
        age: 20,
    };
    
    // Using the method defined by the trait
     println!("{}", person.describe());
    println!("{}", son.contract());

    struct Beh {
        name: String,
        age: i32,
    }

    let mut karni: Vec<Beh> = Vec::new();

    karni.push(Beh {
        name: String::from("mgaraja"),
        age: 29,
    })

    println!("{:?}", karni);
}
