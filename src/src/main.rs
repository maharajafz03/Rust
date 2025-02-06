
struct User{
    name: String,
    age: u8,

}

fn main() {
   let user = User{
    name: String::from("mother_fuckerz"),
    age: 28,
   };

   println!("{}", user.name);
}