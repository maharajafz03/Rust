// <<-------owner , borrowing & refference------->>

fn main () { 
    
let my_place = String::from("rust");
let res = calculate(&my_place);
println!("this is {}, and value {}", my_place, res);


let name = String::from("hello") ;
let func = dep(&name);
println!("{}", func);

fn dep(s: &String) -> usize{
    s.len()
}

fn calculate(s: &String) -> usize {
    s.len()
 }


 let a1 = String::from("web3");
 let a2 = a1;
 println!("{}", a2);


 let a = String::from("solana");
 let mut b = a.clone();
 let con = b.clone() + "king";

 b.push_str("master");

 println!("{}", con);
 println!("i love {}, build ", a);
 println!("i love {}, build ", b);


 let mut num: i32 = 5;
 let x = &mut num;
  
  *x += 4;

  println!("{}", num);
//   println!("{}", x);
}

