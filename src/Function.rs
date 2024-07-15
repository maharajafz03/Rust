// just lerning about  function & returnStament,,and so on;;;;;;

fn main() {
    let num: i8 = 21;
    println!("{}", num);
    king_dance(54 );
    data_base("maga", "billioner", 25, "dubai");

    let _x  = {
        let price = 5;
        let quantity =20;
        price * quantity
    };
    println!("result {}",_x);

    let _y = con_tor(3, 27);
    println!("return value {}", _y);

    let _a = cal_c(322.4, 78.53);
    println!("this is your data {}", _a);
}

fn king_dance(count: i32, ){
    println!("my_bank_account count is {} only usd", count);
}

fn data_base(name: &str, status: &str, age: u32, place: &str){
    println!("im {}, {}, age is {}, nationality {}", name, status, age, place);
}

fn con_tor(a: i32, b: i32) -> i32{
    a + b
}


fn cal_c(weight: f32, height: f32) -> f32{
    weight / height
}


