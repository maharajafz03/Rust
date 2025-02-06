

// signed int = (+ & -)
// unsigned int = (+)



fn main() {
    let number: i32 = 89;
    let num1: i32 = -89;
    //note this integer have a both positive and negative value

    let unsigned: u32 = 67;
    // unnsigned hold obly positive value 

    //let name: String = f"hello";

<<<<<<< HEAD
    let pi: f32 = 3.45;
   // float value

    println!("{}",number);
    println!("{}",num1);
    println!("{}",unsigned);
    println!("{}", pi);

    let male: bool = true || false;
    println!("{}", male);

    let array: [i32; 4] = [1,2,3,4];
     // array should be look like this 
    println!("{:?}", array);

    let bikes: [&str; 2] = ["s1000rr", "ducati"];
    println!("{:?}", bikes);
    println!("{}", bikes[1]);

    let human   = ("maga".to_string(), 20, true);
    println!("{:?}", human);

    //slices

    let  maga :&[&str] = &["raja", "yamu", "nna"];
    println!("{:?}", maga);

    let manage: &[i32] = &[1,3,4,5];
    println!("{:?}", manage);

    let animal: &[&String] = &[&"maga".to_string() , &"raja".to_string()];
    println!("{:?}", animal)

=======
    println!("{}",number);
    println!("{}",num1);
    println!("{}",unsigned);
    
>>>>>>> 24f4bbbc369b353b44f5da73ab14cf59c6e37be0
}