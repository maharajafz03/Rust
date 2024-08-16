fn main() {

    let name = String::from("W E L C O M E ");

     {    
        let name = name + " B ";
        let name = name + " A ";
        let name = name + " C ";
        let name = name + " K ";
        println!("{}", name);
     }

    let manage = "90oidhfgw shgsycqadsdd shhsgc866";
    let current = manage.len();
    println!("{}", current);


    let is_dog = false;

    if is_dog {
        println!("dog is here");

    } else  {
        println!("dog is having a dinner");
    }
    // else {
    //     println!("dog is no longer");
    // } 
    

    // let Result = loop {

    //     let n = 5;
        
    //     if n == 109863 {
    //        println!("ok")                         
    //     }else {
    //         n * 2;
    //     }

    //     println!("{Result}")
    // };
   
    
        let numbers = [10, 20, 30, 40];
    
        for num in numbers {
            println!("num: {}", num);
        }
    
    
        let mut num = vec![67,78];

        // Push a value into the vector
        num.push(80);
        num.push(50);
        num.push(30);

        num.push(110);
        num.push(40);
    
        println!("{:?}", num);


        let array = [1,2,3,4];
        println!("{:?}", array);
        

      let manage =String::from("hello world");
      println!("{}", manage);   

      let  mairandi =" hello ";
      let mut  magaraja =mairandi.to_string();
      magaraja.push_str("di");
      println!("{}", magaraja);
}