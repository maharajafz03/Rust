fn main() {
 println!("{}", is_number(2));
}
fn is_number(num: i32) -> bool {
    
    if num % 2 == 0 {
        return true;
    }else {
        return false;
    }
}