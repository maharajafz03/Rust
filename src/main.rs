// just learning about data_types & variables [imutable];;;

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

    //slice::::::

    let num:&[i32] = &[1,2,3,4,5];
    println!("{:?}", num);

    // ---------------->
    

    let mut  my_name = String::from("hello");
    
     my_name.push_str("mother Fucker");
     println!("{}", my_name);

     //String-------|stack & Heap|-------String

     let mut data: String = String::from("welcome da pundai");
     data.push_str("hows that fucks");
     println!("{}", data);

    //vec<T>-------------method

    //  let mut num: Vec<i32> = Vec::new(21);
    //  num.push(4);
    //  num.push(7);
    //  num.push(5);
    //  num.push(2);
    //  println!("{:?}", num);

    // Create a mutable vector of integers
    let mut numbers: Vec<i32> = Vec::new();

    // Push integer values into the vector
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    // Print the vector
    println!("{:?}", numbers); // Output: [10, 20, 30]

    let mut greeting: String = String::from("hello world");
    println!("{}", greeting);
    greeting.push_str( " mother fucker i love rust");
    println!("{}", greeting);
    let naan:String = greeting;
    println!("{}", naan);


    let mut num: String = String::from("heello");
    num.push_str("world");
    println!("{}", num);

    let mut  nan:String = num;
    nan.push_str("234");
    println!("{}", nan);


    let mut rana: String =String::from ("welcome");
    rana.push_str(" da  pundai");
    println!("{}", rana)



}