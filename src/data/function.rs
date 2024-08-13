
fn main() {

    let mut name = String::from("hello welcome ");
    dance(&mut name);
    println!("{}", name);
    
}

fn dance(name: &mut String) {
    name.push_str(" da ");

}