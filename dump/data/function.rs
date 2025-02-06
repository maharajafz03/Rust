use std::string;


fn main() {

    let mut name = String::from("hello welcome ");
    dance(&mut name);
    println!("{}", name);

    const  U: i32 = 78;
    println!("{}", U);

    println!("{}", A); 

    let x = 67;
    let x = x +1;
    let x = x +1;
    let x = x +1;
    let x = x +1;
    let x = x +1;
    let x = x +1;
    let x = x +1;
    let x = x +1;

    println!("{}", x)
    
}

fn dance(name: &mut String) {
    name.push_str(" da ");

}

const A: i32 = 78;
