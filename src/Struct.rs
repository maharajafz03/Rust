

fn main(){

//     let mut account = bank{
//          owner: "maga".to_string(),
//          balance: 432,
//     };



// }

// struct bank {
//     owner: String,
//     balance: i32,
// }

// impl bank {
//     fn withdraw(&mut Self, amount: i32) {
//         println!("withdraw {} from account owned by {}", amount, self.owner);
//         self.balance -= amount;
//     }

//     fn check_balance(&self) {
//         println!("account owned by {}, blance {}", self.owner, self.balance);
//     }
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10763 {
        break counter * 2;   //Solana rpc stuck..!!
    }   
};

println!("the result is {result}")

}