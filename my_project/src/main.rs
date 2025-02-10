// struct Dash {
//     name: String,
//     age: u32
// }

// impl Dash {
//      fn display(&self) {
//         println!("{}", self.name);
//         println!("{}", self.age);
//      }

//      fn modified(& mut self, name: &str, age: u32) {
//         self.name = name.to_string();
//         self.age = age;
//      }
// }

// fn main() {
//     let mut user = Dash{
//         name: String::from("magaraja"),
//         age: 25
//     };
//     user.display();
//     user.modified("yamuna", 23);
//     user.display();
//     user.modified("sino", 23);
//     user.display();
// }

fn main() {
    let x = Some(5);

    let a = (67);
match x {
    Some(val) => println!("Found: {}", val),
    None => println!("No value found"),
}

}