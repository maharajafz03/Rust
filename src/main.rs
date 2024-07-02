
// fn hello() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }



// fn main() {
//     let x = 5;

//     let  x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

// fn main(){
//     let greeting: &str = "hello world";
//     println!("{}",greeting);
// }


// let mut greeting: &str = "hello world";

//  greeting.push(str: "mother fucker");

//   println!("{}", greeting);


// fn main(){
//     let x: i32 = 42;
//     let y: u64 = 100;
//     println!("Signed Integer: {}", x);
//     println!("Unsigned Integer: {}", y);
// }

fn main(){
    // char in rust::::::
    let x: char = 'A';
    println!("x value is {}", x);


    // bool data-types::::::

    let is_sleeping: bool = true;
    println!("yamuna is sleeping?{}", is_sleeping);

    // compond data-types:::::[array]

    let number: [i32; 5] = [1,2,3,4,5];
    println!("this is a array {:?}", number[4]);

    //tuples:::::

    let human: (&str, i32, bool, [i32; 6]) = ("allen", 30, true, [11,2,3,4,5,908]);
    println!("{:?}", human);
}