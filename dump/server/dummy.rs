use core::num;

fn main() {
    let num = String::from("Hello");
    let sum = num;  // num is moved to sum
    
    println!("{}", sum); // prints "Hello"
    println!("{}", num); // This would cause a compile-time error because num is no longer valid
}


fn losser() {
    let num
}